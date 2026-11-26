# Import path reads the documented delimiter

Shipped. The unbacked-import message lives in `src/graph.rs` (`unbacked_pointer`);
`src/gate.rs` only re-severities it to Warn, so no gate.rs change. The one
`@path` reader is `extract::line_at_imports`, shared by `kind.rs`'s directive
primitive, so the escape reaches every kind composing `AtImport`. Only a
backslash before a space is an escape; any other backslash stays literal
(the docs name only the space case).
