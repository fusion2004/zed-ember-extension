; Based on Zed's TypeScript queries (zed-industries/zed@08cbb88, crates/grammars/src/typescript/overrides.scm).

(comment) @comment.inclusive

(string) @string

(template_string
  (string_fragment) @string)

(_
  value: (call_expression
    function: (identifier) @function_name_before_type_arguments
    type_arguments: (type_arguments)))
