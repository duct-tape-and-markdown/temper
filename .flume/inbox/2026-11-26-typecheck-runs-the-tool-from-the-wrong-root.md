# Emit runs a `{ package, bin }` typecheck tool at a doubled path (`.temper/.temper/node_modules/.bin/tsc`)

Found on the dogfood after 7348e253 (EMIT-RUNS-THE-DECLARED-TYPECHECK, continuing). I declared `typecheck: { tool: tool({ package: "typescript", bin: "tsc" }), project: input(import.meta.url, "./tsconfig.json") }` on this repo's harness, with `.temper/node_modules/.bin/tsc` installed and `tsc --noEmit -p .temper` passing by hand. `cargo run -- emit` then refuses with `temper::drift::typecheck_spawn`: "failed to run the program's typecheck `./.temper/node_modules/.bin/tsc` … No such file or directory". The command is spelled relative to the harness root, but it runs from inside the `.temper/` workspace (the commit says "runs from the `.temper/` workspace"), so it resolves to `.temper/.temper/…`. The emit fixture (`emit_refuses_a_program_whose_typecheck_reports_a_type_error`) hides this because of its layout.

Fix: resolve the tool's path and the working directory in one frame. Either run from the workspace with a workspace-relative `node_modules/.bin/<bin>`, or run from the harness root with the harness-relative path. The command reference's anchor is one fact (0086, 0100). Acceptance:
- **new:** a harness at the repo root whose `.temper/` holds the compiler emits with its typecheck declared;
- **kept:** a type error still refuses with the compiler's diagnostics.

This blocks turning on the refusal for an undeclared typecheck, because the dogfood must declare one first.
