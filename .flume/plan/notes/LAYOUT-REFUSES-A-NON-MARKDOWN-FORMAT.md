# Layout refusal rides the Vocabulary variant; no dedicated error

`kind::markdown_layout_format` refuses a layout over `json-document` / `toml-document`
in both lifts: `content_from_row` (a declared kind) and `CustomKind::overlay_content`
(a relocated built-in's layout overlay), since the overlay is the same lowering.

The refusal is a `LockRowError::Vocabulary` whose `column` string embeds the kind name:
"column `format` of layout kind `intent` value `json-document` is outside the closed
vocabulary". `LockRowError` lives in `src/drift.rs`, outside this tick's fence, so no
dedicated variant (e.g. `LayoutFormat { kind, format }`) could be added. A future tick
touching drift.rs could give it one; the message wording is then the only thing to move.

SDK type-level refusal (optional per entry) was not done.
