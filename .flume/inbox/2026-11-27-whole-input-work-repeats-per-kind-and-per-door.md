# Whole-input work repeats per kind and per door, past every count pin

Found measuring the Rust-or-TS question (ledger), with strace and gdb on 0.0.26, this repo, and generated trees. `specs/process/engineering.md` ("Cost scale is hoisted") states each fact below. The pins count one counter per mechanism, so the run's total goes unpinned.

- **Per-kind index scans:** `import::scan_locus` runs 60 times per `check`, and `collect_glob` calls `glob::compile_glob` per child, whose memo hands back a clone that rebuilds its regex cache. At 100k files this is 36 of 40 stack samples (`check` 2.4 s; the walk is 4).
- **Four tree indexes:** `Discovery` (two flavors), `compose::repo_file_set`, `coverage_note::claude_walk`, and `install::locus_files`. `repo_file_set` is a bare `walkdir`: it reads no `.gitignore` (adoption.md: an ignored file is not authored here) and skips the `.git/` and nested-root fences every walk holds. `check` opens 642 entries under the ignored `target/`.
- **A walk per locus:** `install::locus_member_sites` walks from the root 11 times per shell-edge `guard`.
- **The lock read twice:** guard at both edges (`read_committed_declarations` plus `emit_owned_targets` or `shell_edge_findings`) and `explain`. Only `gate()` and `emit` are pinned.
- **Each member read three times per `check`:** `install::evaluate_placements`, `drift::place`, and `drift::config_stale_members_from_doc`.

Acceptance (all **new**), each a count over `check`, `guard` at both edges, and `explain`:
- one tree index per run, which `.gitignore`, `.git/` and nested roots all fence;
- each index scanned once, and one matcher built per distinct glob;
- each file opened at most once per run.
