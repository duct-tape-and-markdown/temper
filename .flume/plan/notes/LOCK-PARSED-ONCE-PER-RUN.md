# guard and explain parse the lock once per run

Shipped. Observations for plan:

- The guard's orchestration lived inline in `src/main.rs`, out of reach of a
  thread-local count pin. It now lives in `install::run_guard` (the lock's one
  owner for the verb), and `guarded_loci` / `guarded_manifests` /
  `container_kinds` / `manifest_path` moved from main.rs into install.rs with it.
  main.rs keeps only stdin decode and printing.
- `read::explain_target` now takes the harness root (main passes `.`) so a test
  can drive it without chdir. Its cwd-relative `Path::new(".")` sites read that root.
- `compose::read_committed_declarations` and `drift::emit_owned_targets(&Path)`
  survive with one remaining caller each (main.rs emit arm; one install test).
  Either may be a candidate for retirement once those callers hold a doc.
- Ordering change: guard now decodes the stdin payload before reading the lock,
  so bad stdin plus a corrupt lock reports the stdin error first.
