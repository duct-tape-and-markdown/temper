# Does the path-resolved edge set share the run's fenced tree index, or keep its own raw-disk walk?

`compose::repo_file_set` is a bare `walkdir` over the whole tree. It reads no `.gitignore` and takes none of discovery's fences (`.git/`, nested governed root). Over this repo, `check` opens 642 entries under the ignored `target/`. Its raw-disk reading is deliberate: `specs/model/contract.md`, "edge" says "Path-resolved edges resolve against raw disk, never the ignore-filtered discovery view: an extra file can only suppress a finding, a pruned one can forge one". The unit test `repo_file_set_stays_raw_disk_including_gitignored_targets` holds this.

The harness rule `.claude/rules/rust.md`, "A run reads each input once, through one owner", which the sweep now applies, says the tree index is built in one place behind the ignore rules, `.git/` and nested roots. It also says a kind that claims ignored documents reads them "never by a whole-tree walk with the rules waived". The inbox record asks for one tree index per run that all three fences hold. Read literally, that drops gitignored `@import` backing targets, which the contract section says must stay visible.

Two answers fit. (a) The edge set keeps raw disk, the rule's one-index clause excepts it, and the cost is accepted or bounded some other way (for example, resolving per target by stat, as `backed_on_disk` already does). (b) contract.md narrows "raw disk", for example by fencing `.git/` and nested roots while still reading ignored files, so one index can serve both.

Needed: which of the two, stated in contract.md "edge" or in the rule. ONE-FENCED-TREE-INDEX-PER-RUN leaves `repo_file_set` out until this is settled. Raised by inbox 2026-11-27-whole-input-work-repeats-per-kind-and-per-door.
