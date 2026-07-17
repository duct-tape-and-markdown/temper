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
/// only the kind facts and their default contract clauses, exactly the family
/// `src/builtin_lock.toml` embeds (`sdk/src/builtins.ts`, the built-in kinds/default
/// contracts). `expect` is also the one surface a kind reaches the lock through, so
/// `supporting-doc` — whose members compose under a host rather than at a locus of their
/// own, and whose one default clause bounds their place in the graph rather than their
/// bytes — is bound here like any other, contributing its kind fact and that clause.
const MEMBERLESS_BUILTIN_PROGRAM: &str = r#"
import { emit, harness } from "@dtmd/temper";
import {
  agent,
  agentDefaultContract,
  command,
  commandDefaultContract,
  hook,
  hookDefaultContract,
  installedPlugin,
  installedPluginDefaultContract,
  marketplace,
  marketplaceDefaultContract,
  mcpServer,
  mcpServerDefaultContract,
  memory,
  memoryAnthropicDefaultContract,
  pluginManifest,
  pluginManifestDefaultContract,
  rule,
  ruleDefaultContract,
  skill,
  skillDefaultContract,
  supportingDoc,
  supportingDocDefaultContract,
} from "@dtmd/temper/claude-code";

const program = harness({
  members: [],
  expect: [
    { kind: agent, clauses: agentDefaultContract },
    { kind: command, clauses: commandDefaultContract },
    { kind: hook, clauses: hookDefaultContract },
    { kind: installedPlugin, clauses: installedPluginDefaultContract },
    { kind: marketplace, clauses: marketplaceDefaultContract },
    { kind: mcpServer, clauses: mcpServerDefaultContract },
    { kind: memory, clauses: memoryAnthropicDefaultContract },
    { kind: pluginManifest, clauses: pluginManifestDefaultContract },
    { kind: rule, clauses: ruleDefaultContract },
    { kind: skill, clauses: skillDefaultContract },
    { kind: supportingDoc, clauses: supportingDocDefaultContract },
  ],
});

process.stdout.write(emit(program).seam);
"#;

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
    let (_harness, into) = common::wire_sdk_harness("memberless", MEMBERLESS_BUILTIN_PROGRAM);

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
