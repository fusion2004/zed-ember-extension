; Based on Zed's TypeScript queries (zed-industries/zed@08cbb88, crates/grammars/src/typescript/brackets.scm).
; Glimmer additions are at the end of the file.

("(" @open
  ")" @close)

("[" @open
  "]" @close)

("{" @open
  "}" @close)

("<" @open
  ">" @close)

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
