## Surface

`src/main.rs:1141` (inside `#[cfg(test)] mod tests`,
`repo_file_set_stays_raw_disk_including_gitignored_targets`) builds its own
one-off temp dir: `std::env::temp_dir().join(format!("temper-backing-{}",
std::process::id()))`, then `fs::remove_dir_all` + `fs::create_dir_all`.
Same job TEST-SCAFFOLDING-CONSOLIDATE just gave every other in-src
`#[cfg(test)]` module a `tempfile`-backed `tmpdir(label)` for — this is a
lone straggler TEST-SCAFFOLDING-CONSOLIDATE's file list didn't name (it
carries no `label` param and no counter, so it didn't match that entry's
literal "counter+pid+label" description; found re-grepping `std::process::id()`
across `src/`/`tests/` while verifying that entry's acceptance bar, out of
scope for it since main.rs wasn't in its `files.edit`).

## Observed at

96e947d

## Suggested consolidation

Switch to `tempfile::Builder::new().prefix("temper-backing").tempdir().expect(..).keep()`,
matching the sibling in-src conversions TEST-SCAFFOLDING-CONSOLIDATE shipped.
