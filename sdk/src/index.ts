/**
 * temper's authoring face — the six-noun model as a typed module library
 * (`specs/intent/00-intent.md`, the SDK Decision; `specs/architecture/20-surface.md`).
 * A harness author imports plain nouns — the built-in kinds, `harness()`, the
 * clause and requirement constructors, `needs`, and the three prose constructors —
 * and composes members as typed values. `emit` compiles the whole into the
 * declaration rows the engine reads, a byte-faithful projection, and the lock;
 * every type erases at the seam, and Turing-completeness stays quarantined at
 * authoring time.
 */

// Prose — three constructors, one field type.
export type { Blocks, File, Mention, Mentionable, Prose, Text } from "./prose.js";
export { blocks, file, renderText, text } from "./prose.js";

// Genre values — posture-3 composed values for `blocks()`.
export type { Alternative, GenreValue } from "./genres.js";
export { bound, decision, genreValue, law } from "./genres.js";

// Needs — the derived permission union's source.
export type { Capability } from "./needs.js";
export { bash, capability, permissionUnion } from "./needs.js";

// Contracts — clauses, predicates, requirements.
export type { Clause, Predicate, Requirement, Severity } from "./contract.js";
export {
  allowedChars,
  clause,
  forbiddenKeys,
  maxLen,
  maxLines,
  minLen,
  nameMatchesDir,
  required,
  requireSections,
  requirement,
  type,
} from "./contract.js";

// The engine room — kinds and genres as typed constructors.
export type {
  EdgeField,
  Format,
  KindDefinition,
  KindFacts,
  Locus,
  Member,
  MemberInit,
  Registration,
  UnitShape,
} from "./kind.js";
export { genre, kind } from "./kind.js";

// The face nouns — the built-in Claude Code kinds.
export type { Memory, Rule, Skill } from "./builtins.js";
export { memory, rule, skill } from "./builtins.js";

// The assembly — `harness()` and its five fields.
export type { ExpectBinding, Harness } from "./assembly.js";
export { harness } from "./assembly.js";

// Declaration rows — the erased program the seam carries.
export type {
  AssemblyFactRow,
  ClauseRow,
  Declarations,
  KindFactRow,
  RequirementRow,
} from "./declarations.js";
export { SEAM_VERSION, compileDeclarations, declarationsToJson } from "./declarations.js";

// Projection — the byte-faithful `.claude/**` write.
export type { Projection, ProjectionInput, ProjectOptions } from "./project.js";
export { placementLines, projectBytes, projectMember, projectionPath, renderField } from "./project.js";

// The lock — rollup rows plus declaration rows.
export type { LockRow } from "./lock.js";
export { lockRow, sha256Hex, stampLock } from "./lock.js";

// Emit — the compile to the committed seam.
export type { EmitOptions, EmitResult, ResolveOptions } from "./emit.js";
export { emit, writeEmit } from "./emit.js";
