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

import type { Chain, Phase, TickContext, Gate } from "@dtmd/flume";
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

// ---------- phases ----------

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
  gates: [pendingParseGate],
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
  writablePaths: [
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
    "LICENSE",
    "LICENSE.*",
    ".gitignore",
    ".editorconfig",

    // CI
    ".github/**",

    // NOTE: build does NOT touch .flume/** (harness territory), .claude/** or
    // CLAUDE.md (the hand-curated CC harness — also a dogfood fixture, edited by
    // humans), specs/** (the evergreen human-authored corpus), or contracts/**
    // (the curated packaged guidance — the tool's sourced opinions; build EMBEDS
    // these, e.g. via include_str!, but never writes them). Three curated
    // territories. If a build entry needs to change one, block it and surface the
    // question. The harness writes the post-merge ship commit to pending.json itself.
  ],
  gates: [fmtGate, clippyGate, testGate],
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

const authorChain: Chain = {
  phases: [plan, build],
  humanOnly: [], // no spec phase; the specs/ corpus is authored in-session, never by a phase
};

export default authorChain;

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
export const agent = withTerminalRenderer(
  withSessionCapture(
    claudeCode({
      outputFormat: "stream-json",
      extraArgs: ["--exclude-dynamic-system-prompt-sections"],
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
