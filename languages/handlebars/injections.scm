((comment_statement) @injection.content
  (#set! injection.language "comment"))

((style_element
  (raw_text) @injection.content)
  (#set! injection.language "css"))

((script_element
  (raw_text) @injection.content)
  (#set! injection.language "javascript"))
