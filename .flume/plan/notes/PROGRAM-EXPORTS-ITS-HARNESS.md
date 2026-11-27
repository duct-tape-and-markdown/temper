# PROGRAM-EXPORTS-ITS-HARNESS

Shipped. The engine runs `node --import <recorder> --input-type=module --eval
<driver> <entry>`; the driver imports `@dtmd/temper/pipe` (new package
subpath, `sdk/src/pipe.ts`) which imports the entry, refuses unless its
default export is a composed harness, and prints `emit(harness).seam`.
`emit`, `EmitResult`, `RegistrationFact`, `SettingsResidue` left the root
(`index.ts`); they live on `./pipe`. SDK tests import `emit` from
`../src/emit.js`.

Human halves owed (outside build's fence):
- `.temper/harness.ts`: drop `emit` from the root import and replace the
  `process.stdout.write(emit(program).seam);` tail with
  `export default program;` — the dogfood emit fails until this lands.
- `docs/proposals/posture-recursion/harness.ts`: same tail.
- Migration-note line (release cut): programs end with
  `export default harness(...)`; `emit` is no longer importable from
  `@dtmd/temper`.

Fixture programs moved (tail -> `export default`, `emit` import dropped):
every SDK fixture in tests/it/{builtin_lock_frozen,bundle,coverage,
coverage_note,emit,gauntlet,hook_kind,install,lock_declaration_rows,
mcp_kind,mcp_server_kind,module_closure,nested_member,projection_path_seam,
read_verbs,requirement_roster,settings_kind,tap,tool_kind,type_predicate}.rs;
`declare const process` shims removed (emit, tap, tool_kind). The raw seam
writer NOISY_STDOUT_PROGRAM became a console.log + default-export program.
Regenerated: example lock (harness.ts hash), gauntlet snapshot (same),
`sdk/src/generated/Payload.ts` (doc comment).

`@types/node` stays a scaffolded devDependency: the scaffold's tsconfig names
`"types": ["node"]`; its note now says so instead of citing stdout.

Observed: a program's own stdout writes still corrupt the seam (PayloadParse).
The pipe could route the seam off stdout or redirect console.log to stderr;
not decided by the spec, left as-is.
