# 0118 — one interior globstar in a placement template

- **Date:** 2026-11-27 · **Status:** accepted

## Context

RULE-CLAIMS-NESTED-CLAUDE-RULES parked. 0115 has `rule` claim `**/.claude/rules/**`: a subdirectory's own rules folder, searched recursively as the runtime searches it. The template grammar (0081) allows only a leading `**/`. Each spelling inside the grammar breaks a cited fact. One stops recursing inside `.claude/rules/`. The other never finds a subdirectory's folder.

## Decision

**A template may carry one interior `**` after literal segments** (`representation.md`, "locus"), for a folder the runtime searches recursively. Identity joins every span a wildcard covers with `:` and leaves out the literal folders. So `apps/web/.claude/rules/frontend/x.md` is `apps:web:frontend:x`, and a root rule keeps `frontend:x`. Two documents whose spans join alike are coincident addresses, refused naming both paths. This is one identity grammar for every nested placement (0096).

## Rejected

- **A distinct joiner for nested rules** (`apps:web::frontend:x`). It keeps every legal tree representable, at the cost of a second identity grammar. The collision it avoids needs two files whose paths concatenate identically, and the refusal names both. Revisit this if an adopter hits the collision.
- **Unrestricted globstars.** Names narrow by clause, never by pattern.

## Consequences

The splice drops each `**` to zero segments on placement. Memory's any-depth claims cede to the rule's folder claim.
