## Surface

`LAYOUT-PROSE-REGION-UNREACHABLE-LOCK-EXPLAIN` cannot reach green inside its
fence, and the shape its notes prescribe — a `nested_member` row with
`kind = "prose"`, which the 2026-09-07 revert's snapshot diff endorsed — is the
reason. That diff was only the *first* insta failure; accepting it exposes a
second, harder one the earlier attempt never saw.

Implemented in-fence and verified locally (`cargo test --test layout_kind`
14/14 green, including a new regression that pins the non-empty captured span,
the committed lock row, and `explain` narrating it at both member and leaf
grain). Then `cargo test --test gauntlet` with both snapshots accepted:

```
-ok = true
+ok = false
+::error title=nested-member.admissibility::spec:representation: nested member
+ `__prose__` under `spec:representation` is of kind `prose`, which no host
+ declares as a nested template — an orphaned lock row no kind's `templates` or
+ member collection admits
-... across 20 kinds: ... invariant (2 embedded), ... rule (2), ...
+... across 21 kinds: ... invariant (2 embedded), ... prose (1 embedded), ...
```

The `nested_member` family is typed as *embedded members of a kind a host
templates*, and two independent surfaces enforce that — neither in the fence:

- `src/admissibility.rs:23` `declared_embedded_kinds` builds its set from each
  kind row's path-less `templates` entries and each layout region's
  `member_kind`. A **prose** region carries no `member_kind` by construction, so
  a prose row is an orphan and `nested_member_admissibility`
  (`src/admissibility.rs:60`) errors on it. This turns the gauntlet corpus's
  `check` red — the one corpus whose green is the self-hosting bar
  (`specs/intent.md`, "Self-hosting"). Accepting `ok = false` into
  `gauntlet__check_diagnostics.snap` would be in-fence and is not an option.
- `src/gate.rs:504` groups `nested_member_counts` straight off the lock's rows,
  independent of the declared set, so the synthetic row also announces a kind
  that does not exist in the `coverage.checked` disclosure
  (`src/coverage_note.rs:80`) — a phantom `prose (1 embedded)` beside the real
  kinds.

`src/compose.rs:1139` `embedded_features_by_kind` seeds from the same declared
set, so today the row is *silently skipped* there while
`drift::nested_members_from_rows` (host-address keyed, kind-agnostic) still
lifts it onto the host's `Features` — which is exactly why `explain` narrates
it and the gate refuses it. That split is the admissibility rule's own stated
rationale ("the by-kind corpus would unmodel it while the host-address read
still carries it"), landed on by this entry from the other side.

Read together: the cascade is not three fixups, it is the family rejecting the
tenant. A verbatim prose region has no kind, no identity, and no `templates`
entry — it is the host's *own* content, not a member embedded in it.

## Observed at

697e9b49 (HEAD when observed) — the gauntlet's two snapshots are the whole
delta; `cargo test` was otherwise green with the change applied (311 unit +
every integration suite).

## Suggested consolidation

Re-scope with a **dedicated `layout_prose` declaration family**, not a widened
`nested_member`. One row per layout host (`host`, `span`); a layout admits at
most one verbatim prose region (`src/admissibility.rs:324` refuses a second, and
`Layout::read`'s `preamble_taken` makes it a permanent no-op), so no key or
ordinal is needed and the family is keyed by host alone. It then needs no
`declared_embedded_kinds` exemption, adds no phantom kind to the by-kind corpus
or the coverage disclosure, and says on its face what it carries.

Fence needed: the entry's four paths **plus**

- `src/extract.rs` — a `layout_prose: Option<String>` column on `Features`, so
  `read.rs` narrates the span the same way it reads every other feature rather
  than pattern-matching a magic nested-member key.
- `src/compose.rs` — populate it off the row beside `nested_members`
  (`local_document_rows` already routes the local-locus half through
  `drift::read_layout_document`, so both loci land on one read).
- `sdk/src/generated/LayoutProseRow.ts`, `sdk/src/generated/Declarations.ts`,
  `sdk/src/generated/Features.ts`, `sdk/src/generated/index.ts` — `Declarations`
  and `Features` both derive `ts_rs::TS`, so the bindings regenerate. The
  entry's fence lists `sdk/src/index.ts` but excludes `sdk/src/generated/**`,
  which is what makes *any* new family unshippable as scoped.
- `tests/seam_bindings_current.rs`, `tests/closed_keys.rs` — the binding-freshness
  and closed-lock-key proofs both enumerate the families.
- `tests/lock_declaration_rows.rs` — the per-family round-trip home.

If a re-scope prefers the cheaper path, the `nested_member`-riding shape ships
with `src/admissibility.rs` + `src/gate.rs` (or `src/coverage_note.rs`) added
instead — but it buys a synthetic kind that three separate surfaces must each
be taught to ignore, and leaves `nested_member` documented as one thing and
carrying two.
