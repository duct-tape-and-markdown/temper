# Seated hook as unrepresented install evidence: two readings taken

`gate_installed`'s program-less branch (`evaluate_unrepresented`, src/install.rs) made two calls that the spec sentence leaves open. Neither changes the acceptance, but a spec pass may want to pin them:

- "Seated" is any SessionStart group whose command contains `check . --reporter session-start`, so it covers every spelling and not only the current rendering. A retired spelling (the unanchored command) counts as evidence and reports *stale*. If only the exact rendering counted, the drift would read as "never installed" and pass.
- "The modelines it placed" means a frontmatter modeline whose `$schema` starts with the temper CDN root (`placement::cdn_schema_root`), at any version. A governed file added after the install, with no modeline, is not judged. Re-running install would still place one.

A settings.json that fails to parse is treated as no evidence (silent). The represented branch surfaces it as "could not be evaluated" instead. On a program-less tree that report would claim a gate nobody showed was installed.

The unrepresented gate-hook clause now says `stale`, never `missing`. Missing is unreachable there by construction.
