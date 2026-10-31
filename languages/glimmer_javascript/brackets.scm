; Based on Zed's JavaScript queries (zed-industries/zed@08cbb88, crates/grammars/src/javascript/brackets.scm),
; without the TypeScript and JSX patterns, which this grammar doesn't have.
; Glimmer additions are at the end of the file.

("(" @open
  ")" @close)

("[" @open
  "]" @close)

("{" @open
  "}" @close)

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

(("`" @open
  "`" @close)
  (#set! rainbow.exclude))

; Glimmer `<template>` tags
(("</" @open
  ">" @close)
  (#set! rainbow.exclude))

((glimmer_template
  (glimmer_opening_tag) @open
  (glimmer_closing_tag) @close)
  (#set! newline.only)
  (#set! rainbow.exclude))
