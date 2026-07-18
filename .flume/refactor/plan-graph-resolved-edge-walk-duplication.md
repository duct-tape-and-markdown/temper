## Surface

`src/graph.rs` walks the whole declared-edge corpus (every admissible edge ×
every source of its `from` kind × every named target) independently at up to
four sites inside one `gate()` invocation (`src/main.rs:1130-1168`):

- `check` (111-134) — its own from-scratch loop over `edges`/`by_kind`,
  producing dangling-route `Diagnostic`s. Always runs (`main.rs:1131`).
- `resolved_edges` (967-994) — the *same* iteration shape (admissible edge →
  sources → targets → `target_identity`/`resolves`), producing
  `Vec<ResolvedEdge>` instead of diagnostics.
- `acyclic` (195-212) calls `resolved_arcs` (1001-1010), which calls
  `resolved_edges` fresh, at 196. Always runs (`main.rs:1147`).
- `degree` (236-311) calls `resolved_arcs` again at 252, only when any
  selection carries a `Degree` clause (opt-in, `main.rs:1154`).
- `mention_reachable` (355-437) calls `resolved_edges` again directly at
  371, only when a selection carries `mention-reachable` (opt-in,
  `main.rs:1162`).

So a `gate()` run performs the identical O(edges × sources × targets) walk
2-4 times over the same immutable `edges`/`by_kind` inputs, depending on
which opt-in clauses are declared. `check`'s copy is a genuine second
implementation of the walk (same loop structure, divergent output type);
the other three at least route through `resolved_edges`/`resolved_arcs` but
still recompute it per call site rather than sharing one materialized
result — the class `specs/process/engineering.md` ("Cost scale is hoisted,
and pinned by count") names for tree walks/glob compiles, applied here to
edge resolution. The module's own doc header (1-13) already claims "one
resolved-edge enumeration... shared" (READ-EDGE-UNIFY) — true of the type,
not of the runtime cost.

## Observed at

e2325df

## Suggested consolidation

Compute `resolved_edges` once in `gate()` and thread the `Vec<ResolvedEdge>`
into `acyclic`/`degree`/`mention_reachable` (replacing their internal
`resolved_arcs`/`resolved_edges` calls with a fold over the passed-in
slice). `check`'s dangling-route diagnostics need the *unresolved* half
`resolved_edges` discards — the real design decision is the shared walk's
output shape: either widen `resolved_edges` to return both the resolved set
and the dangling occurrences in one pass (a `(Vec<ResolvedEdge>,
Vec<Diagnostic>)` or a small struct), or give `check` its own filter over a
richer per-target enumeration `resolved_edges` derives from. Either way,
`check`'s copy of the walk retires in favor of the shared one — worth a
`tests/check_cost.rs`-style count-pin (edge-walk count == 1 per `gate()`)
the way the discovery/lock-parse consolidations already established.
