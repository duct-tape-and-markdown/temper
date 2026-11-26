# Install places the schema modeline on an unrepresented harness

`specs/distribution.md`, "Keystroke": "Unrepresented, `temper schema` generates a JSON Schema from the compiled clauses, wired via the `yaml-language-server` modeline `install` places." Install places no modeline on an unrepresented project (`src/install.rs:1150-1153`; `tests/it/install.rs::declining_wires_the_session_start_reporter_alone_and_never_creates_temper_dir`). This is intent the code does not meet (0105). Install's no answer still wires the session-start reporter alone, and also places the modeline on each governed YAML-frontmatter document. Each modeline points at the built-in lock's schema. Acceptance:
- **new:** declining representation leaves a modeline on `.claude/skills/x/SKILL.md`;
- **kept:** declining still creates no `.temper/`.
