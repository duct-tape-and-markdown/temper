//! The `--frozen` lane for temper's own std-lib:
//! re-derive the built-in lock from `@dtmd/temper/claude-code` — a real `node`
//! subprocess running the built SDK, exactly as `tests/emit.rs` drives the seam — and
//! byte-compare its declaration rows against the embedded `src/builtin_lock.toml`.
//!
//! A memberless harness binds every built-in kind to every built-in default contract via
//! `expect` (`sdk/src/assembly.ts`: `expect` keys clauses to a kind value with no
//! member needed), the identical construction `src/builtin_lock.toml`'s own header
//! says produced it. Agreement is mechanical: this test is the CI job the fail-loud
//! invariant describes, never a human re-reading two files side by side.

use std::fs;
use std::path::PathBuf;

use temper::drift::{self, EmitOptions};

mod common;

/// A memberless harness binding every built-in kind to every built-in default contract via
/// `expect` — no members, so `compileDeclarations` (`sdk/src/declarations.ts`) emits
/// only the five kind facts and their default contract clauses, exactly the family
/// `src/builtin_lock.toml` embeds (`sdk/src/builtins.ts`, the built-in kinds/default contracts).
const MEMBERLESS_BUILTIN_PROGRAM: &str = r#"
import { emit, harness } from "@dtmd/temper";
import {
  agent,
  agentDefaultContract,
  command,
  commandDefaultContract,
  hook,
  hookDefaultContract,
  mcpServer,
  mcpServerDefaultContract,
  memory,
  memoryAnthropicDefaultContract,
  rule,
  ruleDefaultContract,
  skill,
  skillDefaultContract,
} from "@dtmd/temper/claude-code";

const program = harness({
  members: [],
  expect: [
    { kind: agent, clauses: agentDefaultContract },
    { kind: command, clauses: commandDefaultContract },
    { kind: hook, clauses: hookDefaultContract },
    { kind: mcpServer, clauses: mcpServerDefaultContract },
    { kind: memory, clauses: memoryAnthropicDefaultContract },
    { kind: rule, clauses: ruleDefaultContract },
    { kind: skill, clauses: skillDefaultContract },
  ],
});

process.stdout.write(emit(program).seam);
"#;

/// Wire the memberless fixture under `<harness>/.temper/harness.ts`, with a
/// `node_modules/@dtmd/temper` resolving to the repo's own built SDK — the stand-in
/// for a real consumer's installed dependency (`tests/emit.rs`'s `wire_sdk_harness`).
fn wire_memberless_harness() -> (PathBuf, PathBuf) {
    let harness = common::tmpdir("memberless");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    fs::write(into.join("harness.ts"), MEMBERLESS_BUILTIN_PROGRAM).unwrap();

    let node_modules_scope = into.join("node_modules").join("@dtmd");
    common::vendor_sdk(&node_modules_scope);

    (harness, into)
}

/// The embedded lock's declaration rows, with its hand-authored provenance header
/// comment stripped: that header explains the row family's provenance, but is not
/// itself part of `drift::emit`'s row output, so it plays no part in the byte-compare.
fn embedded_declaration_rows() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/builtin_lock.toml");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("the embedded built-in lock must exist at {path:?}: {err}"));
    let start = text
        .find("[[declaration.kind]]")
        .expect("the embedded lock carries declaration rows");
    text[start..].to_string()
}

#[test]
fn the_embedded_builtin_lock_byte_equals_the_sdk_modules_own_memberless_emit() {
    let (_harness, into) = wire_memberless_harness();

    drift::emit_program(&into, EmitOptions::default()).expect(
        "re-deriving the built-in lock requires a working node + the built \
         @dtmd/temper/claude-code module — the --frozen discipline fails loud here \
         rather than silently skipping the comparison",
    );

    let derived = fs::read_to_string(into.join("lock.toml"))
        .expect("emit_program writes a lock.toml carrying the memberless declaration rows");
    let embedded = embedded_declaration_rows();

    assert_eq!(
        derived, embedded,
        "src/builtin_lock.toml has drifted from @dtmd/temper/claude-code's own memberless \
         emit — regenerate it by re-running that emit and re-embedding the resulting rows \
         verbatim (src/builtin_lock.toml's own header), never by hand-editing a row"
    );
}
