# What posture does a committed locus that is read but never projected take?

Driver: SHIPPED-EXAMPLE-PASSES-ITS-OWN-CHECK's ship note. The shipped example's `src/*.js` sat at the `source` locus with no member, so `root.locus-declared` fired. The remedy that finding names is `commitment: "local"`, and the example now declares it. `specs/model/representation.md`, "locus", defines local as "per-machine and uncommitted", but the example's `src/` is committed. The example now teaches adopters a posture that contradicts its definition.

The corpus has two commitment classes. A projected file is an emit target. A local file is per-machine. Committed ground that the harness reads, as a mention target or a source, but never projects has neither.

Options: (1) a third commitment class for committed, read-only ground: read at check, never an emit target, committed. (2) Widen `local` to mean "not an emit target" and drop "uncommitted" from its definition. (3) Such ground stays undeclared as a locus, and the example references it another way.

Recommendation: 1. "Local" carries a real per-machine fact (the guard and discovery honor it), and blurring it loses that. Sections: `specs/model/representation.md` "locus"; `specs/model/adoption.md`.
