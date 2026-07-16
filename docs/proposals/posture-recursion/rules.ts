// All eleven rules. The shape to notice: what was a `## Invariants`
// heading convention is now a countable list of directive values, and
// every pointer at the standards layer is a consult edge whose rendering
// derives. No rule authors a display string or an inline interpolation.
import { text } from "@dtmd/temper";
import { rule } from "@dtmd/temper/claude-code";
import { consult, directive, orientation } from "./kinds.ts";
import {
  std_aspVbscript,
  std_csharp,
  std_dataAccess,
  std_featureFlags,
  std_jobs,
  std_logging,
  std_sqlNaming,
  std_sqlProcedures,
  std_webUi,
} from "./standards.ts";

// ── always-on ──

export const rule_protocol = rule({
  name: "protocol",
  satisfies: ["always-on-directives"],
  content: [
    directive({ prose: text`**Rule 0.** When anything fails: STOP. Explain to the user. Wait for confirmation before proceeding.` }),
    directive({ prose: text`**Significant actions.** For non-trivial operations, state DOING / EXPECT / IF WRONG. Then execute, then compare; mismatch = stop and surface.` }),
    directive({ prose: text`**Verification rhythm.** Pause and verify after each logical unit of work. Thinking isn't verification — observable output is.` }),
    directive({ prose: text`**Epistemic hygiene.** "I believe X" is not "I verified X". "I don't know" beats confident guessing.` }),
    directive({ prose: text`**Source authority.** Conventions come from the standards skills, not nearby code — much of the platform is legacy and not exemplary. Presence ≠ currency: confirm a pattern holds in recently-added code before applying it; existence ≠ necessity. Legacy-smell escalates to a human.` }),
    directive({ prose: text`**Autonomy check.** Uncertain plus consequential means ask the user first — cheap to ask, expensive to guess wrong.` }),
    directive({ prose: text`**Context decay.** After extended work, verify you still understand the original goal; say "losing the thread" when degraded.` }),
    directive({ prose: text`**Chesterton's fence.** Can't explain why something exists? Don't touch it until you can.` }),
    directive({ prose: text`**Minimal changes.** Only what is requested or clearly necessary; three similar lines beat a premature abstraction.` }),
    directive({ prose: text`**Handoffs.** When stopping: what's done, what's blocked, open questions, files touched. When confused: stop, present theories, get signoff; never silently retry.` }),
  ],
});

// ── path-scoped ──

export const rule_connectdb = rule({
  name: "connectdb",
  paths: ["**/*.sql"],
  satisfies: ["area-directives"],
  content: [
    consult({ prose: text`Proc structure, statements, formatting`, cite: std_sqlProcedures }),
    consult({ prose: text`Naming — tables, columns, procedures, parameters`, cite: std_sqlNaming }),
    directive({ prose: text`**Public-ID boundary.** Public-facing entities expose only their uniqueidentifier (\`ccPublic<Entity>ID\`); the internal int IDENTITY PK never crosses the app boundary. Procs take/return the GUID and resolve internally.` }),
    directive({ prose: text`**Schema boundary.** App-callable procedures live in the WEB schema. CONNECT_UNSAFE is for system/batch work with no user context; dbo is legacy (migrate when touched).` }),
    directive({ prose: text`**Caller-context validation.** WEB-schema procs run under a validated caller context (\`usp__ProcedureHeader\`) before doing work, pairing with the context params SprocManager injects. Don't write WEB procs that skip it.` }),
  ],
});

export const rule_cls = rule({
  name: "cls",
  paths: ["src/cls/**/*.vb", "src/cls/**/*.cs"],
  satisfies: ["area-directives"],
  content: [
    directive({ prose: text`**All data access via SprocManager** — no hand-rolled ADO. WEB schema by default; non-WEB procs need an explicit schema prefix.` }),
    consult({ prose: text`Full pattern — injected context params, the ASP/COM bridge, proc signature, fluent API, null handling`, cite: std_dataAccess }),
    directive({ prose: text`**A CLS change is high-blast-radius.** Every consuming service bundles its own CLS copy at publish, so a CLS change ships only by redeploying each consumer — not just com. Consumers and publish targets: ask cartograph or read \`scripts/publish.ps1\`.` }),
  ],
});

export const rule_csharp = rule({
  name: "csharp",
  paths: ["src/**/*.cs"],
  satisfies: ["area-directives"],
  content: [
    consult({ prose: text`Full style reference — naming, collections, control flow, file organization`, cite: std_csharp }),
    directive({ prose: text`**Thin controllers.** Controllers handle HTTP routing/responses only; validation, business logic, data access, and DTO mapping live in services.` }),
    directive({ prose: text`**Warnings are errors.** Production builds use TreatWarningsAsErrors; fix all warnings before committing.` }),
  ],
});

export const rule_connect_asp = rule({
  name: "connect-asp",
  paths: ["src/connect/public/**/*.html"],
  satisfies: ["area-directives"],
  content: [
    orientation({ prose: text`ASP files use the \`.html\` extension (IIS-configured) — VBScript server-side includes with \`<% %>\` delimiters, not static HTML.` }),
    directive({ prose: text`Database access goes through SprocManager, not hand-rolled ADO; don't copy the legacy \`dbw_*.html\` wrappers.` }),
    consult({ prose: text`VBScript surface conventions`, cite: std_aspVbscript }),
    consult({ prose: text`Data access from ASP (the COM bridge)`, cite: std_dataAccess }),
    consult({ prose: text`UI conventions on this surface — components, modals, mobile`, cite: std_webUi }),
  ],
});

export const rule_connect_optimus = rule({
  name: "connect-optimus",
  paths: ["src/optimus/**/*.cs", "src/optimus/**/*.vb", "src/optimus/**/*.cshtml"],
  satisfies: ["area-directives"],
  content: [
    consult({ prose: text`Data access & DI registration`, cite: std_dataAccess }),
    consult({ prose: text`Hangfire job authoring`, cite: std_jobs }),
    consult({ prose: text`Serilog logging`, cite: std_logging }),
    directive({ prose: text`**One session store.** Optimus and ASP validate against ConnectDB.tbl_Sess; never route Optimus session validation elsewhere. OmegaOneDB.tbl_Sess is the separate mobile/gateway store.` }),
    directive({ prose: text`**Jobs carry tenant context.** Enqueue context-aware work through OptimusJobClient, not BackgroundJob.Enqueue; UnsafeOptimusJobClient is only for context-less server-wide jobs.` }),
  ],
});

export const rule_omegaone = rule({
  name: "omegaone",
  paths: ["src/omegaone/**/*.cs"],
  satisfies: ["area-directives"],
  content: [
    orientation({ prose: text`The \`src/omegaone\` .NET surface (Cliffjumper, InternalApi, OmegaOne) follows the platform's shared .NET conventions — apply those relevant to the code you're editing.` }),
    consult({ prose: text`Data access via SprocManager`, cite: std_dataAccess }),
    consult({ prose: text`Hangfire background jobs`, cite: std_jobs }),
    consult({ prose: text`Serilog logging`, cite: std_logging }),
  ],
});

export const rule_web_ui = rule({
  name: "web-ui",
  paths: ["src/templates/**/*.hbs"],
  satisfies: ["area-directives"],
  content: [
    orientation({ prose: text`The Connect app's legacy server-rendered UI: Handlebars under \`src/templates/\` plus the jQuery component layer. React under \`src/connect/src/\` is governed by \`frontend.md\`; ASP markup by \`connect-asp.md\`.` }),
    consult({ prose: text`Full conventions — data-binding, component ID naming, modals, mobile, layout`, cite: std_webUi }),
  ],
});

export const rule_feature_flags = rule({
  name: "feature-flags",
  paths: [
    "src/connect/public/**/*.html",
    "src/templates/**/*.hbs",
    "src/optimus/**/*.cs",
    "database/cimdb/Data/dbo._edition_licensing_type_Data.sql",
  ],
  satisfies: ["area-directives"],
  content: [
    directive({ prose: text`**Flags resolve fail-safe-off.** An unknown or not-yet-provisioned key resolves to false on all three gating surfaces (ASP \`HasFeature\`, Handlebars \`{{#if (HasFeature)}}\`, Optimus \`[RequireFeature]\` → 404), keyed off the one per-impl cache. Never gate the other way (render then hide) and never special-case an unknown key to true — fail-safe-off is what makes merging behind a flag safe.` }),
    consult({ prose: text`Whether to flag and the rollout lifecycle (product policy)`, cite: std_featureFlags }),
  ],
});

export const rule_frontend = rule({
  name: "frontend",
  paths: [
    "src/connect/src/**/*.ts",
    "src/connect/src/**/*.tsx",
    "src/connect/src/**/*.jsx",
    "src/connect/src/**/*.js",
    "src/connect/src/**/*.css",
    "src/connect/src/**/*.scss",
  ],
  satisfies: ["area-directives"],
  content: [
    orientation({ prose: text`The React/Vite/TypeScript app under \`src/connect/src/\`. Classic ASP under \`src/connect/public/\` is governed by \`connect-asp.md\`.` }),
    directive({ prose: text`**Version ceilings.** React 18 (no 19-only APIs) and Tailwind v2 (no v3+ utilities). Ground truth is \`src/connect/package.json\` — verify there before assuming a newer API exists; this rule is the directive surface, the manifest is the source of versions.` }),
  ],
});

export const rule_routing = rule({
  name: "routing",
  paths: ["src/connect/public/web.config", "src/optimus/**/*.cs", "src/optimus/**/*.cshtml"],
  satisfies: ["area-directives"],
  content: [
    orientation({ prose: text`The ASP-vs-Optimus front door is IIS handler precedence in \`src/connect/public/web.config\`, not URL rewrites: \`ASPClassic\` claims \`*.html\`, the terminal \`Optimus\` handler claims \`*\` and runs only for requests nothing earlier claimed. The \`<rewrite>\` rules transform specific URLs; they do not perform the split.` }),
    directive({ prose: text`**Optimus routes are extensionless and non-colliding.** A route ending in .html is hijacked by ASPClassic; a registered static extension is hijacked by StaticFile*. When changing the front-door split, edit handler ordering in web.config — never reach for a rewrite rule.` }),
  ],
});
