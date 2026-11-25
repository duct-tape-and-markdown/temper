# LOCUS-DECLARES-EXCLUSIONS parked: the fence leaves out files the seam change must touch

I tried adding the `exclude` column to `drift::KindFactRow` (a `Vec<String>`, read
with `opt_str_array` like `leaves`). `cargo build --all-targets` then fails at the
literal default-filling constructors outside the fence:

- `src/test_support.rs` `kind_fact_row` (the in-src default home)
- `tests/it/common/mod.rs`: `kind_facts`, plus two more literal rows (3 errors)

`KindFactRow` derives no `Default`, so every full literal has to name the new field.
These are the only breakages, because every other site struct-updates off them.

The guard edge needs one more file. `install::GuardedLocus` is filled in
`src/main.rs` `guarded_loci`, which is also outside the fence. The guard can't
honor exclusions unless that function carries the kind's excluded paths onto the
locus, or hands install the `CustomKind`.

To re-fence, add `src/test_support.rs`, `tests/it/common/mod.rs` and `src/main.rs`.
Also check `src/gate.rs` `committed_member_sites` (the locus-declared check half).
It reads `compose::KindUnitsAndFeatures` off import discovery, so subtracting in
`import::discover_kind_units` probably covers it, but I have not confirmed that.

The design I meant to ship: `CustomKind` gains `excluded: Vec<String>` instead of
`Governs`. `Governs` is built literally at ~50 sites across builtin_kind, compose,
admissibility and tests, while `CustomKind` always struct-updates off
`with_locus`. The field is lifted in `from_kind_fact_row` and each entry must be a
starless literal path under the locus (refused at load otherwise).
`compose::overlay` relocation (`governs` delta) keeps it, because the SDK's
`relocatedLocus` spreads `base.locus`.
