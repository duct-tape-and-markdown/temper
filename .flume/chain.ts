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

import { execFileSync } from "node:child_process";
import { readFile, readdir } from "node:fs/promises";
import { existsSync, readFileSync } from "node:fs";
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
 * Marker honesty (the dispatch model's one decidable lie). Plan self-schedules:
 * one job per tick off its inputs, with `Plan continues: yes|no` driving the
 * re-wake. A tick that declares `no` while an input is plainly live — an
 * undrained inbox, a spec cursor trailing specs/ HEAD — would silently strand
 * work, and both conditions are statically checkable, so check them here
 * (same pattern as the entry-fence preflight). The cursor claims ROUTED-ness
 * (every slice derived into entries or registered as a keyed open fork), so
 * fork-parked spec content never pins the marker. Fail OPEN on bookkeeping
 * errors (missing files, unparseable cursor): a degradation is a missed
 * catch, never a wedged loop.
 */
const planHonestyGate: Gate = {
  name: "continuation marker is honest",
  when: "afterCommit",
  async run(ctx) {
    let stateText: string;
    try {
      stateText = await readFile(join(ctx.flumeDir, "plan", "state.md"), "utf8");
    } catch {
      return { ok: true, message: "no state.md to check" };
    }
    if (!/^Plan continues:\s*no\b/im.test(stateText)) {
      return { ok: true, message: "marker is yes/absent — re-wake handles it" };
    }
    // Marker says quiet. Live input 1: an undrained inbox.
    try {
      const inbox = await readFile(join(ctx.flumeDir, "inbox.md"), "utf8");
      const stripped = inbox.replace(/<!--[\s\S]*?-->/g, "").trim();
      if (stripped.length > 0) {
        return {
          ok: false,
          message: "state.md says `Plan continues: no` but .flume/inbox.md is undrained",
        };
      }
    } catch {
      // no inbox file — nothing undrained
    }
    // Live input 2: undrained refactor captures (plan-drained, unlike friction).
    try {
      const captures = (await readdir(join(ctx.flumeDir, "refactor"))).filter(
        (f) => f.endsWith(".md") && f !== "README.md",
      );
      if (captures.length > 0) {
        return {
          ok: false,
          message: `state.md says \`Plan continues: no\` but ${captures.length} refactor capture(s) sit undrained in .flume/refactor/`,
        };
      }
    } catch {
      // no refactor directory — nothing undrained
    }
    // Live input 3: specs/ commits past the recorded spec cursor.
    const cursor = /^- Spec derived through:\s*([0-9a-f]{6,40})\b/im.exec(stateText)?.[1];
    if (cursor) {
      try {
        const out = execFileSync(
          "git",
          ["log", "--format=%h", `${cursor}..HEAD`, "--", "specs/"],
          { cwd: resolve(ctx.flumeDir, ".."), encoding: "utf8" },
        ).trim();
        if (out.length > 0) {
          return {
            ok: false,
            message: `state.md says \`Plan continues: no\` but ${out.split("\n").length} specs/ commit(s) sit past the spec cursor ${cursor}`,
            details: out,
          };
        }
      } catch {
        // bad sha or git unavailable — fail open
      }
    }
    return { ok: true, message: "quiet marker verified against inbox + spec cursor" };
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

// `cargo machete --with-metadata` (unused-dependency scan, adopted 2026-07-08)
// is deliberately NOT a gate: a manual/periodic check, same standing as
// `cargo llvm-cov` — see CLAUDE.md, "Common commands". No pipeline enforcement,
// so no shellGate here.

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

// No self-hosting gate in the chain: the recursive dogfood is live at the
// session layer (.claude/settings.json wires temper's SessionStart reporter
// and guard; the harness is authored in .temper/), and its gate — `temper
// check .temper` — rides sessions, not ticks. Build never edits the
// projections, so a per-tick check would only re-verify human territory.

/**
 * The SDK gate: `sdk/**` is TypeScript inside a
 * cargo-gated pipeline, so without this a TS slice would pass every gate
 * trivially while its own compiler and tests never run. `pnpm --dir sdk test`
 * runs tsc + node --test; afterMerge (serial, on the trunk, where
 * sdk/node_modules exists). Cheap when sdk/ is untouched — tsc on a tiny tree.
 */
const sdkGate = shellGate({
  name: "sdk test",
  when: "afterMerge",
  cmd: "pnpm",
  args: ["--dir", "sdk", "test"],
  failHint:
    "The SDK's tsc or tests failed — fix the slice; if node_modules is missing on the trunk, run `pnpm --dir sdk install`.",
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
  ".gitattributes",
  ".editorconfig",

  // CI
  ".github/**",

  // Vendored distribution surface — the plugin temper publishes (skill, hooks,
  // manifest; channel 3, `specs/distribution.md`). A generated surface
  // administered via spec, built here (and later by `temper bundle`), NOT
  // hand-curated like the territories below.
  "plugin/**",

  // The SDK (`specs/model/pipeline.md`; `specs/distribution.md`, channel 1).
  // Product code like src/** — the scaffold was the delegated human half;
  // every subsequent slice is build's.
  "sdk/**",

  // The two deliberate slits in the control-plane fence, one file per
  // uniquely-named capture: friction is agent→human harness feedback
  // (humans drain it); refactor is agent→plan structural-debt observation
  // (plan drains it into pending entries). See each directory's README.
  ".flume/friction/**",
  ".flume/refactor/**",

  // NOTE: build does NOT touch the rest of .flume/** (the control plane),
  // .claude/** or CLAUDE.md, specs/**, or docs/**. These are RATIFICATION
  // territory, not "human-authored" — nearly every byte in them is
  // agent-drafted, but drafted in-session with a human present, landing via
  // ceremony commits (`specs:`, `chore(harness):`). Build runs with no
  // human in its cycle, so it proposes
  // (leave the entry, surface the question — or a friction capture) instead
  // of writing. The harness writes the post-merge ship commit to
  // pending.json itself.
];

/**
 * Entry-fence preflight. An entry declares its paths; the fence is declared
 * globs — whether the work fits its fence is decidable at plan time, so
 * decide it here rather than let build discover it mid-tick.
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

/**
 * Reference resolution — an entry's declared surfaces must resolve at filing
 * time, the decidable subset of "cite what exists": `edit` and `retire`
 * paths exist on disk, `new` paths do not, and the `per` cite's section
 * text appears in its spec file. Symbol-level claims (a struct, a lock
 * column) stay a prompt convention — intent is not decidable here.
 */
const entryRefsGate: Gate = {
  name: "entry references resolve",
  when: "afterCommit",
  async run(ctx) {
    let raw: string;
    try {
      raw = await readFile(join(ctx.flumeDir, "plan", "pending.json"), "utf8");
    } catch {
      return { ok: true, message: "no pending.json to check" };
    }
    const result = parsePending(raw);
    if (!result.ok) return { ok: true, message: "parse gate owns malformed pending" };
    const repoRoot = resolve(ctx.flumeDir, "..");
    const offending: string[] = [];
    for (const entry of result.entries) {
      const gate = (entry as { gate?: { kind?: string } }).gate?.kind;
      if (gate !== "open" && gate !== "blockedBy") continue;
      const tag = (entry as { tag: string }).tag;
      const files = (entry as {
        files?: { new?: { path: string }[]; edit?: { path: string }[]; retire?: string[] };
      }).files;
      for (const f of files?.edit ?? []) {
        if (!existsSync(join(repoRoot, f.path))) offending.push(`  [${tag}] edit path missing on disk: ${f.path}`);
      }
      for (const p of files?.retire ?? []) {
        if (!existsSync(join(repoRoot, p))) offending.push(`  [${tag}] retire path missing on disk: ${p}`);
      }
      for (const f of files?.new ?? []) {
        if (existsSync(join(repoRoot, f.path))) offending.push(`  [${tag}] new path already exists: ${f.path}`);
      }
      const per = (entry as { per?: { path?: string; section?: string } }).per;
      if (per?.path) {
        const specPath = join(repoRoot, per.path);
        if (!existsSync(specPath)) {
          offending.push(`  [${tag}] per cite path missing: ${per.path}`);
        } else if (per.section) {
          const content = readFileSync(specPath, "utf8");
          if (!content.toLowerCase().includes(per.section.toLowerCase())) {
            offending.push(`  [${tag}] per section not found in ${per.path}: "${per.section}"`);
          }
        }
      }
    }
    if (offending.length === 0) {
      return { ok: true, message: "every pickable entry's references resolve" };
    }
    return {
      ok: false,
      message: `${offending.length} declared reference(s) do not resolve on disk — fix the entry, mark the surface new, or route it as an open question`,
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
    ".flume/friction/**",
    ".flume/refactor/**",
    // Plan does NOT touch specs/ (human-authored) or src/ (build's territory).
  ],
  gates: [pendingParseGate, entryFenceGate, entryRefsGate, planHonestyGate],
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
  gates: [fmtGate, clippyGate, testGate, sdkGate],
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
 * Model routing: the runtime exposes one shared `agent` export for every
 * phase, so per-phase model choice keys on the runtime's `<harness>` preamble
 * line `Phase: build` — the preamble precedes the template, so a template's
 * own headings never reach the match. Plan runs Fable, build runs Opus;
 * change a phase's model here.
 */
const planAgent = makeAgent("claude-fable-5");
const buildAgent = makeAgent("claude-opus-4-8");

export const agent: Agent = {
  name: "phase-model-router",
  invoke: (inv) =>
    (inv.prompt.includes("Phase: build") ? buildAgent : planAgent).invoke(
      inv,
    ),
};
