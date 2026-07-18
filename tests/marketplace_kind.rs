//! The `marketplace` built-in kind: `.claude-plugin/marketplace.json`, the catalog a
//! marketplace distributes plugins through (`specs/builtins.md`, "The shipped kinds"; 0031).
//!
//! The second built-in at the `json-document` format and the tenth kind of the roster
//! decision 0031 ratified. It shares the `.claude-plugin` root with `plugin-manifest` and
//! is told apart by its glob alone, so the two file kinds never contend — the fact this
//! module drives first.
//!
//! Its contract's centerpiece is the reserved-names deny list, the one clause here that is
//! worth more than a lint: Claude Code re-checks the list on *every* load, so a catalog
//! published under a name that later becomes reserved stops loading for every user who
//! already added it.
//!
//! The documented rules *below* the top level are gated here too — `owner.name` and each
//! `plugins[]` entry's `name`/`source`, reached by the clause addressing subset. One rule
//! is still out of reach and this module pins that boundary rather than implying coverage
//! it lacks: which form a `source` is remains a union no clause spells, so an undocumented
//! form passes. `sdk/src/builtins.ts` (`marketplaceDefaultContract`) names that hold; the
//! `MarketplaceSource` TypeScript type holds the bar for an SDK author.

use std::fs;

mod common;

use common::{check_harness, write_marketplace_json};

use temper::builtin_kind;
use temper::json_manifest::DocumentMember;
use temper::kind::{Content, Format, Governs, Registration, UnitShape};

/// A catalog in the real Claude Code shape, exercising every documented `source` form: the
/// relative path and the four object forms — `github`, `url`, `git-subdir`, `npm`
/// (code.claude.com/docs/en/plugin-marketplaces, "Plugin sources", retrieved 2026-07-16).
const MARKETPLACE_JSON: &str = r#"{
  "name": "acme-tools",
  "owner": { "name": "DevTools Team", "email": "tools@acme.example" },
  "description": "Acme's internal plugin catalog",
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/formatter" },
    { "name": "deployment-tools", "source": { "source": "github", "repo": "acme/deploy-plugin" } },
    { "name": "git-plugin", "source": { "source": "url", "url": "https://gitlab.com/team/plugin.git", "ref": "main" } },
    { "name": "mono-plugin", "source": { "source": "git-subdir", "url": "https://github.com/acme/monorepo.git", "path": "tools/claude-plugin" } },
    { "name": "npm-plugin", "source": { "source": "npm", "package": "@acme/claude-plugin", "version": "2.1.0" } }
  ]
}
"#;

fn marketplace_kind() -> temper::kind::CustomKind {
    builtin_kind::definition("marketplace").expect("marketplace is embedded")
}

#[test]
fn the_marketplace_kind_owns_its_file_at_a_glob_its_sibling_never_contends_for() {
    let marketplace = marketplace_kind();

    assert_eq!(
        marketplace.governs,
        Some(Governs {
            root: ".claude-plugin".to_string(),
            glob: "marketplace.json".to_string(),
        })
    );
    assert_eq!(marketplace.format, Some(Format::JsonDocument));
    // Identity from the document's own key: the stem is `marketplace` for every catalog
    // ever written, so the named-field mode is the only one that tells two apart.
    assert_eq!(
        marketplace.unit_shape,
        Some(UnitShape::NamedField {
            field: "name".to_string(),
        })
    );
    // It *is* the catalog rather than surfacing inside one — so it owns its file.
    assert_eq!(marketplace.collection_address, None);
    assert_eq!(marketplace.content, Content::File);
    // Channel-less: a catalog reaches the installer, never the model.
    assert_eq!(marketplace.registration, Vec::<Registration>::new());

    // The two `.claude-plugin` kinds share a root and are separated by their globs alone.
    let manifest =
        builtin_kind::definition("plugin-manifest").expect("plugin-manifest is embedded");
    let (marketplace_governs, manifest_governs) = (
        marketplace.governs.expect("marketplace governs a locus"),
        manifest.governs.expect("plugin-manifest governs a locus"),
    );
    assert_eq!(marketplace_governs.root, manifest_governs.root);
    assert_ne!(marketplace_governs.glob, manifest_governs.glob);
}

#[test]
fn a_marketplace_json_surfaces_one_member_whose_identity_is_its_name_field() {
    let harness = common::tmpdir("marketplace-read");
    write_marketplace_json(&harness, MARKETPLACE_JSON);

    let member = DocumentMember::read(
        &marketplace_kind(),
        &harness.join(".claude-plugin/marketplace.json"),
    )
    .unwrap();

    // Identity is the `name` value, never the `marketplace` stem.
    assert_eq!(member.id, "acme-tools");
    // The document's top-level keys are the member's fields, so a clause ranges over a
    // catalog exactly as it ranges over a frontmatter member.
    let fields: Vec<&str> = member.fields.keys().map(String::as_str).collect();
    assert_eq!(fields, vec!["description", "name", "owner", "plugins"]);
}

#[test]
fn a_well_formed_catalog_carrying_every_documented_source_form_passes_the_gate_clean() {
    let harness = common::tmpdir("marketplace-clean");
    write_marketplace_json(&harness, MARKETPLACE_JSON);

    let (findings, ok) = check_harness(&harness);
    assert!(ok, "a documented catalog gates clean: {findings:?}");
    // Each of the five documented `source` forms resolves: the contract gates what the
    // format decides, never what it merely allows.
    assert!(
        findings.iter().all(|f| !f.starts_with("::error")),
        "{findings:?}"
    );
    // And the member is really being checked, not silently skipped past.
    assert!(
        findings.iter().any(|f| f.contains("marketplace (1)")),
        "{findings:?}"
    );
}

#[test]
fn a_reserved_name_is_a_finding_even_though_it_is_well_formed_kebab_case() {
    let harness = common::tmpdir("marketplace-reserved");
    // `anthropic-plugins` is reserved for official use: kebab-case and non-empty, so every
    // other name clause passes it — the deny list is the only clause that catches it.
    write_marketplace_json(
        &harness,
        r#"{ "name": "anthropic-plugins", "owner": { "name": "Someone" }, "plugins": [] }"#,
    );

    let (findings, ok) = check_harness(&harness);
    assert!(!ok, "a reserved name fails the gate");
    let denied = common::findings_for(&findings, "marketplace.deny.name");
    assert!(!denied.is_empty(), "{findings:?}");
    assert!(
        denied.iter().all(|f| f.starts_with("::error")),
        "a reserved name stops the catalog loading, so it is never a note: {findings:?}"
    );
    // The charset clause has nothing to say about it — proof the deny list is load-bearing
    // rather than incidentally shadowed by the kebab-case rule.
    assert!(
        common::findings_for(&findings, "marketplace.allowed_chars.name").is_empty(),
        "{findings:?}"
    );
}

#[test]
fn a_name_that_merely_resembles_an_official_one_is_no_finding_because_no_clause_decides_it() {
    let harness = common::tmpdir("marketplace-impersonation");
    // The docs block names that *impersonate* official marketplaces, and this is the page's
    // own example. Impersonation is semantic judgment, so it ships as the deny clause's
    // guidance and never as a clause (`specs/intent.md`, invariant 2; `specs/builtins.md`,
    // "Undecidable properties are deliberately absent"). Claude Code would refuse this
    // catalog; temper's gate does not, and that divergence is deliberate — a clause that
    // guessed at impersonation would fire on true negatives.
    write_marketplace_json(
        &harness,
        r#"{ "name": "official-claude-plugins", "owner": { "name": "Someone" }, "plugins": [] }"#,
    );

    let (findings, ok) = check_harness(&harness);
    assert!(
        ok,
        "impersonation is undecidable, so no clause fires: {findings:?}"
    );
    assert!(
        common::findings_for(&findings, "marketplace.deny.name").is_empty(),
        "{findings:?}"
    );
}

#[test]
fn a_catalog_with_no_name_refuses_loud_rather_than_degrading_to_a_nameless_member() {
    let harness = common::tmpdir("marketplace-no-name");
    write_marketplace_json(
        &harness,
        r#"{ "owner": { "name": "Someone" }, "plugins": [] }"#,
    );

    let run = common::check_harness_in(&harness, None);

    // `name` is this kind's identity, not merely a required field, so its absence is a
    // *read* refusal that never reaches the clause — the `plugin-manifest` precedent. The
    // contract's own `required("name")` clause states the rule portably regardless.
    assert!(!run.ok, "a nameless catalog fails the gate");
    assert!(
        run.output
            .contains("temper::json_manifest::no_identity_value"),
        "{}",
        run.output
    );
}

#[test]
fn a_name_outside_the_kebab_case_charset_is_a_finding() {
    let harness = common::tmpdir("marketplace-bad-name");
    write_marketplace_json(
        &harness,
        r#"{ "name": "Acme Tools", "owner": { "name": "Someone" }, "plugins": [] }"#,
    );

    let (findings, ok) = check_harness(&harness);
    assert!(!ok, "a non-kebab-case name fails the gate");
    assert!(
        !common::findings_for(&findings, "marketplace.allowed_chars.name").is_empty(),
        "{findings:?}"
    );
}

#[test]
fn a_catalog_missing_owner_or_plugins_is_a_finding() {
    let harness = common::tmpdir("marketplace-missing-required");
    // Both required top-level keys absent — the presence rules the algebra *can* address.
    write_marketplace_json(&harness, r#"{ "name": "acme-tools" }"#);

    let (findings, ok) = check_harness(&harness);
    assert!(!ok, "a catalog missing owner and plugins fails the gate");
    let required = ["marketplace.required.owner", "marketplace.required.plugins"]
        .iter()
        .flat_map(|rule| common::findings_for(&findings, rule))
        .collect::<Vec<_>>();
    assert_eq!(
        required.len(),
        2,
        "one finding each for `owner` and `plugins`: {findings:?}"
    );
    assert!(required.iter().any(|f| f.contains("owner")), "{findings:?}");
    assert!(
        required.iter().any(|f| f.contains("plugins")),
        "{findings:?}"
    );
}

#[test]
fn the_documented_rules_below_the_top_level_fail_through_the_real_gate() {
    let harness = common::tmpdir("marketplace-nested");
    // Three documented rules below the top level, violated at once: `owner` carries no
    // `name`, the first entry no `source`, the second no `name`. Claude Code refuses each,
    // and so does the gate — `owner.name` walks the object, `plugins[*].name`/`.source`
    // grain over the catalog.
    //
    // The third entry names a `source` form the docs do not document (`ftp`). That one
    // still passes, and deliberately: which form a source is remains a union no clause
    // spells, and it is the one hold `marketplaceDefaultContract`'s header still names.
    write_marketplace_json(
        &harness,
        r#"{
  "name": "acme-tools",
  "owner": { "email": "tools@acme.example" },
  "plugins": [
    { "name": "no-source" },
    { "source": "./plugins/nameless" },
    { "name": "bogus", "source": { "source": "ftp", "host": "files.acme.example" } }
  ]
}
"#,
    );

    let (findings, ok) = check_harness(&harness);
    assert!(
        !ok,
        "each violated rule is an error-severity finding: {findings:?}"
    );

    // `owner.name` — the nested key, named by its own path.
    assert!(
        findings.iter().any(|f| f.contains("owner.name")),
        "{findings:?}"
    );
    // The each-grain findings name the offending *entry* by its index, so an author with a
    // fifty-plugin catalog is pointed at the one that is wrong rather than at `plugins`.
    assert!(
        findings.iter().any(|f| f.contains("plugins[0].source")),
        "{findings:?}"
    );
    assert!(
        findings.iter().any(|f| f.contains("plugins[1].name")),
        "{findings:?}"
    );
    // The entries that carry the field are not indicted — the grain fires per element,
    // never once for the array.
    assert!(
        !findings.iter().any(|f| f.contains("plugins[2]")),
        "the third entry names both fields; only its source *form* is unchecked: {findings:?}"
    );
}

#[test]
fn a_well_formed_catalog_passes_every_addressing_clause() {
    // The real-shaped catalog above fills `owner.name` and every entry's `name`/`source`,
    // so the paths that fire on the broken one are silent here: the each-grain is a gate,
    // not a tax on a correct catalog.
    let harness = common::tmpdir("marketplace-clean");
    write_marketplace_json(&harness, MARKETPLACE_JSON);

    let (findings, ok) = check_harness(&harness);
    assert!(ok, "a documented-shape catalog is clean: {findings:?}");
    assert!(
        findings.iter().any(|f| f.contains("marketplace (1)")),
        "the catalog is read and checked, not skipped: {findings:?}"
    );
}

#[test]
fn this_repo_authors_no_marketplace_so_the_kind_counts_zero_members() {
    // Honest, not a gap: temper's own harness is not a distribution catalog, so the shipped
    // kind has nothing to govern here — the `plugin-manifest`/`supporting-doc` precedent.
    let harness = common::tmpdir("marketplace-absent");
    fs::create_dir_all(harness.join(".claude")).unwrap();

    let (findings, ok) = check_harness(&harness);
    assert!(ok, "no catalog is no finding: {findings:?}");
    assert!(
        common::findings_for(&findings, "marketplace.required.owner").is_empty()
            && common::findings_for(&findings, "marketplace.required.plugins").is_empty(),
        "{findings:?}"
    );
}
