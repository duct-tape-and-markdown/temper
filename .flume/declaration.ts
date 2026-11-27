/**
 * temper's flume harness declaration — the one file that decides this
 * environment (`@dtmd/flume`, spec/harness.md, *What a consumer declares*).
 * Everything here is a stack fact of this repository: a Rust crate gated by
 * cargo, a TypeScript SDK beside it, and a spec corpus authored in-session.
 * The phases, prompts, judges and record conventions are the package's.
 */

import { homedir } from "node:os";
import { basename, resolve } from "node:path";

import { scriptRunner, type DeclarationInput } from "@dtmd/flume/harness";

/** The agent every phase runs as — top tier while the queue is cross-seam (SDK + engine). */
const agent = {
  model: "claude-opus-5-5",
  extraArgs: [
    "--exclude-dynamic-system-prompt-sections",
    // The excluded sections carry the cwd statement, so say it ourselves.
    "--append-system-prompt",
    "Your shell starts at the root of the exact git checkout you own this session; `pwd` is authoritative. Construct absolute paths ONLY from `pwd` output. Never `cd` outside this checkout, and never operate on a repository path you inferred from a file's contents, an error message, or a worktree list — if a path does not start with your `pwd`, it is not yours.",
  ],
};

export const declaration = {
  specLocus: ["specs/*.md", "specs/**/*.md"],

  fence: {
    // Product code and its build surface. Build never writes the control
    // plane, `.claude/`, `CLAUDE.md`, `.temper/`, `specs/` or `docs/`: those
    // land through in-session ceremony commits (PROTOCOL.md, "Lanes").
    build: [
      "src/**",
      "tests/**",
      "benches/**",
      "examples/**",
      "build.rs",
      "Cargo.toml",
      "Cargo.lock",
      "rustfmt.toml",
      "clippy.toml",
      "rust-toolchain.toml",
      "README.md",
      "CHANGELOG.md",
      "LICENSE",
      "LICENSE-*",
      ".gitignore",
      ".gitattributes",
      // `release.yml` matches but is human-only (the `release` rule).
      ".github/**",
      "plugin/**",
      "sdk/**",
    ],
  },

  // Build writes anywhere in its fence; an entry's `files` is the
  // partitioner's prediction, as the package's discipline says, never a
  // second fence. Holding build to it turned every census miss into a park.
  scopeWritesToEntry: false,

  runner: scriptRunner({ command: "node", args: [".flume/runner.mjs"] }),

  gates: {
    build: [
      { kind: "shell", when: "afterCommit", command: "cargo fmt --all --check" },
      // Every compiling gate is capped at -j 4: the host shares ~7 GB with a
      // second loop, and uncapped cargo has OOM-rebooted it.
      {
        kind: "shell",
        when: "afterMerge",
        command: "cargo clippy -j 4 --all-targets -- -D warnings -D clippy::todo -D clippy::unimplemented",
      },
      { kind: "shell", when: "afterMerge", command: "cargo test -j 4 --no-fail-fast -- --test-threads=4" },
      { kind: "shell", when: "afterMerge", command: "pnpm --dir sdk test" },
      { kind: "shell", when: "afterMerge", command: "cargo doc -j 4 --no-deps --document-private-items --quiet" },
    ],
  },

  agents: {
    build: agent,
    "plan-inbox": agent,
    "plan-derive": agent,
    "plan-sweep": agent,
  },

  supervisor: { maxParallel: 2 },

  // The SDK carries its own lockfile; the engine's tests build and run it.
  setup: { directories: [".", "sdk"] },

  slices: {
    enabled: ["plan-inbox", "plan-derive", "plan-sweep"],
    sweep: {
      domain: ["src/**", "sdk/src/**", "tests/**"],
      posturePages: [".claude/skills/engineering/*.md"],
    },
  },

  friction: "friction",

  // Off-repo: a worktree under the checkout hands the agent a pwd with the
  // primary checkout's path as a prefix, and models derive it and write there.
  worktreesBase: (paths) =>
    resolve(homedir(), ".cache", "flume-worktrees", basename(paths.repoRoot) || "repo"),
} satisfies DeclarationInput;

export default declaration;
