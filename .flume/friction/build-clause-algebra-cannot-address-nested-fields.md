## Symptom

MARKETPLACE-KIND's entry scoped five acceptance asserts for
`tests/marketplace_kind.rs`. **Three are not expressible in the clause algebra**,
and the entry did not anticipate it — it named only the impersonation rule as a
non-clause (correctly: that one is undecidable).

A clause addresses a field by **top-level key**: every `Predicate` variant carries
a `field: String`, resolved by `Features::field` → `fields.get(name)` — a flat
`BTreeMap` lookup, no path syntax. And `crate::extract::json_to_feature`
(`src/extract.rs:916-926`) flattens on the way in:

- `JsonValue::Object(_) => FeatureValue::Map` — inner keys **discarded**.
- `JsonValue::Array(items) => FeatureValue::List(…json_scalar_string)` — each
  element **stringified**, so structure is gone.

So for `marketplace.json` these documented, decidable rules have no clause:

- `owner.name` required — `owner` is an opaque `Map`.
- each `plugins[]` entry requires `name` + `source` — the array's elements are
  strings by the time a clause sees them.
- the `source` union (relative path's leading `./`; the four object forms' `source`
  discriminator and required fields) — needs both of the above.

Shipped the decidable slice — `required("owner")`, `required("plugins")` — with the
hold named in `marketplaceDefaultContract`'s header and pinned by a
characterization test (`the_rules_below_the_top_level_are_not_gateable_today`)
that flips when the vocabulary lands. This is the honest encoding, and matches the
`pluginManifestDefaultContract` / `skillDefaultContract` precedent — but note this
is now the **third** shipped contract naming a "pending a vocabulary addition"
hold, and the first where the gap swallows most of the format's contract rather
than an edge of it. A `marketplace.json` that Claude Code refuses outright (no
`owner.name`, a `plugins[]` entry with no `source`) passes `temper check` clean.

The JSON-document kinds are what surfaced this: a frontmatter member's fields are
mostly scalars, so flat addressing was nearly free. A manifest's are objects and
arrays, and 0031's producer half is all manifests.

## Cost this tick

~25 minutes: reading the predicate set, `scalar`/`Features::field`, and
`json_to_feature` to establish the bound was real rather than a surface I hadn't
found (`rg` first, per the non-negotiable), then re-deciding the entry's
acceptance and rewriting three planned asserts into one characterization test.
Nothing reverted; all gates green.

## Suggested fix

Inbox item — product work, and a decision before an entry. Two halves, and the
second is the harder one:

1. **Addressing.** A field path (`owner.name`, `plugins[].source`) — the `field`
   column is a `String` in both the lock row and the SDK, so a path spelling may
   round-trip without a schema change. Worth checking before assuming a delta.
2. **Extraction.** `json_to_feature` must stop flattening — `FeatureValue` needs a
   nested arm (`Map` carrying its entries, `List` carrying values not strings).
   That touches the `type` predicate's kind-preservation contract and every
   consumer of `FeatureValue`, so it is not a corner of an entry.

Worth ruling on whether the union-shaped `source` check (a discriminated union: the
`source` value selects which fields are required) is in scope for the vocabulary at
all, or whether that one stays a type-level-only bar permanently. It is decidable,
so "undecidable, deliberately absent" is not the honest name for it — today it is
just unexpressible, and those two should not be conflated in the corpus.
