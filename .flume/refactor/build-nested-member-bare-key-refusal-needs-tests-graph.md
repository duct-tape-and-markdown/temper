## Surface

`NESTED-MEMBER-DUPLICATE-KEY-ADMISSIBILITY` cannot reach green inside its fence:
one **out-of-fence test pins the exact behaviour the entry overturns**.

- `tests/graph.rs:1188` — `embedded_edge_targets::a_bare_nested_member_address_matches_first_same_key`.
  Two `service` hosts each declare a `domain` member keyed `common`; an edge spells
  the bare `common`, and the test asserts `run.ok` because "bare nested member
  addresses should resolve to the first match (ambiguous but deterministic)".
  The entry's acceptance is that this exact corpus **refuses at resolution naming
  every carrier host**. The test is the codified prior policy — the one
  `graph::member_named`'s doc calls "an open ambiguity … a policy decision owned
  elsewhere" — so deciding the policy necessarily rewrites it (into the refusal, or
  into a host-qualified spelling that still resolves).

The implementation itself fits the fence and was built and run this tick
(`src/admissibility.rs`, `src/graph.rs`, `src/read.rs`, `src/gate.rs`): a
same-host `(kind, key)` repeat refuses at admissibility; `graph::resolve_name`
answers `None`/`One`/`Ambiguous(hosts)` so `resolved_edges`, `route_mentions`,
`read::why_impl` and `gate::embedded_hosts_by_source` share one verdict and one
wording (`graph::ambiguous_nested_key_message`, the SDK's `emit.ts:337-347` bar).
Full `cargo test` over it: **one** other failure, `tests/nested_member.rs`'s own
`host_qualified_addresses::a_bare_key_still_names_an_embedded_member` — in fence,
and part of the entry's own rewrite. (`tests/hook_kind.rs`'s failure in that run
was the already-captured fanout stall, whose hung child I killed by hand.)

## Observed at

13b5bdaa (HEAD when observed).

## Suggested consolidation

Add `tests/graph.rs` to the entry's fence and re-issue it. Nothing structural is
missing — the entry is one file short of buildable, and its own notes already
predicted the shrink ("strictly smaller now: the plumbing is done and only the
judgment is left"). If the fence is meant to stay narrow, the alternative is to
retire that one test into `tests/nested_member.rs`'s `host_qualified_addresses`
module first, as its own entry, since both suites now pin the same grain from two
homes.
