#!/usr/bin/env node
/**
 * scripts/build-changelog.mjs — mine a `## [Unreleased]` draft from git
 * history since the last release (`specs/distribution.md`, "Versioning": the
 * changelog is a release artifact mined from git history at the cut, not a
 * per-commit obligation). Ported from flume's miner of the same name.
 *
 * Prints the draft to stdout for a human to fold into CHANGELOG.md at cut
 * time. Never writes CHANGELOG.md itself — the mined text is raw material
 * (commit subject + body), not the polished record a human curates.
 *
 * Boundary resolution: the last release is CHANGELOG.md's own top-most
 * `## [X.Y.Z]` heading (Keep a Changelog orders newest-first), resolved to
 * the commit that introduced that heading text via `git log -S` — never "the
 * latest tag", which can lag a cut. A semver tag (`vX.Y.Z`) is the fallback
 * only when CHANGELOG.md records no version at all; every other failure to
 * resolve refuses.
 *
 * Entry source: `build:` commits — the one shipping unit per pending entry.
 * plan:/chore:/specs:/docs: commits are process bookkeeping, and a `build:`
 * commit that touches only `.flume/` is a capture (a fence-short attempt's
 * record), never a shipped change, so both stay out of the draft.
 *
 * Breaking marker: a commit body line starting with `BREAKING:` routes the
 * entry under `### Breaking`; everything else lands under `### Uncategorized`,
 * the curating human's cue for what is still unsorted.
 */

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { isDirectInvocation } from "./directInvocation.mjs";

const FIELD_SEP = "\x1f";
const RECORD_SEP = "\x1e";
const LOG_FORMAT = `%H${FIELD_SEP}%s${FIELD_SEP}%b${RECORD_SEP}`;

const BUILD_PAREN_TAG = /^build\(([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*)\):\s*(.+)$/;
// Bracket pairs must match — `(TAG]` is not a valid tag delimiter, so the
// two shapes are separate alternatives rather than independent open/close
// character classes (which would accept mismatched pairs like `(TAG]`).
const BUILD_TRAILING_TAG =
  /^build:\s*(.+?)\s*(?:\(([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*)\)|\[([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*)\])\s*$/;
const BUILD_PLAIN = /^build:\s*(.+)$/;
// Subjects that read as an attempted build: entry — used only to decide
// whether a parse failure is loud-worthy (a malformed tag attempt) versus
// an unrelated commit that legitimately isn't a build: entry.
const BUILD_SUBJECT_ATTEMPT = /^build[:(]/;

/**
 * Run git in `cwd` and return its stdout.
 *
 * `maxBuffer: Infinity`: the log over `<last release>..HEAD` grows with the
 * repository, and Node's default cap (1 MiB for `execFileSync`,
 * nodejs.org/api/child_process.html, retrieved 2026-09-30) fails a large read
 * as a spawn error. The process's own memory bounds it instead, failing
 * loudly rather than handing back a truncated log to curate a release from.
 */
function git(cwd, args) {
  return execFileSync("git", args, {
    cwd,
    encoding: "utf8",
    maxBuffer: Infinity,
  });
}

/**
 * Resolve the commit boundary for "since the last release", or `null` when no
 * prior release is recorded. Throws when CHANGELOG.md records a version the
 * boundary cannot be resolved from, rather than answering it from a tag.
 */
export function resolveLastRelease(root) {
  const changelogPath = join(root, "CHANGELOG.md");
  let changelogVersion = null;
  try {
    const text = readFileSync(changelogPath, "utf8");
    const m = text.match(/^## \[(\d+\.\d+\.\d+)\]/m);
    if (m) changelogVersion = m[1];
  } catch (err) {
    // Absent (`ENOENT`) is the only reading the tag fallback is scoped to.
    // Any other read failure refuses: falling through would answer the
    // boundary from a tag that may lag the last cut.
    if (err.code !== "ENOENT") {
      throw new Error(
        `${changelogPath} could not be read (${err.code}): ${err.message}`,
        { cause: err },
      );
    }
  }

  if (changelogVersion) {
    const needle = `## [${changelogVersion}]`;
    const out = git(root, [
      "log",
      "--reverse",
      "--format=%H",
      "-S",
      needle,
      "--",
      "CHANGELOG.md",
    ]);
    const sha = out.split("\n").find((line) => line.trim() !== "");
    // A recorded version is the boundary, resolvable or not — never answered
    // from a tag. The shape that gets here: a cut in progress, where
    // CHANGELOG.md carries the new heading in the working tree but no commit
    // has introduced it yet.
    if (!sha) {
      throw new Error(
        `CHANGELOG.md records version ${changelogVersion}, but no commit introducing '${needle}' to CHANGELOG.md was found — the release boundary is unresolved`,
      );
    }
    return sha.trim();
  }

  const tags = git(root, ["tag", "--list"])
    .split("\n")
    .map((t) => t.trim())
    .filter((t) => /^v\d+\.\d+\.\d+$/.test(t));
  if (tags.length > 0) {
    const latest = tags.sort(compareSemverTags).at(-1);
    return git(root, ["rev-list", "-n", "1", latest]).trim();
  }

  return null;
}

function compareSemverTags(a, b) {
  const pa = a.slice(1).split(".").map(Number);
  const pb = b.slice(1).split(".").map(Number);
  for (let i = 0; i < 3; i++) {
    const diff = (pa[i] ?? 0) - (pb[i] ?? 0);
    if (diff !== 0) return diff;
  }
  return 0;
}

function parseCommits(raw) {
  return raw
    .split(RECORD_SEP)
    .map((rec) => rec.replace(/^\n/, ""))
    .filter((rec) => rec.trim() !== "")
    .map((rec) => {
      const [sha, subject = "", body = ""] = rec.split(FIELD_SEP);
      return { sha, subject, body };
    });
}

function parseBuildSubject(subject) {
  let m = BUILD_PAREN_TAG.exec(subject);
  if (m) return { tag: m[1], desc: m[2].trim() };

  m = BUILD_TRAILING_TAG.exec(subject);
  if (m) return { tag: m[2] ?? m[3], desc: m[1].trim() };

  m = BUILD_PLAIN.exec(subject);
  if (m) return { tag: null, desc: m[1].trim() };

  return null;
}

function cleanBody(body) {
  const lines = body.split(/\r?\n/);
  // Trailing blank lines and trailing Co-Authored-By trailers interleave
  // (git appends a final newline after the last trailer), so strip both
  // from the end until neither pattern removes anything more.
  let shrank = true;
  while (shrank) {
    shrank = false;
    while (lines.length > 0 && lines.at(-1).trim() === "") {
      lines.pop();
      shrank = true;
    }
    while (lines.length > 0 && /^co-authored-by:/i.test(lines.at(-1).trim())) {
      lines.pop();
      shrank = true;
    }
  }
  while (lines.length > 0 && lines[0].trim() === "") lines.shift();
  return lines.join("\n");
}

function indent(text, prefix) {
  return text
    .split("\n")
    .map((line) => (line.length > 0 ? prefix + line : ""))
    .join("\n");
}

function formatEntry(desc, tag, body) {
  const header = tag ? `- ${desc} (${tag})` : `- ${desc}`;
  return body ? `${header}\n\n${indent(body, "  ")}` : header;
}

/** Whether a commit touches only `.flume/` — a build tick's capture, never a shipped change. */
function isCaptureOnly(root, sha) {
  const paths = git(root, ["show", "--name-only", "--format=", sha])
    .split("\n")
    .filter((p) => p.trim() !== "");
  return paths.length > 0 && paths.every((p) => p.startsWith(".flume/"));
}

/** Derive `## [Unreleased]` entries from a `sinceSha..HEAD`-shaped commit range (or the full history when `sinceSha` is `null`). */
export function deriveEntries(root, sinceSha) {
  const range = sinceSha ? `${sinceSha}..HEAD` : "HEAD";
  const raw = git(root, ["log", "--reverse", `--format=${LOG_FORMAT}`, range]);
  const commits = parseCommits(raw);

  const entries = [];
  const warnings = [];
  for (const commit of commits) {
    const parsed = parseBuildSubject(commit.subject);
    if (!parsed) {
      if (BUILD_SUBJECT_ATTEMPT.test(commit.subject)) {
        warnings.push(
          `${commit.sha.slice(0, 12)} matches no declared tag shape, dropped from draft: ${commit.subject}`,
        );
      }
      continue;
    }
    if (isCaptureOnly(root, commit.sha)) continue;
    const body = cleanBody(commit.body);
    entries.push({
      breaking: /^BREAKING:/im.test(body),
      text: formatEntry(parsed.desc, parsed.tag, body),
    });
  }
  return { range, entries, warnings };
}

export function renderSection(entries) {
  const breaking = entries.filter((e) => e.breaking);
  const rest = entries.filter((e) => !e.breaking);

  // `### Breaking` leads and `### Uncategorized` closes it: a markdown
  // subheading owns every line down to the next heading, so the second
  // heading is what bounds the first. A heading renders iff its bucket is
  // non-empty — a draft with one kind of entry carries one subheading.
  const blocks = ["## [Unreleased]"];
  if (breaking.length > 0) {
    blocks.push("### Breaking");
    blocks.push(breaking.map((e) => e.text).join("\n\n"));
  }
  if (rest.length > 0) {
    blocks.push("### Uncategorized");
    blocks.push(rest.map((e) => e.text).join("\n\n"));
  }
  return blocks.join("\n\n") + "\n";
}

function fail(message) {
  process.stderr.write(`[build-changelog] ${message}\n`);
  process.exitCode = 1;
}

function main() {
  const cwd = process.cwd();
  let root;
  try {
    root = git(cwd, ["rev-parse", "--show-toplevel"]).trim();
  } catch (err) {
    fail(`not a git repository (${cwd}): ${err.message}`);
    return;
  }

  let sinceSha;
  try {
    sinceSha = resolveLastRelease(root);
  } catch (err) {
    fail(`could not resolve the last release boundary: ${err.message}`);
    return;
  }

  // `deriveEntries` shells out to git; name what the tool was doing rather
  // than exit over a raw stack.
  let derived;
  try {
    derived = deriveEntries(root, sinceSha);
  } catch (err) {
    fail(`could not derive entries for the draft: ${err.message}`);
    return;
  }
  const { range, entries, warnings } = derived;

  for (const warning of warnings) {
    process.stderr.write(`[build-changelog] ${warning}\n`);
  }

  if (entries.length === 0) {
    fail(
      `no build: commits found in range '${range}' — nothing to changelog. ` +
        (sinceSha
          ? `Last release resolved to ${sinceSha.slice(0, 12)}.`
          : "No prior release found (no CHANGELOG.md version heading, no vX.Y.Z tag)."),
    );
    return;
  }

  process.stdout.write(renderSection(entries));
}

if (isDirectInvocation(import.meta.url)) {
  main();
}
