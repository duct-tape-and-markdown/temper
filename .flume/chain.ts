/**
 * author's flume chain — plan → build, for a Rust/cargo project.
 *
 * Loaded by the flume CLI from `.flume/chain.ts`; the default export is the
 * Chain. Two phases, no spec phase: the evergreen `specs/` corpus is human-
 * authored, never phase-written. Plan reconciles `pending.json` against the
 * corpus + current `src/` state; build ships entries to the trunk.
 *
 * This chain imports the runtime from the published `@dtmd/flume` package
 * (not `../src/` — that's flume's own dogfood). The gates are the one place
 * this differs materially from flume's TypeScript dogfood chain: the product
 * is Rust, so the validation gates are cargo, not pnpm/tsc/vitest.
 */

import { readFile } from "node:fs/promises";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import type { Agent, Chain, Phase, TickContext, Gate } from "@dtmd/flume";
import {
  claudeCode,
  withSessionCapture,
  withTerminalRenderer,
  shellGate,
  parsePending,
  renderSchemaForPrompt,
} from "@dtmd/flume";

/** Absolute path to this chain.ts directory (.flume/), regardless of cwd. */
const CHAIN_DIR = dirname(fileURLToPath(import.meta.url));

// ---------- project-specific gates ----------

/** pending.json conforms to the schema. Reverts plan's commit on violation. */
const pendingParseGate: Gate = {
  name: "pending.json parses",
  when: "afterCommit",
  async run(ctx) {
    let raw: string;
    try {
      // Read from the resolved state root the dispatcher hands in, not a
      // hardcoded `.flume/`, so the gate is correct under a relocated flumeDir.
      raw = await readFile(join(ctx.flumeDir, "plan", "pending.json"), "utf8");
    } catch {
      return { ok: false, message: "pending.json missing after plan commit" };
    }
    const result = parsePending(raw);
    if (result.ok) {
      return {
        ok: true,
        message: `pending.json parsed (${result.entries.length} entries)`,
      };
    }
    return {
      ok: false,
      message: `pending.json has ${result.errors.length} schema violations`,
      details: result.errors
        .map((e) => `  [${e.index}] ${e.path}: ${e.message}`)
        .join("\n"),
    };
  },
};

/**
 * Rust gate placement (CHAIN-AUTHORING §2): cheap structural at afterCommit,
 * expensive correctness at afterMerge. For Rust the expensive step is
 * *compilation* — `cargo clippy`/`cargo test` compile the crate cold in each
 * fresh worktree. Under fanout, afterCommit gates run N worktrees in parallel,
 * so an N-wide cold compile is exactly the contention trap the docs warn about
 * (a clean commit reverted on a timeout that is really just CPU starvation).
 *
 * So: `cargo fmt --check` is the only afterCommit gate — it touches no deps and
 * does not compile, so it is safe to run N-wide. clippy (with `-D warnings`,
 * which also catches every compile error) and the test suite run afterMerge,
 * serially on the trunk, where they get the cores they need and a failure
 * reverts only the offending entry.
 *
 * No `setupWorktree` hook (unlike flume's pnpm chain): cargo resolves deps from
 * the global registry cache under `~/.cargo`, shared across worktrees for free;
 * only `target/` is per-worktree, and that is the cold compile we keep off the
 * parallel afterCommit path on purpose.
 */
const fmtGate = shellGate({
  name: "cargo fmt",
  when: "afterCommit",
  cmd: "cargo",
  args: ["fmt", "--all", "--check"],
  failHint: "Run `cargo fmt --all` — formatting is the cheap structural gate.",
});

const clippyGate = shellGate({
  name: "cargo clippy",
  when: "afterMerge",
  cmd: "cargo",
  args: ["clippy", "--all-targets", "--", "-D", "warnings"],
  failHint: "clippy denies warnings; fix the lints (this also catches compile errors).",
});

const testGate = shellGate({
  name: "cargo test",
  when: "afterMerge",
  cmd: "cargo",
  args: ["test"],
  failHint: "Tests failed — entry reverted, returns to pending.",
});

/**
 * The self-hosting gate (`specs/00-intent.md` finish line): temper checks its
 * own harness surface (temper.toml + .temper/ + .claude/) after every merge.
 * Advisories report without failing (exit 0); a `required` violation — broken
 * conformance, coverage, or admissibility on temper's own house — reverts the
 * entry. From here on, every build entry is gated by the product it builds.
 */
const selfCheckGate = shellGate({
  name: "temper check (self)",
  when: "afterMerge",
  cmd: "cargo",
  args: ["run", "--quiet", "--", "check"],
  failHint:
    "temper's own surface went red — fix the code or the harness, never bypass the self-check.",
});

// ---------- phases ----------

/**
 * Build's writable fence, extracted so the entry-fence preflight (below) and the
 * build phase share one declaration — a fence with two copies would drift.
 */
const BUILD_WRITABLE_PATHS = [
  // Rust source, tests, benches, examples, build script
  "src/**",
  "tests/**",
  "benches/**",
  "examples/**",
  "build.rs",

  // Cargo + toolchain config
  "Cargo.toml",
  "Cargo.lock",
  "rustfmt.toml",
  "clippy.toml",
  "rust-toolchain.toml",

  // Root docs + dotfiles
  "README.md",
  "CHANGELOG.md",
  "AGENTS.md",
  "LICENSE",
  "LICENSE.*",
  "LICENSE-*",
  ".gitignore",
  ".editorconfig",

  // CI
  ".github/**",

  // Vendored distribution surface — the plugin temper publishes (skill, hooks,
  // manifest). A generated surface administered via spec, built here (and later
  // by `temper bundle`), NOT hand-curated like the territories below.
  "plugin/**",

  // NOTE: build does NOT touch .flume/** (harness territory), .claude/** or
  // CLAUDE.md (the hand-curated CC harness — also a dogfood fixture, edited by
  // humans), specs/** (the evergreen human-authored corpus), packages/** or
  // kinds/** (the curated std-lib sources — built-in package and kind
  // definitions, citation-bearing product territory the build EMBEDS but never
  // writes). If a build entry needs to change one, block it and surface the
  // question. The harness writes the post-merge ship commit to pending.json itself.
];

/**
 * Entry-fence preflight. Twice this week plan filed root-doc work whose declared
 * file paths fell outside build's writable globs (LICENSE-MIT, AGENTS.md): build
 * correctly bailed, but only after burning a session discovering statically
 * checkable facts. An entry declares its paths; the fence is declared globs —
 * whether the work fits its fence is decidable at plan time, so decide it here.
 * Fails plan's commit with the offending paths; the human either widens the
 * fence (chain.ts is human territory) or plan re-scopes the entry.
 */
const globToRegex = (glob: string): RegExp => {
  const escaped = glob
    .replace(/[.+?^${}()|[\]\\]/g, "\\$&")
    .replace(/\*\*/g, "<<GLOBSTAR>>")
    .replace(/\*/g, "[^/]*")
    .replace(/<<GLOBSTAR>>/g, ".*");
  return new RegExp(`^${escaped}$`);
};

const entryFenceGate: Gate = {
  name: "entry paths fit build's fence",
  when: "afterCommit",
  async run(ctx) {
    let raw: string;
    try {
      raw = await readFile(join(ctx.flumeDir, "plan", "pending.json"), "utf8");
    } catch {
      return { ok: true, message: "no pending.json to fence-check" };
    }
    const result = parsePending(raw);
    if (!result.ok) return { ok: true, message: "parse gate owns malformed pending" };
    const fence = BUILD_WRITABLE_PATHS.map(globToRegex);
    const offending: string[] = [];
    for (const entry of result.entries) {
      const gate = (entry as { gate?: { kind?: string } }).gate?.kind;
      if (gate !== "open" && gate !== "blockedBy") continue; // parked/deferred entries may be re-scoped before they open
      const files = (entry as {
        files?: { new?: { path: string }[]; edit?: { path: string }[]; retire?: string[] };
      }).files;
      // `retire` entries are bare path strings; a deletion is a write too.
      const paths = [
        ...[...(files?.new ?? []), ...(files?.edit ?? [])].map((f) => f.path),
        ...(files?.retire ?? []),
      ];
      for (const path of paths) {
        if (!fence.some((re) => re.test(path))) {
          offending.push(`  [${(entry as { tag: string }).tag}] ${path}`);
        }
      }
    }
    if (offending.length === 0) {
      return { ok: true, message: "every pickable entry's paths fit build's fence" };
    }
    return {
      ok: false,
      message: `${offending.length} declared path(s) fall outside build's writablePaths — widen the fence (human, chain.ts) or re-scope the entry`,
      details: offending.join("\n"),
    };
  },
};

const plan: Phase = {
  name: "plan",
  description:
    "Reconcile .flume/plan/{pending.json,state.md,open-questions.md} against specs/ + current src state; drain .flume/inbox.md.",
  promptPath: "prompts/plan.md",
  concurrency: "singleton",
  writablePaths: [
    ".flume/plan/pending.json",
    ".flume/plan/state.md",
    ".flume/plan/open-questions.md",
    ".flume/inbox.md",
    // Plan does NOT touch specs/ (human-authored) or src/ (build's territory).
  ],
  gates: [pendingParseGate, entryFenceGate],
  promptArgs() {
    return { PENDING_SCHEMA: renderSchemaForPrompt() };
  },
  handoff(result) {
    // Plan re-wakes itself when state.md ends with `Plan continues: yes`
    // (PROTOCOL.md "Plan continuation marker"). Otherwise hand to build if
    // anything is pickable, else hibernate.
    let planContinues = false;
    try {
      const stateText = readFileSync(
        resolve(CHAIN_DIR, "plan", "state.md"),
        "utf8",
      );
      planContinues = /^Plan continues:\s*yes\b/im.test(stateText);
    } catch {
      // state.md missing — treat as stable.
    }
    if (planContinues) return ["plan"];
    const hasPickable = result.pendingAfter.some((e) => e.gate.kind === "open");
    return hasPickable ? ["build"] : [];
  },
};

const build: Phase = {
  name: "build",
  description: "Ship one (or N disjoint) pending entries to the trunk.",
  promptPath: "prompts/build.md",
  concurrency: "fanout",
  // One declaration, shared with the entry-fence preflight gate (above).
  writablePaths: BUILD_WRITABLE_PATHS,
  gates: [fmtGate, clippyGate, testGate, selfCheckGate],
  promptArgs(ctx: TickContext) {
    if (!ctx.assignedEntry) {
      throw new Error("build phase requires an assignedEntry");
    }
    return {
      ENTRY_JSON: JSON.stringify(ctx.assignedEntry, null, 2),
      TAG: ctx.assignedEntry.tag,
      PER_PATH: ctx.assignedEntry.per.path,
      PER_SECTION: ctx.assignedEntry.per.section,
    };
  },
  handoff(result) {
    // Wake plan only when the wave produced signal to audit (shipped commits
    // or gate fires). A true no-op wave hibernates; `flume wake plan` forces it.
    if (result.shippedTags.length === 0 && result.gateResults.length === 0) {
      return [];
    }
    return ["plan"];
  },
};

const temperChain: Chain = {
  phases: [plan, build],
  humanOnly: [], // no spec phase; the specs/ corpus is authored in-session, never by a phase
};

export default temperChain;

/**
 * Foundations governor (CHAIN-AUTHORING §6). A pending entry may declare
 * `dependsOnForks: ["slug"]`; the dispatcher skips it while any slug is
 * unresolved. Open questions live in `.flume/plan/open-questions.md`, keyed as
 * `(slug)`; an entry's foundation is "settled" when its line reads `RESOLVED`.
 *
 * Fail OPEN, never closed: an absent or mistyped slug is treated as resolved, so
 * a bookkeeping error can never permanently wedge the loop. Every degradation is
 * a missed block (a surface built one tick early), never a stuck loop.
 */
export const forkResolver = (repoRoot: string) => {
  let text = "";
  try {
    text = readFileSync(
      join(repoRoot, ".flume", "plan", "open-questions.md"),
      "utf8",
    );
  } catch {
    return () => true; // no open-questions file → nothing is blocked
  }
  return (slug: string) => {
    const esc = slug.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const re = new RegExp(`\\(${esc}(?![-A-Za-z0-9])`);
    const line = text.split("\n").find((l) => re.test(l));
    return !line || /\bRESOLVED\b/.test(line);
  };
};

/**
 * Per-tick session capture + condensed terminal output. Sessions are rooted at
 * FLUME_DIR (the relocatable state root) so the whole footprint tears down with
 * one `rm`; the `?? CHAIN_DIR` fallback is defensive only.
 */
const makeAgent = (model: string) =>
  withTerminalRenderer(
    withSessionCapture(
      claudeCode({
        outputFormat: "stream-json",
        extraArgs: [
          "--exclude-dynamic-system-prompt-sections",
          "--model",
          model,
        ],
      }),
      {
        dir: resolve(process.env.FLUME_DIR ?? CHAIN_DIR, "sessions"),
        filename: (inv) => {
          const ts = new Date().toISOString().replace(/[:.]/g, "-");
          const cwdName = inv.cwd.split("/").pop() ?? "tick";
          return `${ts}-${cwdName}.jsonl`;
        },
      },
    ),
  );

/**
 * Model routing: plan reconciles the queue against the corpus (judgment) and
 * runs on Opus; build executes one clearly-dictated entry under the cargo
 * gates and runs on Sonnet. The runtime exposes one shared `agent` export for
 * every phase, so route on the rendered prompt's first heading — `# ASSIGNED
 * ENTRY` is build's (prompts/build.md); everything else is plan's.
 */
const planAgent = makeAgent("claude-opus-4-8");
// Build rides Opus for the declared-adapter wave (format-declaration swap of
// the import/apply faces — equivalence-sensitive, human call 2026-07-02).
// Revert to claude-sonnet-5 when the wave drains.
const buildAgent = makeAgent("claude-opus-4-8");

export const agent: Agent = {
  name: "phase-model-router",
  invoke: (inv) =>
    (inv.prompt.startsWith("# ASSIGNED ENTRY") ? buildAgent : planAgent).invoke(
      inv,
    ),
};
