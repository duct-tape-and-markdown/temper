# Projection sample: `.claude/rules/connectdb.md`

What the agent reads, rendered from `rules.ts` — two derived citation
lines (the consult template: `"{prose}: the {cite.name} skill."`), three
directives the contract can count. No authored display text anywhere in
the source.

```markdown
---
# temper: managed projection — edit the owning .temper/ module, never this file.
paths: ["**/*.sql"]
---
# ConnectDB (T-SQL)

Proc structure, statements, formatting: the `sql-procedures` skill.

Naming — tables, columns, procedures, parameters: the `sql-naming` skill.

**Public-ID boundary.** Public-facing entities expose only their
uniqueidentifier (`ccPublic<Entity>ID`); the internal int IDENTITY PK
never crosses the app boundary. Procs take/return the GUID and resolve
internally.

**Schema boundary.** App-callable procedures live in the WEB schema.
CONNECT_UNSAFE is for system/batch work with no user context; dbo is
legacy (migrate when touched).

**Caller-context validation.** WEB-schema procs run under a validated
caller context (`usp__ProcedureHeader`) before doing work, pairing with
the context params SprocManager injects. Don't write WEB procs that skip
it.
```
