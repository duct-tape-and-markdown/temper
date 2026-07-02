//! End-to-end proof of kind-declared reference normalization
//! (`specs/15-kinds.md`, "Decision: reference resolution is declared by the kind,
//! never guessed by the engine").
//!
//! A custom `spec` kind composes a `references` primitive that declares
//! `strip_suffix = ".md"` and a `[[relationships]]` edge over that reference
//! syntax. Its members carry backtick-filename references (`` `NN-name.md` ``) at
//! sibling member ids. Extracted through the declared normalization, each ref maps
//! to the sibling's unit id and resolves under [`temper::graph::check`] with zero
//! dangling findings — the exact-match discipline the Decision demands.
//!
//! The control loads the *same* kind minus the `strip_suffix` declaration: the same
//! refs keep their `.md` tail, match no member id, and every one dangles. That
//! contrast is the whole point — the engine applies exactly the declared rule and
//! then demands an exact id match, never a loose fallback that could mask a genuine
//! dangling reference.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::extract::Features;
use temper::kind::{CustomKind, Unit};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "reference-resolution-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Write a `spec` kind's authored definition at `<kinds_dir>/spec/KIND.md` and load
/// it. The header composes a `references` primitive (optionally declaring the
/// `strip_suffix` normalization) and a `references` → `spec` relationship, so the
/// loaded [`CustomKind`] carries both the extractor and the declared edge exactly as
/// a project's own `KIND.md` would.
fn spec_kind(kinds_dir: &std::path::Path, strip_suffix: bool) -> CustomKind {
    let normalization = if strip_suffix {
        "strip_suffix = \".md\"\n"
    } else {
        ""
    };
    let kind_md = format!(
        "+++\n\
         governs = {{ root = \"specs\", glob = \"*.md\" }}\n\
         \n\
         [[extraction]]\n\
         primitive = \"references\"\n\
         feature = \"references\"\n\
         {normalization}\
         \n\
         [[relationships]]\n\
         field = \"references\"\n\
         to = \"spec\"\n\
         +++\n\
         # spec\n\
         \n\
         temper's own custom kind, governing `specs/`.\n"
    );
    let dir = kinds_dir.join("spec");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("KIND.md"), kind_md).unwrap();
    CustomKind::load(kinds_dir, "spec").unwrap()
}

/// A raw spec member: a frontmatter-less unit whose body carries the given
/// backtick-filename references, at the unit id `id`.
fn member(id: &str, body: &str) -> Unit {
    Unit {
        id: id.to_string(),
        frontmatter: BTreeMap::new(),
        body: body.to_string(),
        source_path: PathBuf::from(format!("specs/{id}.md")),
        satisfies: Vec::new(),
    }
}

/// The corpus every case shares: three members cross-referencing each other by
/// backtick-filename (`` `NN-name.md` ``). Every reference names a sibling that
/// exists, so once normalized to an id it resolves; un-normalized it dangles.
fn corpus() -> Vec<Unit> {
    vec![
        member(
            "00-intent",
            "# Intent\n\nThe extraction half lives in `15-kinds.md`.\n",
        ),
        member(
            "10-contracts",
            "# Contracts\n\nThe north star is `00-intent.md`; kinds are `15-kinds.md`.\n",
        ),
        member(
            "15-kinds",
            "# Kinds\n\nThe predicate half is `10-contracts.md`.\n",
        ),
    ]
}

/// Extract every member of the corpus through `kind`'s composed extractor, yielding
/// the `Features` slice `graph::check` reads.
fn extract(kind: &CustomKind, units: &[Unit]) -> Vec<Features> {
    units
        .iter()
        .map(|unit| kind.extraction.extract(unit))
        .collect()
}

#[test]
fn a_declared_strip_suffix_resolves_backtick_filename_refs_to_member_ids() {
    let kinds_dir = tmpdir("resolves");
    let kind = spec_kind(&kinds_dir, true);
    let units = corpus();
    let features = extract(&kind, &units);
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &features[..])]);

    // With `strip_suffix = ".md"`, `` `15-kinds.md` `` normalizes to the unit id
    // `15-kinds`, so every cross-reference resolves to a real sibling — zero dangles.
    let diagnostics = temper::graph::check(&kind.relationships, &by_kind);
    assert!(
        diagnostics.is_empty(),
        "declared reference normalization resolves every backtick-filename ref to a member id, got:\n{diagnostics:#?}"
    );
}

#[test]
fn without_the_declared_normalization_the_same_refs_dangle() {
    let kinds_dir = tmpdir("dangles");
    let kind = spec_kind(&kinds_dir, false);
    let units = corpus();
    let features = extract(&kind, &units);
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &features[..])]);

    // The control declares no `strip_suffix`: the reference value keeps its `.md`
    // tail (`` `15-kinds.md` ``), which matches no member id (`15-kinds`), so the
    // engine's exact match dangles rather than falling back to a loose comparison
    // that could mask a genuine break (the Decision's rejected alternative (b)).
    let diagnostics = temper::graph::check(&kind.relationships, &by_kind);
    assert!(
        !diagnostics.is_empty(),
        "without the declared normalization the `.md`-tailed refs match no id and must dangle"
    );
    assert!(
        diagnostics
            .iter()
            .all(|diagnostic| diagnostic.rule == "graph.route"),
        "every finding is a route-resolution dangle, got:\n{diagnostics:#?}"
    );
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("15-kinds.md")),
        "a dangling finding names the un-normalized `.md`-tailed target, got:\n{diagnostics:#?}"
    );
}
