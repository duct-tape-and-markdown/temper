/**
 * The shipped **dial** kind — temper's own, not a provider's.
 *
 * A local-locus TOML document at `.temper/dial.toml` whose entries name a clause by
 * its compiled address (`contract.md`, "clause") and declare the severity this machine
 * reads it at: see the annoying finding, read its label, dial it — locally,
 * uncommitted, and without ever softening the shared gate.
 *
 * This module is **root-exported** (`@dtmd/temper`), never the `./claude-code`
 * subpath: the subpath is the Claude Code provider face, and the dial is a claim about
 * temper's own gate rather than about any harness Claude Code reads. Its facts are
 * therefore not external facts and carry no `cite` — the citation is the corpus
 * (decisions 0030, 0032).
 *
 * **The schema is the envelope** (decision 0030, re-spelled by 0032). Severity is the
 * only verb an entry has: there is no `off`, no `skip`, no `delete`, and the severity
 * vocabulary is the same closed two the authored clause declares under. A dialed clause
 * therefore still reports, and the bound 0030 stated as an admissibility condition over
 * a layer's *effect* is held structurally — by the shape of a kind, which an author
 * cannot spell their way past. The other half of the bound is the engine's:
 * dialed softening is inert in block mode, so a block-mode pass on any machine implies
 * the shared gate's pass (`pipeline.md`, "Layers").
 */

import { clause, closedKeys, enumOf, required, type } from "./contract.js";
import type { Clause, Severity } from "./contract.js";
import { kind } from "./kind.js";
import type { KindDefinition } from "./kind.js";

/**
 * One dial entry: a clause's compiled address, and the severity this machine reads it
 * at. The two fields are the whole verb — the interface is where the envelope is
 * spelled, and there is nothing here to widen it with.
 */
export interface DialEntry {
  /**
   * The clause's compiled address — the label every finding prints as its `rule` id
   * and `explain` narrates (decision 0032). Spelled straight back out of the finding:
   * that round trip is what the label's legibility exists for.
   */
  readonly label: string;
  /** The severity this machine reads that clause at. */
  readonly severity: Severity;
}

/**
 * The dial document's fields — the whole top-level table of `.temper/dial.toml`.
 *
 * `name` is the machine's own label, and it is the member's identity: the file's stem is
 * `dial` on every machine that has one, so identity comes from a declared field.
 */
export interface Dial {
  /** This machine's name — the dial's identity, and what its findings speak. */
  readonly name: string;
  /** The dialed clauses. Absent or empty is a dial that changes nothing. */
  readonly clause?: readonly DialEntry[];
}

/**
 * The `dial` kind: `.temper/dial.toml`, a **local** file locus — the kind is declared,
 * committed and reviewed here; a machine's own dial document is not.
 *
 * Read-side only by construction (decision 0034): `toml-document` is a read face with no
 * write twin, emit's codomain is the committed tree, and a local member's rows never
 * enter the lock. Channel-less: a dial is read by temper's own gate, never surfaced to a
 * model or an installer, so it declares no registration.
 */
export const dial: KindDefinition<Dial> = kind<Dial>({
  name: "dial",
  locus: { kind: "at", root: ".temper", glob: "dial.toml", commitment: "local" },
  format: "toml-document",
  unitShape: "named-field",
  registration: [],
  identityField: "name",
});

/**
 * The default contract for `dial` — the envelope, stated as clauses.
 *
 * Every clause here is `required`: this contract is the one thing on the machine a dial
 * must not be able to soften, since softening it is how a dial would talk its way out of
 * its own shape. The engine refuses a dial entry naming a `dial.*` label for that reason
 * — the clauses below are what an entry is *checked against*, never what it ranges over.
 *
 * `closed-keys` is the load-bearing one: it is what makes a misspelt verb a finding
 * instead of a silently ignored key. A dial that wrote `off = true` beside its entries
 * would otherwise read as an honest dial that happened to do nothing.
 */
export const dialDefaultContract: readonly Clause[] = [
  clause(required("name"), {
    severity: "required",
    guidance:
      "A dial names the machine it speaks for — its identity, and what its findings are reported under. There is no filename fallback: every machine's dial is `dial.toml`.",
  }),
  clause(type("name", ["string"]), { severity: "required" }),
  clause(type("clause", ["list"]), {
    severity: "required",
    guidance:
      "The dialed clauses are an array of tables — `[[clause]]` per entry, each naming one `label` and one `severity`.",
  }),
  clause(required("clause[*].label"), {
    severity: "required",
    guidance:
      "An entry names the clause it dials by the address the finding printed as its `rule` id — spell it back verbatim. An entry naming a label no clause in this harness carries is a finding, never a silent no-op.",
  }),
  clause(type("clause[*].label", ["string"]), { severity: "required" }),
  clause(required("clause[*].severity"), {
    severity: "required",
    guidance:
      "Severity is an entry's only verb: it is what a dial says, and all a dial says.",
  }),
  clause(enumOf("clause[*].severity", ["required", "advisory"]), {
    severity: "required",
    guidance:
      "The two severities an authored clause declares under are the two a dial re-declares under — there is no third value that means `off`. Softening a clause to `advisory` still reports it, and is inert in block mode; hardening one to `required` binds in every mode.",
  }),
  clause(closedKeys(), {
    severity: "required",
    guidance:
      "The dial's key set is exhaustive: `name` and `clause`, nothing else. A key outside it is a verb this schema does not have — most likely an attempt to spell deletion, which it cannot.",
  }),
];
