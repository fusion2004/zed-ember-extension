(element_node
  (element_node_start
    (tag_name) @name)) @item

(element_node
  (element_node_void
    (tag_name) @name)) @item

(block_statement
  (block_statement_start
    [
      "{{#"
      "{{~#"
    ] @context
    path: (identifier) @name)) @item
