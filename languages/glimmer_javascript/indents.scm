; Based on Zed's JavaScript queries (zed-industries/zed@08cbb88, crates/grammars/src/javascript/indents.scm),
; without the TypeScript and JSX patterns, which this grammar doesn't have.
; Glimmer additions are at the end of the file.

[
  (call_expression)
  (assignment_expression)
  (member_expression)
  (lexical_declaration)
  (variable_declaration)
  (assignment_expression)
  (if_statement)
  (for_statement)
] @indent

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
