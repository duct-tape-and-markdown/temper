# The dangling-edge, requirement-fill, dangling-requirement and unbacked-import checks become clauses

Intent ("The engine ships no baked judgment"): "every check whose severity anyone could want to dial is a clause … The only fixed checks are well-formedness." `contract.md`, "well-formedness", names those two fixed checks: admissibility and acyclicity. The 2026-11-26 census found four more hard-wired checks:
- `graph.route` for a dangling edge (`src/graph.rs:83`, `:2383`);
- `requirement.unfilled` (`src/coverage.rs:79-80`), the "must be filled" default, which `contract.md` calls a whole-grain cardinality clause at error severity;
- `requirement.dangling` (`src/coverage.rs:103`);
- the unbacked-import check (`src/graph.rs:1694`).

None has a clause label, none can be dialed, and none carries guidance. Each becomes a shipped clause in its kind's or the root's default contract, with a label, guidance, a cite and its present severity as the default. The spec's *(pending)* marks on these claims come off at the cut that ships this. Acceptance:
- **new:** each of the four findings carries a clause label;
- **new:** a dial over that label lowers its read severity, and it still reports;
- **new:** `required: false` on a requirement makes the fill clause advisory instead of silent;
- **kept:** every one fires at its present severity by default.
