# Module closure drops what the repository ignores

Shipped: `import::repository_keeps(real_path, harness_root)` judges a loaded module against the git ignore declarations discovery honours; `drift::resolve_modules` skips modules it rejects. The per-prefix matcher moved out of `discovers_path` into a shared `admitted(root, relative, git_declarations, fence)`, so there is still one gitignore judgment.

The judgment's root is the module's own repository: the nearest ancestor with a `.git` (dir or worktree pointer file), falling back to the harness root when no ancestor has one. A module in another repository is judged by that repository's rules. A module in no repository and outside the harness keeps its row, because no rules apply to it. The entry did not settle this. If "the repository" means the harness's repository only, out-of-repo modules would lose their rows too, and that needs a spec ruling.

Side effect in the suites: the vendored test SDK (`sdk/dist`, gitignored in this checkout) now gets no row, so `without_module_rows(.., is_vendored_sdk)` in gauntlet/emit/builtin_lock_frozen probably strips nothing now. It is debt and could likely be retired. It stays for checkouts without `.git` (e.g. tarballs).

Human half: re-emit the dogfood lock (`.temper/lock.toml`), whose `sdk/dist` rows should disappear.
