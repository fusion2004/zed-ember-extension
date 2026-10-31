use std::{collections::HashSet, env, fs};

use zed_extension_api::{
    self as zed,
    lsp::{Completion, CompletionKind},
    serde_json::{self, json, Value},
    settings::LspSettings,
    CodeLabel, CodeLabelSpan, LanguageServerId, Result, Worktree,
};

const GLINT_SERVER_ID: &str = "glint";
const ELS_SERVER_ID: &str = "ember-language-server";

const VTSLS_PACKAGE: &str = "@vtsls/language-server";
const VTSLS_PATH: &str = "node_modules/@vtsls/language-server/bin/vtsls.js";

const GLINT_PLUGIN_PACKAGE: &str = "@glint/tsserver-plugin";
const GLINT_PLUGIN_PATH: &str = "node_modules/@glint/tsserver-plugin/package.json";

const ELS_PACKAGE: &str = "@ember-tooling/ember-language-server";
const ELS_PATH: &str =
    "node_modules/@ember-tooling/ember-language-server/bin/ember-language-server.js";

/// Language ids the Glint tsserver plugin should handle. The first two are the
/// ids this extension sends to its own Glint server; the others are what Zed
/// derives from the language names when a user runs the built-in vtsls on
/// Glimmer files instead.
const GLINT_PLUGIN_LANGUAGES: [&str; 4] = ["glimmer-ts", "glimmer-js", "glimmer ts", "glimmer js"];

struct EmberExtension {
    /// Packages already installed or updated during this session.
    checked_packages: HashSet<&'static str>,
}

fn extension_dir() -> String {
    env::current_dir().unwrap().to_string_lossy().to_string()
}

fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok_and(|stat| stat.is_file())
}

impl EmberExtension {
    /// Makes sure the latest version of an npm package is installed in the
    /// extension directory, checking for updates once per session. If the
    /// registry can't be reached, an existing installation is used as is.
    fn ensure_package(
        &mut self,
        language_server_id: &LanguageServerId,
        package: &'static str,
        expected_path: &str,
    ) -> Result<()> {
        if self.checked_packages.contains(package) && file_exists(expected_path) {
            return Ok(());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let latest_version = match zed::npm_package_latest_version(package) {
            Ok(version) => version,
            Err(error) => {
                if file_exists(expected_path) {
                    println!(
                        "could not check for {package} updates, using installed version: {error}"
                    );
                    self.checked_packages.insert(package);
                    return Ok(());
                }
                return Err(error);
            }
        };

        if !file_exists(expected_path)
            || zed::npm_package_installed_version(package)?.as_deref()
                != Some(latest_version.as_str())
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            if let Err(error) = zed::npm_install_package(package, &latest_version) {
                if !file_exists(expected_path) {
                    return Err(error);
                }
                println!("failed to update {package}, using installed version: {error}");
            }
            if !file_exists(expected_path) {
                return Err(format!(
                    "installed package '{package}' did not contain expected path '{expected_path}'"
                ));
            }
        }

        self.checked_packages.insert(package);
        Ok(())
    }

    fn glint_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        // The plugin is loaded by tsserver from this extension's directory, so it
        // is needed even when the user supplies their own vtsls binary.
        self.ensure_package(language_server_id, GLINT_PLUGIN_PACKAGE, GLINT_PLUGIN_PATH)?;

        if let Some(command) = user_binary(GLINT_SERVER_ID, worktree, &["--stdio"]) {
            return Ok(command);
        }

        self.ensure_package(language_server_id, VTSLS_PACKAGE, VTSLS_PATH)?;
        node_script_command(VTSLS_PATH)
    }

    fn els_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        if let Some(command) = user_binary(ELS_SERVER_ID, worktree, &["--stdio"]) {
            return Ok(command);
        }

        self.ensure_package(language_server_id, ELS_PACKAGE, ELS_PATH)?;
        node_script_command(ELS_PATH)
    }
}

/// Returns the command for a user-configured `lsp.<server>.binary.path`, if any.
fn user_binary(
    server_id: &str,
    worktree: &Worktree,
    default_args: &[&str],
) -> Option<zed::Command> {
    let binary = LspSettings::for_worktree(server_id, worktree)
        .ok()?
        .binary?;
    let path = binary.path?;
    Some(zed::Command {
        command: path,
        args: binary
            .arguments
            .unwrap_or_else(|| default_args.iter().map(|arg| arg.to_string()).collect()),
        env: binary.env.unwrap_or_default().into_iter().collect(),
    })
}

fn node_script_command(script_path: &str) -> Result<zed::Command> {
    Ok(zed::Command {
        command: zed::node_binary_path()?,
        args: vec![
            env::current_dir()
                .unwrap()
                .join(script_path)
                .to_string_lossy()
                .to_string(),
            "--stdio".to_string(),
        ],
        env: Default::default(),
    })
}

fn glint_plugin_entry() -> Value {
    json!({
        "name": GLINT_PLUGIN_PACKAGE,
        "location": extension_dir(),
        "enableForWorkspaceTypeScriptVersions": true,
        "languages": GLINT_PLUGIN_LANGUAGES,
        "configNamespace": "typescript",
    })
}

/// Finds the project's TypeScript SDK, relative to the worktree root.
///
/// vtsls needs `tsserver.js`, which TypeScript 7+ no longer ships, so newer
/// versions are skipped in favor of the TypeScript bundled with vtsls.
fn workspace_tsdk(worktree: &Worktree) -> Option<String> {
    [".yarn/sdks/typescript", "node_modules/typescript"]
        .into_iter()
        .find(|dir| {
            worktree
                .read_text_file(&format!("{dir}/package.json"))
                .ok()
                .and_then(|text| serde_json::from_str::<Value>(&text).ok())
                .and_then(|package| package.get("version")?.as_str().map(str::to_owned))
                .and_then(|version| version.split('.').next()?.parse::<u32>().ok())
                .is_some_and(|major| major < 7)
        })
        .map(|dir| format!("{dir}/lib"))
}

/// Mirrors the defaults Zed uses for its built-in vtsls, so Glimmer files
/// behave like TypeScript and JavaScript files.
fn glint_workspace_configuration(worktree: &Worktree) -> Value {
    let mut typescript = json!({
        "suggest": {
            "completeFunctionCalls": true
        },
        "inlayHints": {
            "parameterNames": {
                "enabled": "all",
                "suppressWhenArgumentMatchesName": false
            },
            "parameterTypes": {
                "enabled": true
            },
            "variableTypes": {
                "enabled": true,
                "suppressWhenTypeMatchesName": false
            },
            "propertyDeclarationTypes": {
                "enabled": true
            },
            "functionLikeReturnTypes": {
                "enabled": true
            },
            "enumMemberValues": {
                "enabled": true
            }
        },
        "tsserver": {
            "maxTsServerMemory": 8192
        }
    });
    if let Some(tsdk) = workspace_tsdk(worktree) {
        typescript["tsdk"] = Value::String(tsdk);
    }

    let mut configuration = json!({
        "typescript": typescript,
        "javascript": typescript,
        "vtsls": {
            "autoUseWorkspaceTsdk": true,
            "experimental": {
                "completion": {
                    "enableServerSideFuzzyMatch": true,
                    "entriesLimit": 5000
                }
            },
            "tsserver": {
                "globalPlugins": [glint_plugin_entry()]
            }
        }
    });

    if let Some(user_settings) = LspSettings::for_worktree(GLINT_SERVER_ID, worktree)
        .ok()
        .and_then(|settings| settings.settings)
    {
        merge_json(&mut configuration, user_settings);
    }

    configuration
}

/// Recursively merges `source` into `target`, with `source` winning.
fn merge_json(target: &mut Value, source: Value) {
    match (target, source) {
        (Value::Object(target), Value::Object(source)) => {
            for (key, value) in source {
                match target.get_mut(&key) {
                    Some(existing) => merge_json(existing, value),
                    None => {
                        target.insert(key, value);
                    }
                }
            }
        }
        (target, source) => *target = source,
    }
}

impl zed::Extension for EmberExtension {
    fn new() -> Self {
        Self {
            checked_packages: HashSet::new(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            GLINT_SERVER_ID => self.glint_server_command(language_server_id, worktree),
            ELS_SERVER_ID => self.els_server_command(language_server_id, worktree),
            id => Err(format!("unknown language server: {id}")),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let user_options = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.initialization_options);

        Ok(match language_server_id.as_ref() {
            ELS_SERVER_ID => Some(user_options.unwrap_or_else(|| json!({ "editor": "zed" }))),
            _ => user_options,
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        Ok(match language_server_id.as_ref() {
            GLINT_SERVER_ID => Some(glint_workspace_configuration(worktree)),
            // The Ember Language Server reads its settings (`addons`,
            // `ignoredProjects`, `useBuiltinLinting`, ...) from the top level of
            // the configuration object.
            ELS_SERVER_ID => Some(
                LspSettings::for_worktree(ELS_SERVER_ID, worktree)
                    .ok()
                    .and_then(|settings| settings.settings)
                    .unwrap_or_else(|| json!({})),
            ),
            _ => None,
        })
    }

    // The two hooks below load the Glint plugin into Zed's built-in TypeScript
    // servers, so that `.ts` and `.js` files can import from `.gts` and `.gjs`
    // files. Only the Glint server answers, to avoid doing the work twice.

    fn language_server_additional_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        target_language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<Value>> {
        if language_server_id.as_ref() != GLINT_SERVER_ID || !file_exists(GLINT_PLUGIN_PATH) {
            return Ok(None);
        }

        Ok(match target_language_server_id.as_ref() {
            "typescript-language-server" => Some(json!({
                "plugins": [{
                    "name": GLINT_PLUGIN_PACKAGE,
                    "location": extension_dir(),
                    "languages": GLINT_PLUGIN_LANGUAGES,
                }],
            })),
            _ => None,
        })
    }

    fn language_server_additional_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        target_language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<Value>> {
        if language_server_id.as_ref() != GLINT_SERVER_ID || !file_exists(GLINT_PLUGIN_PATH) {
            return Ok(None);
        }

        Ok(match target_language_server_id.as_ref() {
            "vtsls" => Some(json!({
                "vtsls": {
                    "tsserver": {
                        "globalPlugins": [glint_plugin_entry()]
                    }
                }
            })),
            _ => None,
        })
    }

    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        if language_server_id.as_ref() != GLINT_SERVER_ID {
            return None;
        }

        let highlight_name = match completion.kind? {
            CompletionKind::Class | CompletionKind::Interface | CompletionKind::Enum => "type",
            CompletionKind::Constructor => "type",
            CompletionKind::Constant => "constant",
            CompletionKind::Function | CompletionKind::Method => "function",
            CompletionKind::Property | CompletionKind::Field => "property",
            CompletionKind::Variable => "variable",
            _ => return None,
        };

        let len = completion.label.len();
        let name_span = CodeLabelSpan::literal(completion.label, Some(highlight_name.to_string()));
        let detail = completion
            .label_details
            .and_then(|details| details.description)
            .or(completion.detail);

        Some(CodeLabel {
            code: Default::default(),
            spans: match detail {
                Some(detail) => vec![
                    name_span,
                    CodeLabelSpan::literal(" ", None),
                    CodeLabelSpan::literal(detail, None),
                ],
                None => vec![name_span],
            },
            filter_range: (0..len).into(),
        })
    }
}

zed::register_extension!(EmberExtension);
