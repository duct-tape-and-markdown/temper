/**
 * author's flume chain — plan → build, for a Rust/cargo project.
 *
 * Loaded by the flume CLI from `.flume/chain.ts`; the default export is a
 * **factory** the engine calls with its own API (flume ≥0.10) — every engine
 * value arrives on that parameter, and the only engine import left is
 * `import type`, so a second physical engine can never enter the process.
 * Two phases, no spec phase: the evergreen `specs/` corpus is human-
 * authored, never phase-written. Plan reconciles `pending.json` against the
 * corpus + current `src/` state; build ships entries to the trunk.
 *
 * The gates are the one place this differs materially from flume's
 * TypeScript dogfood chain: the product is Rust, so the validation gates
 * are cargo, not pnpm/tsc/vitest.
 */

import { execFileSync } from "node:child_process";
import { readFile, readdir } from "node:fs/promises";
import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { z } from "zod";

import type {
  Agent,
  Chain,
  ChainFactory,
  EntryExtension,
  Gate,
  Phase,
  TickContext,
} from "@dtmd/flume";

/** Absolute path to this chain.ts directory (.flume/), regardless of cwd. */
const CHAIN_DIR = dirname(fileURLToPath(import.meta.url));

// Ephemeral worktrees live OUTSIDE the repo (FLUME_WORKTREES_DIR, honored by
// the runtime's createWorktree): a worktree at <root>/.flume/worktrees/<slug>
// hands every build agent a pwd containing the root checkout's path as a
// prefix, and models derive <root> from it and operate there — the 07-18
// stray-write vector. Off-repo paths remove the derivation wholesale.
process.env.FLUME_WORKTREES_DIR ??= resolve(
  process.env.HOME ?? "/tmp",
  ".cache",
  "flume-worktrees",
  basename(resolve(CHAIN_DIR, "..")) || "repo",
);

// ---------- entry extension (flume ≥0.8) ----------

/**
 * Tag grammar, carried over verbatim from the 0.6 engine schema: the v0.8
 * core keeps only a mechanical-safety floor (charset, no whitespace, length),
 * so the ALL-CAPS convention this queue was authored under is now the
 * chain's to declare. Composes as an intersection with the core floor
 * (CHAIN-AUTHORING §11) — a narrowing, not a replacement.
 */
const TAG_PATTERN = /^[A-Z][A-Z0-9]*(?:[-.][A-Za-z0-9]+)*(?:\([a-z0-9]+\))?$/;

/**
 * The project-specific pending-entry fields (CHAIN-AUTHORING §10). Field
 * shapes and bounds carried over verbatim from the 0.6 engine schema the
 * queue was authored under; the ≤200/≤500 caps are the ones the
 * `pending-entry` rule warns about. `schemaDelta` (0.6 core, no consumer)
 * is retired with v0.8, not re-declared. One declaration drives both the
 * parse gates below (`parsePending`) and the plan prompt's
 * `{{PENDING_SCHEMA}}` (`renderSchemaForPrompt`), so the two cannot drift.
 */
const entryExtension = {
  tag: {
    schema: z.string().regex(TAG_PATTERN, "tag must match TAG_PATTERN"),
    hint: `"ALL-CAPS-WITH-DASHES" | "TAG-NAME(slice)"`,
  },
  summary: {
    schema: z.string().min(1).max(200),
    hint: `"one-line what, in human terms" // ≤200 chars — mechanics live in files[].description/acceptance/notes`,
  },
  per: {
    schema: z.strictObject({
      path: z.string().min(1),
      section: z.string().min(1),
    }),
    hint: `{ "path": "specs/….md", "section": "heading text, without the leading ##" } // the spec section that owns the intent — the entry's "why"`,
  },
  tests: {
    schema: z
      .array(
        z.strictObject({
          path: z.string().min(1),
          asserts: z.string().min(1),
        }),
      )
      .default([]),
    hint: `[ { "path": "tests/foo.rs", "asserts": "..." } ] // what must turn green for acceptance`,
  },
  acceptance: {
    schema: z.string().min(1),
    hint: `"what validation turns green to signal this entry shipped"`,
  },
  notes: {
    schema: z.string().max(500).optional(),
    hint: `"optional context not in the spec" // ≤500 chars; ends with 'scoped at <short-sha>'`,
  },
} satisfies EntryExtension;

/** Typed read of an entry's `per` cite — extension fields are `unknown` on the wire. */
const perOf = (entry: Record<string, unknown>) =>
  entryExtension.per.schema.parse(entry.per);

/**
 * The premise delta, rendered as data instead of an errand: what landed on
 * the entry's declared files between its `scoped at <sha>` stamp
 * (pending-entry rule) and this worktree's HEAD. The build prompt used to
 * instruct the agent to derive this itself; the render owns it now, and
 * failure degrades to a visible marker, never a silent empty.
 */
const scopedDelta = (ctx: TickContext): string => {
  const entry = ctx.assignedEntry;
  if (!entry) return "(no assigned entry)";
  let sha: string | undefined;
  try {
    const notes = entryExtension.notes.schema.parse(entry.notes);
    sha = /\bscoped at ([0-9a-f]{6,40})\b/.exec(notes ?? "")?.[1];
  } catch {
    // malformed notes — the parse gate owns that; degrade to no-stamp
  }
  if (!sha)
    return "(no `scoped at <sha>` stamp in notes — treat the tree as current)";
  const declared = [
    ...entry.files.edit.map((f) => f.path),
    ...entry.files.new.map((f) => f.path),
    ...entry.files.retire,
  ];
  try {
    const out = execFileSync(
      "git",
      ["log", "--oneline", `${sha}..HEAD`, "--", ...declared],
      { cwd: ctx.cwd, encoding: "utf8" },
    ).trim();
    return out.length > 0
      ? `commits on this entry's files since it was scoped (${sha}..HEAD):\n${out}`
      : `(none — the premise holds as scoped at ${sha})`;
  } catch {
    return `(delta unavailable — check manually: git log ${sha}..HEAD -- <entry files>)`;
  }
};

// ---------- project-specific gates ----------

/**
 * Marker honesty (the dispatch model's one decidable lie). Plan self-schedules:
 * one job per tick off its inputs, with `Plan continues: yes|no` driving the
 * re-wake. A tick that declares `no` while an input is plainly live — an
 * undrained inbox, a spec cursor trailing specs/ HEAD — would silently strand
 * work, and both conditions are statically checkable, so check them here
 * (same pattern as the pending-gate fence preflight). The cursor claims ROUTED-ness
 * (every slice derived into entries or registered as a keyed open fork), so
 * fork-parked spec content never pins the marker. Fail OPEN on bookkeeping
 * errors (missing files, unparseable cursor): a degradation is a missed
 * catch, never a wedged loop.
 */
const planHonestyGate: Gate = {
  name: "continuation marker is honest",
  when: "afterCommit",
  async run(ctx) {
    // A plan-phase honesty check judges plan's own commits alone: a stale
    // marker written by an earlier tick is not the current commit's
    // dishonesty, and a human `specs:` commit that merely moves HEAD past
    // a cursor is never this gate's to revert. Fail open when the subject
    // is unreadable, per the gate's own posture.
    if (ctx.commitSha) {
      try {
        const subject = execFileSync(
          "git",
          ["show", "-s", "--format=%s", ctx.commitSha],
          { cwd: ctx.repoRoot, encoding: "utf8" },
        ).trim();
        if (!subject.startsWith("plan:")) {
          return { ok: true, message: "not a plan commit — marker honesty is plan's own bar" };
        }
      } catch {
        // unreadable subject — fall through to the checks, failing open
      }
    }
    // Every read below is anchored to the commit under judgment, never the
    // disk this evaluation happens to run on: flume ≥0.12 evaluates a
    // singleton's gates both in the agent worktree and at the merge site,
    // and the primary checkout's tree (which the engine no longer touches)
    // holds the PREVIOUS tick's files — a disk read there judged the commit
    // by a stale state.md and an inbox it had in fact drained (first
    // observed reverting the 2026-08-26 inbox-drain tick). Disk is the
    // fallback only when there is no commit to read (fail-open posture).
    const fromCommit = (path: string): string | null => {
      if (!ctx.commitSha) return null;
      try {
        return execFileSync("git", ["show", `${ctx.commitSha}:${path}`], {
          cwd: ctx.repoRoot,
          encoding: "utf8",
        });
      } catch {
        return null;
      }
    };
    let stateText: string;
    const committedState = fromCommit(".flume/plan/state.md");
    if (committedState !== null) {
      stateText = committedState;
    } else {
      try {
        stateText = await readFile(join(ctx.flumeDir, "plan", "state.md"), "utf8");
      } catch {
        return { ok: true, message: "no state.md to check" };
      }
    }
    // The ledger cap (plan-state rule): state.md is ~10 lines by schema, 30
    // is the generous bound. An essay here is re-derived every tick — the
    // narrative's home is the plan commit body, written once.
    const stateLines = stateText.trimEnd().split("\n").length;
    if (stateLines > 30) {
      return {
        ok: false,
        message: `state.md is ${stateLines} lines (cap 30) — it is a ledger, not a narrative; move reasoning/evidence to the plan commit body and keep \`This tick:\` to one line`,
      };
    }
    if (!/^Plan continues:\s*no\b/im.test(stateText)) {
      return { ok: true, message: "marker is yes/absent — re-wake handles it" };
    }
    // Marker says quiet. Live input 1: an undrained inbox.
    {
      let inbox = fromCommit(".flume/inbox.md");
      if (inbox === null) {
        try {
          inbox = await readFile(join(ctx.flumeDir, "inbox.md"), "utf8");
        } catch {
          inbox = null; // no inbox file — nothing undrained
        }
      }
      const stripped = inbox?.replace(/<!--[\s\S]*?-->/g, "").trim() ?? "";
      if (stripped.length > 0) {
        return {
          ok: false,
          message: "state.md says `Plan continues: no` but .flume/inbox.md is undrained",
        };
      }
    }
    // Live input 2: undrained refactor captures (plan-drained, unlike friction).
    {
      let captures: string[] = [];
      if (ctx.commitSha) {
        try {
          captures = execFileSync(
            "git",
            ["ls-tree", "--name-only", ctx.commitSha, "--", ".flume/refactor/"],
            { cwd: ctx.repoRoot, encoding: "utf8" },
          )
            .split("\n")
            .map((l) => l.trim())
            .filter((f) => f.endsWith(".md") && !f.endsWith("README.md"));
        } catch {
          captures = []; // unreadable tree — fail open
        }
      } else {
        try {
          captures = (await readdir(join(ctx.flumeDir, "refactor"))).filter(
            (f) => f.endsWith(".md") && f !== "README.md",
          );
        } catch {
          captures = []; // no refactor directory — nothing undrained
        }
      }
      if (captures.length > 0) {
        return {
          ok: false,
          message: `state.md says \`Plan continues: no\` but ${captures.length} refactor capture(s) sit undrained in .flume/refactor/`,
        };
      }
    }
    // Live input 3: specs/ commits past the recorded spec cursor.
    const cursor = /^- Spec derived through:\s*([0-9a-f]{6,40})\b/im.exec(stateText)?.[1];
    if (cursor) {
      try {
        const out = execFileSync(
          "git",
          ["log", "--format=%h", `${cursor}..${ctx.commitSha ?? "HEAD"}`, "--", "specs/"],
          { cwd: ctx.repoRoot, encoding: "utf8" },
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

// ---------- phases: shared declarations ----------

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

  // CI — temper.yml is build's (precedent: 6df1b760 rewrote it in a build
  // tick). release.yml is human-only by convention despite matching this
  // glob: the `release` rule scopes to that exact path, and every commit
  // touching it is chore(release)/fix(release), never build:.
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

  // The three deliberate slits in the control-plane fence, one file per
  // uniquely-named capture: friction is agent→human harness feedback
  // (humans drain it); refactor is agent→plan structural-debt observation
  // (plan drains it into pending entries); amendments carry apply-ready
  // harness diffs humans ratify (0044). See each directory's README.
  ".flume/friction/**",
  ".flume/refactor/**",
  ".flume/amendments/**",

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
 * The capture channels a scoped build tick may always write, hoisted so the
 * build phase, the pending-gate fence, and the ship predicate below share
 * one declaration.
 */
const BUILD_CHANNEL_PATHS = [
  ".flume/friction/**",
  ".flume/refactor/**",
  ".flume/amendments/**",
];

/** Prefix forms of the channel globs, for the ship predicate's path test. */
const CHANNEL_PREFIXES = BUILD_CHANNEL_PATHS.map((g) => g.replace(/\*\*$/, ""));


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
const forkResolver = (repoRoot: string) => {
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

// ---------- chain factory (flume ≥0.10) ----------

const factory: ChainFactory = (flume) => {
  const {
    claudeCode,
    withSessionCapture,
    withTerminalRenderer,
    shellGate,
    parsePending,
    pendingGate,
    renderSchemaForPrompt,
  } = flume;

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
   * No `setupWorktree` hook (unlike flume's pnpm chain), so the v0.8
   * lockfile-aware `setupWorktree` helper has nothing here to replace: cargo
   * resolves deps from the global registry cache under `~/.cargo`, shared
   * across worktrees for free; only `target/` is per-worktree, and that is
   * the cold compile we keep off the parallel afterCommit path on purpose.
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
    args: [
      "clippy",
      "--all-targets",
      "--",
      "-D",
      "warnings",
      // Placeholder macros are denied mechanically (rust.md's own bar) —
      // the build prompt no longer carries a "no todo!()" instruction.
      "-D",
      "clippy::todo",
      "-D",
      "clippy::unimplemented",
    ],
    failHint:
      "clippy denies warnings and placeholder macros (todo!/unimplemented!); fix the lints (this also catches compile errors).",
  });

  const testGate = shellGate({
    name: "cargo test",
    when: "afterMerge",
    cmd: "cargo",
    args: ["test"],
    failHint: "Tests failed — entry reverted, returns to pending.",
  });

  const docGate = shellGate({
    name: "cargo doc",
    when: "afterMerge",
    cmd: "cargo",
    args: ["doc", "--no-deps", "--document-private-items", "--quiet"],
    failHint: "cargo doc denies broken intra-doc links via the crate-level deny; fix the stale link or unbracket it.",
  });

  // No self-hosting gate in the chain: the recursive dogfood is live at the
  // session layer (.claude/settings.json wires temper's SessionStart reporter
  // and guard; the harness is authored in .temper/), and its gate — `temper
  // check .` — rides sessions, not ticks. Build never edits the
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

  /**
   * One declaration drives both the phase's gate array and the prompt's
   * rendered `{{GATES}}` list (same idiom as the hoisted fence) — the list
   * the agent reads cannot drift from the gates the dispatcher runs.
   */
  const buildGates = [fmtGate, clippyGate, testGate, sdkGate, docGate];

  /**
   * Pending-list validation + entry-fence preflight, the `pendingGate`
   * builtin (flume ≥0.9): validates against the composed core+extension
   * schema, then pre-checks each fenced entry's declared `files` against
   * build's fence — decidable at plan time, so decided here rather than
   * discovered by build mid-tick. The `hint` carries the resolution rule
   * to the operator at the failure site. `fenceWhen` exempts parked/deferred
   * entries: plan must be able to park work whose paths sit outside today's
   * fence while the human decides whether to widen it (the 0.8-era
   * hand-rolled fork existed for exactly this predicate).
   */
  const buildPendingGate = pendingGate({
    extension: entryExtension,
    targetFence: {
      writablePaths: BUILD_WRITABLE_PATHS,
      entryChannelPaths: BUILD_CHANNEL_PATHS,
    },
    fenceWhen: (entry) =>
      entry.gate.kind === "open" || entry.gate.kind === "blockedBy",
    hint: "On a fence violation: widen BUILD_WRITABLE_PATHS in chain.ts (human territory) or have plan re-scope/park the entry — never squeeze the path through as a channel capture.",
  });

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
      const result = parsePending(raw, entryExtension);
      if (!result.ok) return { ok: true, message: "parse gate owns malformed pending" };
      const offending: string[] = [];
      for (const entry of result.entries) {
        if (entry.gate.kind !== "open" && entry.gate.kind !== "blockedBy")
          continue;
        const tag = entry.tag;
        for (const f of entry.files.edit) {
          if (!existsSync(join(ctx.repoRoot, f.path))) offending.push(`  [${tag}] edit path missing on disk: ${f.path}`);
        }
        for (const p of entry.files.retire) {
          if (!existsSync(join(ctx.repoRoot, p))) offending.push(`  [${tag}] retire path missing on disk: ${p}`);
        }
        for (const f of entry.files.new) {
          if (existsSync(join(ctx.repoRoot, f.path))) offending.push(`  [${tag}] new path already exists: ${f.path}`);
        }
        const per = perOf(entry);
        const specPath = join(ctx.repoRoot, per.path);
        if (!existsSync(specPath)) {
          offending.push(`  [${tag}] per cite path missing: ${per.path}`);
        } else {
          const content = readFileSync(specPath, "utf8");
          if (!content.toLowerCase().includes(per.section.toLowerCase())) {
            offending.push(`  [${tag}] per section not found in ${per.path}: "${per.section}"`);
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
      ".flume/amendments/**",
      // Plan does NOT touch specs/ (human-authored) or src/ (build's territory).
    ],
    gates: [buildPendingGate, entryRefsGate, planHonestyGate],
    promptArgs() {
      return { PENDING_SCHEMA: renderSchemaForPrompt(entryExtension) };
    },
    handoff(result) {
      // Wake-on-bail (v0.8, `TickResult.noCommit`): a plan tick that produced
      // no commit left state.md untouched, so the continuation marker below is
      // a *previous* tick's — a stale `yes` would re-wake a bailing plan
      // forever. Skip the marker, hand to build iff anything is pickable.
      if (result.noCommit) {
        return result.pendingAfter.some((e) => e.gate.kind === "open")
          ? ["build"]
          : [];
      }
      // Plan re-wakes itself when state.md ends with `Plan continues: yes`.
      // `after-build` yields the loop to a ready build wave first and resumes
      // planning when the wave hands back — legal only when the sole remaining
      // live job is non-queue-shaping (the posture sweep; the plan-state rule
      // owns the vocabulary). Otherwise hand to build if anything is pickable,
      // else hibernate.
      let marker = "";
      try {
        const stateText = readFileSync(
          resolve(CHAIN_DIR, "plan", "state.md"),
          "utf8",
        );
        marker =
          /^Plan continues:\s*(yes|after-build|no)\b/im
            .exec(stateText)?.[1]
            ?.toLowerCase() ?? "";
      } catch {
        // state.md missing — treat as stable.
      }
      const hasPickable = result.pendingAfter.some((e) => e.gate.kind === "open");
      if (marker === "after-build") return hasPickable ? ["build"] : ["plan"];
      if (marker === "yes") return ["plan"];
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
    // Per-entry narrowing, declared rather than inherited (0.10 flipped the
    // default off): the fence contract build's prompt teaches — "your commit
    // may touch exactly entry.files plus the capture dirs" — and the
    // under-scope flow (file a capture, plan re-scopes) are co-designed with
    // it, and the wave-width cost that motivated the flip is fenced off here
    // by the pending-entry rule's own bar (a shared path serializes via
    // blockedBy; files declares the honest ripple, never a defensive
    // superset). Revisit against the verdict rows' `invocations[]` if wave
    // width sags.
    scopeWritesToEntry: true,
    // The per-entry fence is entry.files ∪ these channels; writablePaths is
    // only the ceiling. The capture dirs must be granted here or a scoped
    // tick that files a capture reverts whole, capture included.
    entryChannelPaths: BUILD_CHANNEL_PATHS,
    gates: buildGates,
    // The park signal, declared (0.10: ship classification is the chain's
    // call, never inferred from paths by the engine). Build's prompt names
    // one legitimate not-shipped commit: capture-only — "commit the capture
    // alone, and end the tick" when entry.files under-scopes. Such a commit
    // keeps its entry pending for plan to re-scope; anything touching a
    // non-channel path shipped real work and drains the entry.
    shipped: ({ touchedPaths }) =>
      !(
        touchedPaths.length > 0 &&
        touchedPaths.every((p) =>
          CHANNEL_PREFIXES.some((prefix) => p.startsWith(prefix)),
        )
      ),
    promptArgs(ctx: TickContext) {
      if (!ctx.assignedEntry) {
        throw new Error("build phase requires an assignedEntry");
      }
      // Extension fields ride `assignedEntry` as `unknown` (v0.8) — narrow
      // through the declared schema, never a cast.
      const per = perOf(ctx.assignedEntry);
      return {
        ENTRY_JSON: JSON.stringify(ctx.assignedEntry, null, 2),
        TAG: ctx.assignedEntry.tag,
        PER_PATH: per.path,
        PER_SECTION: per.section,
        GATES: buildGates.map((g) => `- ${g.name} (${g.when})`).join("\n"),
        SCOPED_DELTA: scopedDelta(ctx),
      };
    },
    handoff(result) {
      // Nothing pickable (flume ≥0.13, `TickResult.nothingPickable`): the
      // wave never invoked an agent, and `pendingAfter` is the queue as it is
      // on disk — a quarantined `open` entry still reads `open` there. Handing
      // back to build here is the live-lock 0.12 burned runs on; hibernate
      // instead. A quarantine is per-run: the operator's relaunch (or a plan
      // re-scope) is what clears it, and `flume wake plan` forces the interim.
      if (result.nothingPickable) {
        return [];
      }
      // Wake-on-bail (v0.8, `TickResult.noCommit`): a wave that ran and
      // produced no usable commit — voluntary bail, whole-wave gate revert,
      // platform preempt — wakes plan to reconcile (re-scope, park, or route
      // an open question). Re-fanning out instead would thrash against the
      // same premise (prior-attempts blocks the re-pick, the wave no-ops, and
      // the pre-0.8 arms below would read that as a clean hibernate — exactly
      // the stranded-bail blind spot §3 of the migration guide names).
      if (result.noCommit) {
        return ["plan"];
      }
      // Waves chain: ship bookkeeping auto-opens blockedBy gates its own wave
      // satisfied (runtime, 07-18), so when pickable entries remain the next
      // wave forms with no plan interim. Plan reconciles at the drain — its
      // audit cursors span multi-wave windows by design. A true no-op wave
      // hibernates; `flume wake plan` forces it.
      const quarantined = new Set(result.quarantinedTags ?? []);
      if (
        result.pendingAfter.some(
          (e) => e.gate.kind === "open" && !quarantined.has(e.tag),
        )
      ) {
        return ["build"];
      }
      if (result.shippedTags.length === 0 && result.gateResults.length === 0) {
        return [];
      }
      return ["plan"];
    },
  };

  const temperChain: Chain = {
    phases: [plan, build],
    humanOnly: [], // no spec phase; the specs/ corpus is authored in-session, never by a phase
    entryExtension,
    // No `supervisorPolicy`: the v0.7 defaults (quarantineScope "run",
    // abortThreshold 3, maxParallel 4) match the behavior this bay ran under
    // pre-0.8, and the metrics record gives no reason to move any knob.
  };

  /**
   * Per-tick session capture + condensed terminal output. Sessions are rooted at
   * FLUME_DIR (the relocatable state root) so the whole footprint tears down with
   * one `rm`; the `?? CHAIN_DIR` fallback is defensive only. The filename is the
   * engine default — ISO timestamp + cwd basename, the collision discriminator
   * this chain used to hand-roll before 0.10 absorbed it.
   */
  const makeAgent = (model: string) =>
    withTerminalRenderer(
      withSessionCapture(
        claudeCode({
          outputFormat: "stream-json",
          extraArgs: [
            "--exclude-dynamic-system-prompt-sections",
            // The excluded dynamic sections carry the cwd statement, so say it
            // ourselves: pwd is the checkout, absolute paths derive from it,
            // never from a path glimpsed elsewhere. Static text — cache-stable.
            "--append-system-prompt",
            "Your shell starts at the root of the exact git checkout you own this session; `pwd` is authoritative. Construct absolute paths ONLY from `pwd` output. Never `cd` outside this checkout, and never operate on a repository path you inferred from a file's contents, an error message, or a worktree list — if a path does not start with your `pwd`, it is not yours.",
            "--model",
            model,
          ],
        }),
        { dir: resolve(process.env.FLUME_DIR ?? CHAIN_DIR, "sessions") },
      ),
    );

  /**
   * Model routing: per-phase, keyed on the runtime's `<harness>` preamble
   * line `Phase: <name>` in the rendered prompt — the mechanism the 07-10
   * note reserved. Plan crawls and plans on Sonnet; build chugs entries on
   * Haiku with the cargo gates as the safety net (judgment where wrongness
   * compounds, cheap where the gates catch it). Route build to Sonnet when
   * the queue carries cross-seam feature entries — SDK + engine in one
   * entry sits above the cheap tier's reliable ceiling. An unrecognized
   * phase runs the plan model.
   */
  const planAgent = makeAgent("claude-sonnet-5");
  const buildAgent = makeAgent("claude-haiku-4-5-20251001");
  const routed: Agent = {
    name: "phase-router",
    invoke: (opts) =>
      (/^Phase:\s*build\b/m.test(opts.prompt) ? buildAgent : planAgent).invoke(
        opts,
      ),
  };

  return {
    chain: temperChain,
    agent: routed,
    forkResolver,
  };
};

export default factory;
