; Literals

(string_literal) @string

(concat_statement) @string

(number_literal) @number

(boolean_literal) @boolean

(comment_statement) @comment

; Elements

(tag_name) @tag

; Components start with a capital letter, e.g. `<Foo::Bar>` or `<Foo.bar>`
((tag_name) @type @tag.component
  (#match? @type "^[A-Z]"))

; Components passed in as arguments, e.g. `<@component>`
((tag_name) @variable.parameter @tag.component
  (#match? @variable.parameter "^@"))

; Named blocks, e.g. `<:header>`
((tag_name) @label
  (#match? @label "^:"))

(attribute_name) @attribute

; `...attributes`
((attribute_name) @keyword
  (#eq? @keyword "...attributes"))

; Mustache expressions

(identifier) @variable

(path_expression
  (identifier) @property)

(path_expression
  .
  (identifier) @variable)

; `@arguments`
((identifier) @variable.parameter
  (#match? @variable.parameter "^@"))

((identifier) @variable.special
  (#eq? @variable.special "this"))

(hash_pair
  key: (identifier) @property)

(block_params
  (identifier) @variable.parameter)

(helper_invocation
  helper: [
    (identifier) @function
    (path_expression
      (identifier) @function .)
  ])

; Built-in keywords

((mustache_statement
  [
    (identifier) @keyword
    (helper_invocation
      helper: (identifier) @keyword)
  ])
  (#any-of? @keyword "yield" "outlet" "debugger" "else" "has-block" "has-block-params"))

((sub_expression
  [
    (identifier) @keyword
    (helper_invocation
      helper: (identifier) @keyword)
  ])
  (#any-of? @keyword "has-block" "has-block-params"))

; `{{else if ...}}` / `{{else unless ...}}`
(mustache_statement
  (helper_invocation
    helper: (identifier) @_else
    .
    argument: (identifier) @keyword)
  (#eq? @_else "else")
  (#any-of? @keyword "if" "unless" "each" "let" "with"))

(block_statement_start
  path: (identifier) @keyword)

(block_statement_end
  path: (identifier) @keyword)

; Classic components invoked as blocks, e.g. `{{#my-component}}`
((block_statement_start
  path: (identifier) @type @tag.component)
  (#not-any-of? @type
    "if" "unless" "each" "each-in" "let" "with" "in-element" "-in-element"))

((block_statement_end
  path: (identifier) @type @tag.component)
  (#not-any-of? @type
    "if" "unless" "each" "each-in" "let" "with" "in-element" "-in-element"))

(block_params
  "as" @keyword)

; Punctuation

[
  "{{"
  "}}"
  "{{~"
  "~}}"
  "{{#"
  "{{~#"
  "{{/"
  "{{~/"
  "("
  ")"
] @punctuation.bracket

[
  "<"
  ">"
  "</"
  "/>"
] @punctuation.bracket

"." @punctuation.delimiter

(block_params
  "|" @punctuation.bracket)

[
  "="
] @operator
