//! Library-level proofs over the read family's `why` traversal
//! (`specs/architecture/20-surface.md`, "Decision: one read verb — `explain`").
//!
//! The four read *CLI verbs* (`why`/`requirements`/`impact`/`context`) retired at
//! CLI-COLLAPSE; their traversals re-home under the single `explain` verb at
//! EXPLAIN-UNIFY (fork-gated on `(explain-target-disambiguation)`), so there is no CLI
//! surface to drive here yet. The traversal *engine* survives untouched
//! ([`temper::read`]) for `explain` to reuse — so this file exercises the read library
//! directly, proving the floor-binding resolution (`bound_package`) the narration stands
//! on independent of any CLI spelling.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-read-verbs-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Floor-binding narration over the read family's public `why` API (READ-FLOOR-BINDING-DEFAULT):
/// `bound_package` names each embedded kind's *real* floor — resolved through the one
/// `QUALIFIED_FLOOR_BINDINGS` table — instead of defaulting every non-rule kind to
/// `skill.anthropic`. A memory member carries the disambiguated qualified identity
/// (`claude-code.memory`/`agents-md.memory`; the bare `memory` collides across two
/// providers), so it is threaded as a custom member the way a qualified built-in reaches
/// the read family. Skills/rules keep their own floors — these exercise the resolution
/// branches directly.
mod floor_binding {
    use std::collections::BTreeMap;

    use temper::check::Workspace;
    use temper::compose::Requirement;
    use temper::extract::Features;
    use temper::read::{self, CustomMember};

    use super::tmpdir;

    /// Narrate one custom member (its `kind` and `id`) through `why` over an otherwise-empty
    /// surface, returning the stdout narration. The workspace loads an empty temp dir (no
    /// skills/rules) and the roster/edge inputs are empty, so the governing-package line is
    /// all this exercises.
    fn why_kind(kind: &str, id: &str) -> String {
        let ws = Workspace::load(&tmpdir("floor-binding")).unwrap();
        let custom = [CustomMember {
            kind: kind.to_string(),
            id: id.to_string(),
            satisfies: Vec::new(),
        }];
        let roster: BTreeMap<String, Requirement> = BTreeMap::new();
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::new();
        read::why(&ws, None, &custom, &roster, &by_kind, &[], id)
    }

    #[test]
    fn a_memory_member_names_its_own_floor_never_the_skill_floor() {
        // `claude-code.memory` binds `memory.anthropic`; the `agents-md` provider binds
        // `memory.agents-md`. Neither may be mis-narrated as `skill.anthropic` — the
        // default-to-skill bug this entry closes.
        let claude = why_kind("claude-code.memory", "project-memory");
        assert!(
            claude.contains("binds the `memory.anthropic` package"),
            "a claude-code memory member is bound to its own floor: {claude}"
        );
        assert!(
            !claude.contains("skill.anthropic"),
            "a memory member is never narrated as skill-bound: {claude}"
        );

        let agents = why_kind("agents-md.memory", "AGENTS");
        assert!(
            agents.contains("binds the `memory.agents-md` package"),
            "an agents-md memory member is bound to its own floor: {agents}"
        );
    }

    #[test]
    fn a_bare_builtin_name_resolves_to_its_qualified_floor() {
        // `skill`/`rule` reach `bound_package` as bare names; each resolves to its qualified
        // identity (`claude-code.skill`/`claude-code.rule`) and names its own floor.
        let skill = why_kind("skill", "reviewer");
        assert!(
            skill.contains("binds the `skill.anthropic` package"),
            "{skill}"
        );
        let rule = why_kind("rule", "collaboration");
        assert!(
            rule.contains("binds the `rule.anthropic` package"),
            "{rule}"
        );
    }

    #[test]
    fn a_floorless_kind_falls_back_to_its_own_name() {
        // A kind with no author binding and no embedded floor is named by its own kind name,
        // not silently mis-bound to the skill floor.
        let out = why_kind("adr", "0001-adopt-temper");
        assert!(out.contains("binds the `adr` package"), "{out}");
        assert!(!out.contains("skill.anthropic"), "{out}");
    }
}
