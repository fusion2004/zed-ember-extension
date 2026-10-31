("(" @open
  ")" @close)

(mustache_statement
  [
    "{{"
    "{{~"
  ] @open
  [
    "}}"
    "~}}"
  ] @close)

(block_statement_start
  [
    "{{#"
    "{{~#"
  ] @open
  [
    "}}"
    "~}}"
  ] @close)

(block_statement_end
  [
    "{{/"
    "{{~/"
  ] @open
  [
    "}}"
    "~}}"
  ] @close)

(block_params
  "|" @open
  "|" @close)

(("<" @open
  "/>" @close)
  (#set! rainbow.exclude))

(("<" @open
  ">" @close)
  (#set! rainbow.exclude))

(("</" @open
  ">" @close)
  (#set! rainbow.exclude))

(("\"" @open
  "\"" @close)
  (#set! rainbow.exclude))

(("'" @open
  "'" @close)
  (#set! rainbow.exclude))

; Pressing enter between `<div>|</div>` or `{{#if}}|{{/if}}` puts the cursor on
; its own, indented line.
((element_node
  (element_node_start) @open
  (element_node_end) @close)
  (#set! newline.only)
  (#set! rainbow.exclude))

((block_statement
  (block_statement_start) @open
  (block_statement_end) @close)
  (#set! newline.only)
  (#set! rainbow.exclude))
