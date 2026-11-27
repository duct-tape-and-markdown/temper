//! The `--frozen` lane for temper's own std-lib:
//! re-derive the built-in lock from `@dtmd/temper/claude-code` — a real `node`
//! subprocess running the built SDK, exactly as `tests/emit.rs` drives the seam — and
//! byte-compare its declaration rows against the embedded `src/builtin_lock.toml`.
//!
//! A memberless harness binds every built-in kind to every built-in default contract via
//! `expect` (`sdk/src/assembly.ts`: `expect` keys clauses to a kind value with no
//! member needed), the identical construction `src/builtin_lock.toml`'s own header
//! says produced it. `dial` is the one kind bound off the SDK's *root* rather than its
//! provider subpath — temper's own kind, not Claude Code's — and it is bound here like
//! any other, since what the embedded lock carries is the whole shipped std-lib.
//! Agreement is mechanical: this test is the CI job the fail-loud
//! invariant describes, never a human re-reading two files side by side.
//!
//! The memberless harness contributes a **root contract** as well as kind facts and
//! their defaults, and it does so by declaring nothing: `harness()` composes
//! `rootDefaultContract` wherever an author names no `contract`, so the program below —
//! which names none — carries the root's clauses into the embedded lock. That is what
//! gives the stranger gate (a harness with no lock at all) the same root contract an
//! emitted one gets, and it is why the program text needs no edit to pin the claim.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use temper::contract::{self, Predicate};
use temper::drift::{self, EmitOptions};
use temper::kind::{CustomKind, Registration};

use crate::common;
use crate::module_closure::without_module_rows;

/// A memberless harness binding every built-in kind to every built-in default contract via
/// `expect` — no members, so `compileDeclarations` (`sdk/src/declarations.ts`) emits
/// only the kind facts and their default contract clauses, exactly the family
/// `src/builtin_lock.toml` embeds (`sdk/src/builtins.ts`, the built-in kinds/default
/// contracts). `expect` is also the one surface a kind reaches the lock through, so
/// `supporting-doc` — whose members compose under a host rather than at a locus of their
/// own, and whose one default clause bounds their place in the graph rather than their
/// bytes — is bound here like any other, contributing its kind fact and that clause. So is
/// `handler`, the embedded kind `hook` templates: its six handler-level clauses are its
/// own, and `hookDefaultContract` spreads them, so the byte-compare below covers the new
/// kind row, those six rows under it, and `hook`'s new `templates` column.
pub(crate) const MEMBERLESS_BUILTIN_PROGRAM: &str = r#"
import { dial, dialDefaultContract, harness, requirement, requirementDefaultContract } from "@dtmd/temper";
import {
  agent,
  agentDefaultContract,
  command,
  commandDefaultContract,
  handler,
  handlerDefaultContract,
  hook,
  hookDefaultContract,
  installedPlugin,
  installedPluginDefaultContract,
  knownMarketplace,
  knownMarketplaceDefaultContract,
  marketplace,
  marketplaceDefaultContract,
  mcp,
  mcpDefaultContract,
  mcpServer,
  mcpServerDefaultContract,
  memory,
  memoryDefaultContract,
  outputStyle,
  outputStyleDefaultContract,
  pluginManifest,
  pluginManifestDefaultContract,
  rule,
  ruleDefaultContract,
  settings,
  settingsDefaultContract,
  settingsLocal,
  settingsLocalDefaultContract,
  skill,
  skillDefaultContract,
  supportingDoc,
  supportingDocDefaultContract,
  tool,
  toolDefaultContract,
} from "@dtmd/temper/claude-code";

const program = harness({
  members: [],
  expect: [
    { kind: agent, clauses: agentDefaultContract },
    { kind: command, clauses: commandDefaultContract },
    { kind: dial, clauses: dialDefaultContract },
    { kind: handler, clauses: handlerDefaultContract },
    { kind: hook, clauses: hookDefaultContract },
    { kind: installedPlugin, clauses: installedPluginDefaultContract },
    { kind: knownMarketplace, clauses: knownMarketplaceDefaultContract },
    { kind: marketplace, clauses: marketplaceDefaultContract },
    { kind: mcp, clauses: mcpDefaultContract },
    { kind: mcpServer, clauses: mcpServerDefaultContract },
    { kind: memory, clauses: memoryDefaultContract },
    { kind: outputStyle, clauses: outputStyleDefaultContract },
    { kind: pluginManifest, clauses: pluginManifestDefaultContract },
    { kind: rule, clauses: ruleDefaultContract },
    { kind: settings, clauses: settingsDefaultContract },
    { kind: settingsLocal, clauses: settingsLocalDefaultContract },
    { kind: skill, clauses: skillDefaultContract },
    { kind: supportingDoc, clauses: supportingDocDefaultContract },
    { kind: tool, clauses: toolDefaultContract },
    { kind: requirement, clauses: requirementDefaultContract },
  ],
});

export default program;
"#;

/// A lock text split at the first declaration row: everything ahead of
/// `[[declaration.kind]]`, then the rows themselves. Both sides of the byte-compare go
/// through this one helper, so the compare holds exactly the rows it claims to and each
/// side's prefix is asserted for what it is — the embedded file's hand-authored
/// provenance header on one side, the engine stamp on the other. Neither prefix is part
/// of the declaration family: the header is prose `drift::emit` never writes, and the
/// stamp is a fact about the writer, not one of the program's erased declarations.
fn split_at_declaration_rows(label: &str, text: &str) -> (String, String) {
    let start = text
        .find("[[declaration.kind]]")
        .unwrap_or_else(|| panic!("the {label} lock carries declaration rows"));
    (text[..start].to_string(), text[start..].to_string())
}

/// The embedded lock's text, read whole — the frozen artifact the derived rows are held
/// against. Deliberately unstamped: it is re-derived by the very compare below, so a live
/// engine version in it would break on every release bump.
fn embedded_lock_text() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/builtin_lock.toml");
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("the embedded built-in lock must exist at {path:?}: {err}"))
}

#[test]
fn the_embedded_builtin_lock_byte_equals_the_sdk_modules_own_memberless_emit() {
    let (_harness, into) = common::wire_sdk_harness("memberless", MEMBERLESS_BUILTIN_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).expect(
        "re-deriving the built-in lock requires a working node + the built \
         @dtmd/temper/claude-code module — the --frozen discipline fails loud here \
         rather than silently skipping the comparison",
    );

    let derived_text = fs::read_to_string(into.join("lock.toml"))
        .expect("emit_program writes a lock.toml carrying the memberless declaration rows");
    // The module family is a fact about the program that ran, never the std-lib: a
    // stranger gate fingerprinting this run's `.temper/harness.ts` would hash a file no
    // other harness has. So the derived side carries it and the compare drops it whole.
    assert!(
        derived_text.contains("source_path = \".temper/harness.ts\""),
        "the derived lock fingerprints the program that produced it"
    );
    assert!(
        !embedded_lock_text().contains("[[declaration.module]]"),
        "the embedded lock carries no module rows"
    );
    let derived_text =
        common::without_fixture_typecheck_rows(&without_module_rows(&derived_text, |_| true));
    let (derived_prefix, derived) = split_at_declaration_rows("derived", &derived_text);
    let (_header, embedded) = split_at_declaration_rows("embedded", &embedded_lock_text());

    assert_eq!(
        derived, embedded,
        "src/builtin_lock.toml has drifted from @dtmd/temper/claude-code's own memberless \
         emit — regenerate it by re-running that emit and re-embedding the resulting rows \
         verbatim (src/builtin_lock.toml's own header), never by hand-editing a row"
    );

    // And the compare stays honest about what it dropped. A memberless emit writes one
    // provenance row — the `temper-sdk` counsel rule every emit synthesizes — so the
    // engine stamp and that row are the whole of the derived prefix. Spelled out
    // byte-exact (bar the row's hashes) so this lane keeps noticing any *other* root key
    // emit grows, rather than silently widening the slice to swallow it.
    let counsel_hash = derived_prefix
        .lines()
        .find_map(|line| line.strip_prefix("source_hash = "))
        .expect("the counsel rule's provenance row carries its source hash");
    assert_eq!(
        derived_prefix,
        format!(
            "engine = \"{}\"\n\n[[rule]]\nname = \"temper-sdk\"\n\
             source_path = \".claude/rules/temper-sdk.md\"\n\
             source_hash = {counsel_hash}\nemit_hash = {counsel_hash}\n\n",
            temper::VERSION
        ),
        "the only thing the derived side carries ahead of its declaration rows is the \
         engine stamp; a new root key belongs in this lane's compare, not in the prefix \
         it drops"
    );
}

/// The derived lock carries the **root member's** default contract, not kind floors
/// alone — the half a byte-compare alone would pass silently if `harness()` ever stopped
/// defaulting `contract`, since both sides would simply lose the rows together.
///
/// Asserted off the freshly derived lock rather than the embedded copy: the claim under
/// test is the SDK's defaulting, and the embedded file is downstream of it.
#[test]
fn the_sdk_derived_lock_carries_the_root_members_default_contract() {
    let (_harness, into) = common::wire_sdk_harness("memberless", MEMBERLESS_BUILTIN_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).expect(
        "re-deriving the built-in lock requires a working node + the built @dtmd/temper/claude-code module",
    );

    let lock_path = into.join("lock.toml");
    let text = fs::read_to_string(&lock_path).expect("emit_program writes a lock.toml");
    let declarations =
        drift::parse_declarations(&lock_path, &text).expect("the derived lock parses");

    // The program declares no `contract`, so every root row here is the shipped
    // default's — and the absent `kind` column is the whole of what makes it the root's.
    let root: Vec<_> = declarations
        .clauses
        .iter()
        .filter(|row| row.kind.is_none())
        .collect();
    assert_eq!(
        root.iter()
            .map(|row| (
                row.id.as_deref(),
                row.predicate.as_str(),
                row.severity.as_str()
            ))
            .collect::<Vec<_>>(),
        vec![
            (Some("root.reachable"), "reachable", "advisory"),
            (Some("root.fresh"), "fresh", "advisory"),
            (Some("root.locus-declared"), "locus-declared", "advisory"),
            (Some("root.engine-matches"), "engine-matches", "advisory"),
            (Some("root.resolves"), "resolves", "required"),
            (Some("root.import-backed"), "import-backed", "advisory"),
            (
                Some("root.listing.description+when_to_use"),
                "listing",
                "advisory"
            ),
        ],
        "the memberless emit contributes the root default's rows, got {root:#?}"
    );
    assert!(
        root.iter().all(|row| row.guidance.is_some()),
        "every shipped root clause teaches"
    );
    // Cited, every one: Claude Code's docs where a verdict rests on documented
    // behaviour, temper's own corpus where it compares temper's lock or version.
    assert!(
        root.iter().all(|row| row.cite.is_some()),
        "every shipped root clause is cited, got {root:#?}"
    );

    // And they lift back through the engine's own root-contract reader, so what ships is
    // a contract the gate can run, never a row family that merely parses.
    assert_eq!(
        temper::compose::root_contract_from_rows(&declarations.clauses)
            .expect("the derived root rows lift")
            .clauses
            .iter()
            .map(|clause| clause.predicate.clone())
            .collect::<Vec<_>>(),
        vec![
            contract::Predicate::Reachable,
            contract::Predicate::Fresh,
            contract::Predicate::LocusDeclared,
            contract::Predicate::EngineMatches,
            contract::Predicate::Resolves,
            contract::Predicate::ImportBacked,
            contract::Predicate::Listing {
                fields: vec!["description".to_string(), "when_to_use".to_string()],
                entry: 1536,
                max: 8000,
                withheld_by: Some("disable-model-invocation".to_string()),
                kinds: vec!["skill".to_string(), "command".to_string()],
            },
        ]
    );
}

#[test]
fn the_sdk_derived_installed_plugin_kind_round_trips_through_the_engine_reader() {
    let (_harness, into) = common::wire_sdk_harness("memberless", MEMBERLESS_BUILTIN_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).expect(
        "re-deriving the built-in lock requires a working node + the built @dtmd/temper/claude-code module",
    );

    let derived_lock =
        fs::read_to_string(into.join("lock.toml")).expect("emit_program writes a lock.toml");

    let lock_path = into.join("lock.toml");
    let declarations = drift::parse_declarations(&lock_path, &derived_lock)
        .expect("the derived lock parses as valid declarations");

    assert_eq!(
        declarations.kinds.len(),
        20,
        "the derived lock carries all 20 built-in kind facts"
    );

    for kind_row in &declarations.kinds {
        let msg = format!(
            "the {} kind fact decodes through the engine reader",
            kind_row.name
        );
        CustomKind::from_kind_fact_row(kind_row).expect(&msg);
    }

    let installed_plugin_row = declarations
        .kinds
        .iter()
        .find(|k| k.name == "installed-plugin")
        .expect("the derived lock carries the installed-plugin kind fact");

    let decoded = CustomKind::from_kind_fact_row(installed_plugin_row)
        .expect("the installed-plugin kind fact decodes through the engine reader");

    assert!(
        decoded.registration.contains(&Registration::Enablement {
            field: "enabled".to_string()
        }),
        "the decoded registration set contains the field-carrying Enablement variant"
    );

    let rule_mention_reachable_row = declarations
        .clauses
        .iter()
        .find(|c| {
            c.predicate == "mention-reachable"
                && c.kind.as_deref() == Some("rule")
                && c.field.as_deref() == Some("paths")
        })
        .expect("the derived lock carries the rule's mention-reachable clause row");

    assert_eq!(
        contract::predicate_from_row(rule_mention_reachable_row),
        Some(Predicate::MentionReachable {
            scope_field: "paths".to_string(),
            gate_field: "paths".to_string()
        }),
        "the real SDK-derived rule.mention-reachable gate-column spelling lifts through the engine reader"
    );
}

#[test]
fn no_two_clause_rows_in_the_shipped_lock_share_a_label() {
    // A label is a clause's address: the dial names a clause by it and every finding
    // prints it. Two rows wearing one address are undialable and their findings
    // indistinguishable — and this is the one lock no collision diagnostic reads
    // (`gate.rs` walks the consumer's own declarations), so nothing but this test
    // stands between a shipped collision and an adopter.
    //
    // Every depth. A guard's body rows are ordinary clauses addressed under their host,
    // so they share the namespace the rows above them do.
    let lock_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/builtin_lock.toml");
    let text = fs::read_to_string(&lock_path).expect("the embedded built-in lock must exist");
    let declarations =
        drift::parse_declarations(&lock_path, &text).expect("the embedded lock parses");

    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    // Guards do not nest, so a row and its body exhaust the lock's clause depth.
    for row in declarations
        .clauses
        .iter()
        .flat_map(|row| std::iter::once(row).chain(row.body.iter().flatten()))
    {
        let label = row
            .id
            .as_deref()
            .expect("emit stamps every lock clause row with its label");
        *seen.entry(label).or_default() += 1;
    }

    let collisions: Vec<_> = seen
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(label, count)| format!("{label} ×{count}"))
        .collect();
    assert!(
        collisions.is_empty(),
        "every clause row in the shipped lock addresses itself uniquely: {collisions:?}"
    );

    let when_labels: Vec<&str> = declarations
        .clauses
        .iter()
        .filter(|row| row.predicate == "when")
        .filter_map(|row| row.id.as_deref())
        .collect();
    assert!(
        when_labels.contains(&"mcp-server.when.type=stdio")
            && when_labels.contains(&"mcp-server.when.type=http+sse+streamable-http+ws"),
        "a `when` label carries its guard's value set, sorted and `+`-joined: {when_labels:?}"
    );

    // A body row addresses `<host-label>.<predicate>.<field>` — the host's own address is
    // the owner segment, which is what parts the `url` and `git-subdir` guards' twin
    // `source.url` demands.
    let body_labels: Vec<&str> = declarations
        .clauses
        .iter()
        .flat_map(|row| row.body.iter().flatten())
        .filter_map(|row| row.id.as_deref())
        .collect();
    assert!(
        body_labels.contains(&"marketplace.when.plugins[*].source.source=url.required.source.url")
            && body_labels.contains(
                &"marketplace.when.plugins[*].source.source=git-subdir.required.source.url"
            ),
        "a body label is owned by its host guard's own address: {body_labels:?}"
    );
}
