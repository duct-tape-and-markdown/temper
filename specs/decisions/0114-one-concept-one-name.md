# 0114 — one concept, one name

- **Date:** 2026-11-26 · **Status:** accepted

## Context

A vocabulary pass ran in two steps:
1. An inventory listed 604 author-facing names with their real meanings.
2. A blind name test guessed each meaning from the bare names, a one-paragraph definition and the Claude Code docs.

Divergence between a guess and the real meaning was the evidence. The Claude Code kind surface read cleanly. Confusion gathered in the model layer, and in names that carry two concepts or two names that carry one.

One filter decided what counts: whether an author writes or reads the name. A spec-internal term may stay precise.

## Decision

**One concept, one name** (`authoring.md`, "The SDK"): one name per concept across the SDK, the CLI, the lock and the corpus, with the runtime's words used only in the runtime's sense. It rules six cases:

- **A hook's lifecycle event is its `event`, never its `name`.** On every other kind, `name` is the identity.
- **Severity is `required` or `advisory` wherever an author reads it.** "error" and "warn" retire from the corpus and the help text.
- **`locus` stays the one noun for where members live** (at, embedded, nested file), defined where an author meets it. The relocation delta is spelled as the locus it replaces, so `governs` retires from the SDK, the lock and the finding ids.
- **The program exports its harness as its default.** The engine reads it through the SDK's one pipe, so no program carries the stdout plumbing or a call to an SDK `emit` that shares the CLI verb's name.
- **A requirement is a requirement wherever an author reads it,** never a "domain", which is a kind's.
- **A clause has an id**, keyed `clause` on every surface that names one. "label", "address", "rule" and "code" retire for it.

It also fixes mechanical naming:
- predicate segments of a clause id are kebab-case;
- a default contract is exported under its kind's name;
- every verb takes its workspace the same way;
- a count the docs state is true of the code.

## Rejected

- **`governs` as the one word.** It names only the `at` form, and it reads as a verb.
- **Renaming the SDK `emit` to `compile`.** That keeps the plumbing in every program.
- **`code` for a clause's id.** It is one more generic word.
- **Renaming every generic word** (`kind`, `gate`, `template`, `file`, `fields`). That is churn. A secondary sense is renamed only where an author meets it.
- **Treating Claude Code's own words as collisions** (`memory`, `tool`, `command`, `rule` as a kind). temper matches the provider on purpose.

## Consequences

These are breaking renames, made at 0.0.x, and each one owes a migration note. They should land before the playbook ships its options, so those options are named once.
