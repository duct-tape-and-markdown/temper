// The standards layer — deliberately unchanged from the live program. A
// convention reference is genuine long-form: the pull tier's stock, dense
// and complete, the sanctioned file() exception to the block grain. The
// posture model changes how these are *cited*, not how they are written.
import { file, skill } from "@dtmd/temper/claude-code";

const src = (path: string) => file(import.meta.url, path);
const reference = { "allowed-tools": ["Read"], "user-invocable": false } as const;

export const std_aspVbscript = skill({
  name: "asp-vbscript",
  description:
    "Classic ASP / VBScript conventions — OPTION EXPLICIT, banned constructs, JS injection helpers, comparison helpers, {{token}} markup templates. Consult when writing or reviewing ASP VBScript rather than inferring from legacy code.",
  ...reference,
  paths: ["src/connect/public/**/*.html"],
  satisfies: ["dev-standards"],
  prose: src("../standards/dev/asp-vbscript.md"),
});

export const std_csharp = skill({
  name: "csharp",
  description:
    "C# style conventions — naming, collections, control flow, file organization, controller/service architecture, warnings-as-errors. Consult when writing or reviewing C# rather than inferring from nearby code.",
  ...reference,
  paths: ["src/**/*.cs"],
  satisfies: ["dev-standards"],
  prose: src("../standards/dev/csharp.md"),
});

export const std_sqlNaming = skill({
  name: "sql-naming",
  description:
    "SQL naming conventions — tables, columns, procedures, parameters. Consult when creating or renaming database objects.",
  ...reference,
  paths: ["**/*.sql"],
  satisfies: ["dev-standards"],
  prose: src("../standards/dev/sql-naming.md"),
});

export const std_sqlProcedures = skill({
  name: "sql-procedures",
  description:
    "Stored-procedure conventions — structure, statements, formatting, advanced patterns. Consult when writing or changing T-SQL procedures.",
  ...reference,
  paths: ["**/*.sql"],
  satisfies: ["dev-standards"],
  prose: src("../standards/dev/sql-procedures.md"),
});

export const std_webUi = skill({
  name: "web-ui",
  description:
    "Server-rendered UI conventions (Handlebars/jQuery) — data-binding, component ID naming, modals, mobile, layout components. Consult when building or changing server-rendered UI.",
  ...reference,
  paths: ["src/templates/**/*.hbs", "src/connect/public/**/*.html"],
  satisfies: ["dev-standards"],
  prose: src("../standards/dev/web-ui.md"),
});

export const std_dataAccess = skill({
  name: "data-access",
  description:
    "The SprocManager data-access pattern — WEB schema, injected context params, fluent API, ASP COM bridge, DI registration. Consult for any database-access code on any surface.",
  ...reference,
  paths: [
    "src/cls/**/*.vb",
    "src/cls/**/*.cs",
    "src/connect/public/**/*.html",
    "src/optimus/**/*.cs",
    "src/optimus/**/*.vb",
    "src/omegaone/**/*.cs",
  ],
  satisfies: ["dev-standards"],
  prose: src("../standards/dev/data-access.md"),
});

export const std_jobs = skill({
  name: "jobs",
  description:
    "Hangfire job authoring — the recipe, queues, retry, tenant context. Consult when creating or changing background jobs.",
  ...reference,
  paths: ["src/optimus/**/*.cs", "src/omegaone/**/*.cs"],
  satisfies: ["dev-standards"],
  prose: src("../standards/dev/jobs.md"),
});

export const std_logging = skill({
  name: "logging",
  description:
    "Serilog conventions — levels, structured logging, enrichment. Consult when adding or changing logging.",
  ...reference,
  paths: ["src/optimus/**/*.cs", "src/omegaone/**/*.cs"],
  satisfies: ["dev-standards"],
  prose: src("../standards/dev/logging.md"),
});

export const std_deployStandards = skill({
  name: "deploy-standards",
  description:
    "The standards governing deploys and restarts — deploy the consuming service, worker-stop-before-deploy, the restart matrix, the WEB_SERVER requirement. Consult for any deploy/restart/what-do-I-ship question; the platform skill executes the operations.",
  when_to_use:
    "Trigger phrases: 'how do we deploy X', 'what do I ship for a CLS change', 'do I need to restart', 'is it safe to publish', 'why isn't my change live'.",
  ...reference,
  satisfies: ["operational-standards"],
  prose: src("../standards/operational/deploy.md"),
});

export const std_featureFlags = skill({
  name: "feature-flags",
  description:
    "Product policy for feature flags — when to flag vs not, default-off rollout, promote-per-track, flag ≠ license, retirement. Consult when deciding whether work ships behind a flag or managing a flag's lifecycle.",
  when_to_use:
    "Trigger phrases: 'should this ship behind a flag', 'rollout plan', 'when can this flag come out', 'flag or license', 'promote this feature'.",
  ...reference,
  satisfies: ["product-standards"],
  prose: src("../standards/product/feature-flags.md"),
});

export const standards = [
  std_aspVbscript,
  std_csharp,
  std_sqlNaming,
  std_sqlProcedures,
  std_webUi,
  std_dataAccess,
  std_jobs,
  std_logging,
  std_deployStandards,
  std_featureFlags,
];
