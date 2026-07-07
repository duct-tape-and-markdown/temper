/**
 * temper's authoring face — the six-noun core as a typed module library.
 * A harness author imports plain nouns — `harness()`, the generic `kind`/`genre`
 * constructors, the clause and requirement constructors, `needs`, and the three
 * prose constructors — and composes members as typed values. `emit` compiles the
 * whole into the declaration rows and the projected members' erased payload —
 * the JSON pipe printed to stdout; the engine is the sole compiler of every
 * projection and the whole lock.
 * Every type erases at the seam, and Turing-completeness
 * stays quarantined at authoring time.
 *
 * The first-party Claude Code provider face — the built-in `skill`/`rule`/
 * `memory` kinds — lives at the `./claude-code` subpath, never here.
 */

// Prose — three constructors, one field type.
export type { Blocks, File, Mention, Mentionable, Prose, Text } from "./prose.js";
export { blocks, file, renderText, text } from "./prose.js";

// Genre values — posture-3 composed values for `blocks()`.
export type { GenreValue } from "./genres.js";
export { genreValue } from "./genres.js";

// Needs — the derived permission union's source.
export type { Capability } from "./needs.js";
export { bash, capability, permissionUnion } from "./needs.js";

// Contracts — clauses, predicates, requirements.
export type { Charset, Clause, Predicate, Requirement, Severity } from "./contract.js";
export {
  allowedChars,
  clause,
  count,
  degree,
  deny,
  forbiddenKeys,
  maxLen,
  maxLines,
  membership,
  minLen,
  nameMatchesDir,
  required,
  requireSections,
  requirement,
  type,
  unique,
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

// The assembly — `harness()` and its five fields.
export type { EnforcementMode, ExpectBinding, Harness } from "./assembly.js";
export { harness } from "./assembly.js";

// Declaration rows — the erased program the seam carries.
export type {
  AssemblyFactRow,
  ClauseRow,
  Declarations,
  KindFactRow,
  RequirementRow,
  SatisfiesRow,
} from "./declarations.js";
export { SEAM_VERSION, compileDeclarations, declarationsToJson } from "./declarations.js";

// Emit — the compile to the seam's JSON pipe; the engine is the sole compiler
// of every projection and the whole lock.
export type { EmitOptions, EmitResult, PayloadMember, ResolveOptions } from "./emit.js";
export { emit } from "./emit.js";
