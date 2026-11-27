# The raw-disk edge walk reads the whole tree to resolve a few globs

Answers question repo-file-set-raw-disk-vs-one-fenced-index. `specs/model/contract.md` ("edge") keeps path-resolved edges on raw disk, and the rust rule now defers to it: a raw-disk reader keeps raw disk and reads only as far as the paths it resolves reach. `compose::repo_file_set` walks the whole tree for the `paths-match` channel, ignored `target/` and `.git/` included: 642 entries under `target/` per `check` on this repo. The globs it resolves (a rule's `paths`) mostly carry a literal prefix, such as `src/**/*.rs`.

Acceptance:
- **new:** over a generated fixture with a large ignored tree outside every resolved glob's prefix, a `check` opens nothing under it;
- **kept:** `repo_file_set_stays_raw_disk_including_gitignored_targets` holds: a gitignored file inside a resolved glob still counts.
