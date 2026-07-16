// The ambient tier as typed blocks. The platform map stays one orientation
// value (tables are prose; the block is the unit of accountability, not the
// sentence). Every pointer is an edge; every edge renders derived.
import { text } from "@dtmd/temper";
import { memory } from "@dtmd/temper/claude-code";
import { consult, directive, orientation, reference } from "./kinds.ts";
import { skill_harness_meta } from "./skills.ts";
import { rule_routing } from "./rules.ts";

export const memory_CLAUDE = memory({
  name: "CLAUDE",
  satisfies: ["orientation", "knowledge-routing"],
  content: [
    orientation({
      prose: text`Multi-tenant feedback and community management platform. Single-repo monorepo (\`centercode-platform\`).

| Component | Location | Tech | Purpose |
|-----------|----------|------|---------|
| Connect | \`src/connect/\` | ASP Classic, React, Vite | Main web application |
| Optimus | \`src/optimus/\` | .NET 9, Hangfire | APIs (Optimus, CAS, Bumblebee) |
| CLS | \`src/cls/\` | VB.NET, C# | Shared class libraries |
| OmegaOne | \`src/omegaone/\` | .NET | Gateway, Cliffjumper, InternalApi, CIM (PHP) |
| Sideswipe | \`src/sideswipe/\` | .NET 6, Docker, MassTransit | Orchestration platform |
| Mobile | \`src/centercodemobile/\` | .NET MAUI | Mobile app |
| Templates | \`src/templates/\` | Handlebars | Email/page templates |
| Database | \`database/<schema>/\` | T-SQL (Redgate-managed) | ConnectDB, CMSDB, OmegaOneDB, CIMDB, CentercodeStaticData |

Support files at repo root: \`scripts/\`, \`pipelines/\`, \`.github/\`, \`.claude/\`, \`.cartograph/\`. Authentication is shared sessions between ASP and Optimus (cookies + OAuth); CIM provisions customer URLs and configuration. Environment setup lives in the \`connect-dev:setup\` skill.`,
    }),
    reference({
      prose: text`Routing — the ASP-vs-Optimus front door, path-scoped`,
      cite: rule_routing,
    }),
    orientation({
      prose: text`Knowledge lives in runner; truth lives in source. How/why or behavioral questions route to the runner skill — invoke the skill, don't call the CLI. Code navigation (callers, routes, blast radius) routes to cartograph.`,
    }),
    consult({
      prose: text`**Promote, don't scatter.** Durable guidance that isn't yet captured — an invariant, convention, or decision — routes through the intake procedure, never left as chat prose`,
      cite: skill_harness_meta,
    }),
    directive({ prose: text`Always ask before committing or pushing.` }),
    directive({
      prose: text`You have access to the user's database — use \`.env\` for connection information. Don't ask the user to perform a query you could perform yourself.`,
    }),
  ],
});
