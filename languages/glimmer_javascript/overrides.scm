; Based on Zed's JavaScript queries (zed-industries/zed@08cbb88, crates/grammars/src/javascript/overrides.scm),
; without the JSX patterns, which this grammar doesn't have.

(comment) @comment.inclusive

(string) @string

(template_string
  (string_fragment) @string)
