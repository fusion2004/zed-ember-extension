(element_node_start
  ">" @end) @indent

(element_node_void
  "/>" @end) @indent

(element_node
  (element_node_start) @start
  (element_node_end)? @end) @indent

(block_statement
  (block_statement_start) @start
  (block_statement_end)? @end) @indent

(sub_expression
  ")" @end) @indent

; `{{else}}` and `{{else if ...}}` line up with their opening block, and the
; content after them is indented again
(block_statement
  (mustache_statement
    [
      (identifier) @_else
      (helper_invocation
        helper: (identifier) @_else)
    ]
    (#eq? @_else "else")) @outdent)

(block_statement
  (mustache_statement
    [
      (identifier) @_else
      (helper_invocation
        helper: (identifier) @_else)
    ]
    (#eq? @_else "else")) @start
  (block_statement_end)? @end) @indent
