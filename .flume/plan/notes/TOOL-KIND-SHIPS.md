# tool shipped as a root-hosted embedded kind; runner edges and lockfile checks remain

- Home: a `tool` roster member is the root member's embedded child, carried as a
  `nested_member` row with `host = "root"` (address `root/tool/<name>`). The engine admits
  it via `admissibility::ROOT_HOSTED_KINDS`; the SDK twin is `ROOT_HOSTED_KINDS` in
  `builtins.ts`. No new row family.
- Name: optional; defaults to the bin, the file's last segment, or the system name.
  `tool()` with no init is the temper package's bin (`@dtmd/temper`/`temper`).
- Seam gap for the runner entries: SDK `memberAddress(toolMember)` still spells
  `tool:<name>`, while the engine resolves `root/tool/<name>`. Any edge a gate hook, tap,
  typecheck or script verifier points at a tool has to agree on one spelling first.
- Not done here (0104 consequences, other entries): `{ package, bin }` checked against
  `package-lock.json`/`pnpm-lock.yaml`; a `{ file }` tool joining the roster as an input;
  rendering the default as the anchored `.bin` path; lowering the three `binary` fields
  and the typecheck onto tool members.
- Existing behavior kept: a top-level member of any *other* embedded kind is still
  silently carried nowhere (emit.test.ts relies on it). A refusal would be honest but is
  a behavior change no entry names.
- The `kind-tool.md` reference topic now says "embedded in the harness root".
- A test fixture's custom kind named `tool` (locus_template.rs) became `toolkit`.
