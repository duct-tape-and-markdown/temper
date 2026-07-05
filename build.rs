//! Build script.
//!
//! The built-in std-lib (packages and kinds) is authored directly as Rust data
//! (`src/builtin.rs`, `src/builtin_kind.rs`) — the compiled default program the
//! engine carries for SDK-less checking (`specs/architecture/15-kinds.md`, "Decision:
//! field typing lives in the SDK — there is no kind file format"). There is no more
//! build-time `PACKAGE.md`/`KIND.md` tree to walk and embed, so this script does
//! nothing; it stays only because Cargo auto-detects `build.rs` at the crate root.

fn main() {}
