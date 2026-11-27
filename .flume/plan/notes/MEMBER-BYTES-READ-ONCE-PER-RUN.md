# Member bytes read once per run

Shipped. Measured first on this repo's own harness: check opened each
member 4 times (compose member read, evaluate_placements, drift::place,
config_stale) and settings.json 5. Guard was already clean at both edges.
Explain opened settings.json and the `.git` pointer twice.

- New `bytes` module. `bytes::open` is the counted door. All production
  `fs::read*` sites now go through it, so `bytes::paths_opened` is the
  per-verb pin. `Bytes` is the owner: it fills on first read and holds
  writes. `Discovery` carries one, read through `disc.bytes()`, so every
  reader that already holds the index shares it.
- Retired `json_manifest::manifest_read_count`, a counter on one
  mechanism. Its pin now reads paths opened.
- Signatures that changed: `drift::place`,
  `config_stale_members_from_doc`, `install::gate_installed`,
  `compose::hosted_member_address` (now takes `disc`), `tap::read_log`,
  and the crate-private `git_dirs`, `ignore_walk`, `discovers_path`.
- Owners outside any run: emit's `read_layout_document`,
  `config_stale_from_doc`, `tap::append`, `repository_keeps`, and the
  install verb's pre-emit and post-emit placement reads each make a
  fresh `Bytes`. Install has to: emit rewrites targets behind any earlier
  read. Emit's own read paths are not pinned for paths opened. That is a
  candidate entry if emit cost matters.
- Out of scope: the `ignore` crate reads `.gitignore` files itself. Those
  bypass the door and are not counted.
