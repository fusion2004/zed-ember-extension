; Based on Zed's TypeScript queries (zed-industries/zed@08cbb88, crates/grammars/src/typescript/indents.scm).
; Glimmer additions are at the end of the file.

[
  (call_expression)
  (assignment_expression)
  (member_expression)
  (lexical_declaration)
  (variable_declaration)
  (assignment_expression)
] @indent

; Indent a braceless body (`if (x)\n  y()`). When the body is a `{}` block the
; `@end` stops the range before the brace so the block rule handles it instead,
; which keeps Allman-style braces unindented.
(if_statement
  consequence: (statement_block)? @end) @indent

(else_clause
  (statement_block)? @end) @indent

(for_statement
  body: (statement_block)? @end) @indent

(for_in_statement
  body: (statement_block)? @end) @indent

(while_statement
  body: (statement_block)? @end) @indent

(_
  "["
  "]" @end) @indent

(_
  "<"
  ">" @end) @indent

(_
  "{"
  "}" @end) @indent

(_
  "("
  ")" @end) @indent

; Glimmer `<template>` tags
(glimmer_template
  (glimmer_opening_tag) @start
  (glimmer_closing_tag)? @end) @indent
