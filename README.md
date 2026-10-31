# Ember & Glimmer for Zed

[Ember.js](https://emberjs.com) support for the [Zed](https://zed.dev) editor.

| Language     | Files                  | Tree-sitter grammar                                                                                    |
| ------------ | ---------------------- | ------------------------------------------------------------------------------------------------------ |
| `Handlebars` | `.hbs`, `.handlebars`  | [tree-sitter-glimmer](https://github.com/ember-tooling/tree-sitter-glimmer)                            |
| `Glimmer JS` | `.gjs`                 | [tree-sitter-glimmer-javascript](https://github.com/ember-tooling/tree-sitter-glimmer-javascript)     |
| `Glimmer TS` | `.gts`                 | [tree-sitter-glimmer-typescript](https://github.com/ember-tooling/tree-sitter-glimmer-typescript)     |

- Syntax highlighting, bracket matching, auto-indentation, and outlines for all three languages.
- `<template>` tags in `.gjs`/`.gts` files and `hbs` tagged templates in tests are highlighted as Handlebars.
- Type-aware completions, diagnostics, hovers, and go-to-definition in `.gjs`/`.gts` files, including inside `<template>` tags, from [Glint](https://github.com/typed-ember/glint).
- Ember-aware completions and navigation (components, helpers, modifiers, routes, services) and template linting from the [Ember Language Server](https://github.com/ember-tooling/ember-language-server).

## Language servers

### Glint

Glint v2 is a [TypeScript server plugin](https://github.com/typed-ember/glint/tree/main/packages/tsserver-plugin).
Like the Vue extension, this extension adds the plugin to Zed's built-in `vtsls` and `typescript-language-server`, so that `.ts` and `.js` files can import from `.gts` and `.gjs` files.

That alone doesn't cover `.gts` and `.gjs` buffers, because Zed only runs its built-in vtsls on TypeScript and JavaScript files, and extensions can't change that default.
So this extension also runs its own [vtsls](https://github.com/yioneko/vtsls) instance with the same plugin, named `glint`, for `Glimmer JS` and `Glimmer TS` files.

Your project needs Glint v2 installed:

```sh
npm install --save-dev @glint/ember-tsc @glint/template
```

The plugin loads `@glint/ember-tsc` from your project, so the version you install is the one the editor uses.
See [Migrating to Glint v2](https://github.com/typed-ember/glint/blob/main/docs/v2-upgrade.md) if you're coming from Glint v1.

Glint needs TypeScript 5 or 6. If your project uses TypeScript 7 or newer, which no longer supports tsserver plugins, the Glint server falls back to the TypeScript version bundled with vtsls.

Settings under `lsp.glint.settings` are passed to vtsls and merged over this extension's defaults, which match Zed's built-in vtsls:

```json
{
  "lsp": {
    "glint": {
      "settings": {
        "typescript": {
          "tsserver": { "maxTsServerMemory": 16384 }
        }
      }
    }
  }
}
```

#### Using a single TypeScript server

The `glint` server is a second tsserver alongside the one Zed runs for `.ts` and `.js` files.
Since the built-in vtsls already has the Glint plugin, you can run it on Glimmer files instead, so one tsserver serves the whole project.
This uses less memory on large apps:

```json
{
  "languages": {
    "Glimmer TS": {
      "language_servers": ["vtsls", "ember-language-server", "!glint", "..."]
    },
    "Glimmer JS": {
      "language_servers": ["vtsls", "ember-language-server", "!glint", "..."]
    }
  }
}
```

The Glint plugin is installed the first time the Glint server starts, so open a `.gts` or `.gjs` file with the default settings once before switching.

### Ember Language Server

The [Ember Language Server](https://github.com/ember-tooling/ember-language-server) runs for `Handlebars`, `Glimmer JS`, and `Glimmer TS` files.
To also use it in `.js` and `.ts` files (for example, to jump from a route to its template), add it to those languages:

```json
{
  "languages": {
    "JavaScript": {
      "language_servers": ["ember-language-server", "..."]
    },
    "TypeScript": {
      "language_servers": ["ember-language-server", "..."]
    }
  }
}
```

Settings under `lsp.ember-language-server.settings` are passed to the server:

```json
{
  "lsp": {
    "ember-language-server": {
      "settings": {
        "useBuiltinLinting": true,
        "collectTemplateTokens": true,
        "ignoredProjects": [],
        "addons": []
      }
    }
  }
}
```

### Using your own binaries

Both servers are installed from npm by default.
To use another installation, set `lsp.glint.binary` or `lsp.ember-language-server.binary`:

```json
{
  "lsp": {
    "ember-language-server": {
      "binary": {
        "path": "/path/to/ember-language-server",
        "arguments": ["--stdio"]
      }
    }
  }
}
```

## Formatting

`.hbs` files are formatted with Prettier's built-in `glimmer` parser.
`.gjs` and `.gts` files use [prettier-plugin-ember-template-tag](https://github.com/ember-tooling/prettier-plugin-ember-template-tag).
If your project has Prettier and the plugin configured, Zed uses them.
Otherwise, tell Zed's bundled Prettier to install the plugin:

```json
{
  "languages": {
    "Glimmer TS": {
      "prettier": { "plugins": ["prettier-plugin-ember-template-tag"] }
    },
    "Glimmer JS": {
      "prettier": { "plugins": ["prettier-plugin-ember-template-tag"] }
    }
  }
}
```

## Limitations

- Glint v2 dropped type checking for `.hbs` templates, so `Handlebars` files get syntax support and the Ember Language Server, but not Glint.
- Glint v2 also ships a small Volar-based language server with a few extras, such as component signature hovers.
  It isn't used here: it relies on the editor forwarding `tsserver/request` messages to the TypeScript server, which Zed only does for the Vue language server.
  The plugin covers type checking, completions, and navigation on its own.
  This is also why `.gts` and `.gjs` buffers need a vtsls of their own: in the Vue extension, the Vue language server handles `.vue` buffers and reaches tsserver through that forwarding, so vtsls never has to open them.

## Development

1. Install Rust with [rustup](https://rustup.rs).
2. In Zed, run `zed: install dev extension` and pick this directory.
3. Run `zed: open log` to see errors from the extension and its language servers.
   Start Zed with `zed --foreground` to also see the extension's `println!` output.

The queries for `Glimmer JS` and `Glimmer TS` are based on Zed's own JavaScript and TypeScript queries.
Each file notes the Zed commit it was copied from, and Glimmer-specific additions are at the end.
