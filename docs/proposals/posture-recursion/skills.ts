// The three procedures. platform shows steps + a consult + a reference in
// one body; harness-meta is the proposal's worked example; check-logs shows
// a procedure whose steps carry code without becoming long-form.
import { file, text } from "@dtmd/temper";
import { skill } from "@dtmd/temper/claude-code";
import { consult, directive, harnessMetaDoc, orientation, platformDoc, reference, step } from "./kinds.ts";
import { std_deployStandards } from "./standards.ts";

// ── platform ──

export const doc_reference = platformDoc({
  name: "reference",
  prose: file(import.meta.url, "./platform/reference.md"),
});

export const skill_platform = skill({
  name: "platform",
  description:
    "Build, publish, or deploy Connect components, or manage the dev server (services, IIS). Use when the user wants to build/publish/deploy a component or start/stop/restart/status the dev server's services or IIS. Wraps scripts/{build,publish,services,iis}.ps1.",
  "argument-hint": "[build|publish|deploy|services|iis] [component-or-action]",
  arguments: ["operation", "target"],
  satisfies: ["ops-execution"],
  content: [
    orientation({
      prose: text`Build/publish/deploy Connect components and manage the dev server via \`scripts/*.ps1\`. Pick the operation; pass component/action as the argument.`,
    }),
    consult({ prose: text`What to ship and when to restart — the governing policy`, cite: std_deployStandards }),
    directive({
      prose: text`Remote operations (publish, deploy, services, iis) act on the dev server: require \`WEB_SERVER\` set, and confirm before running. \`iis restart\` is \`iisreset\` — affects all sites; stop dependent services first.`,
    }),
    step({ prose: text`**build** \`[component]\` (default \`all\`) → \`.\\scripts\\build.ps1 <component>\`. Debug: add \`-Configuration Debug\`. \`cls\` aliases \`com\`; each consuming .NET service auto-builds its own CLS copy. Local, safe to run.` }),
    step({ prose: text`**publish** \`[component]\` → \`.\\scripts\\publish.ps1 <component>\` (\`-SkipBuild\` to publish existing artifacts; \`-RestartServices\` to restart after). Frontend files sync via Deploy Reloaded.` }),
    step({ prose: text`**deploy** \`[component]\` → \`.\\scripts\\publish.ps1 <component> -RestartServices\` (stop → build → publish → restart).` }),
    step({ prose: text`**services** \`<status|start|stop|restart> [cas|optimus|cliffjumper|all]\` (default \`status\`) → \`.\\scripts\\services.ps1\`.` }),
    step({ prose: text`**iis** \`<status|start|stop|restart>\` (default \`status\`) → \`.\\scripts\\iis.ps1 <action>\`.` }),
    step({ prose: text`After build → offer publish. After deploy → verify with \`services status\`. On failure → analyze (\`WEB_SERVER\` unset is the usual publish/deploy failure) and suggest the fix.` }),
    reference({ prose: text`Component, service, and script lookup, plus troubleshooting`, cite: doc_reference }),
  ],
});

// ── harness-meta ──

export const doc_economy = harnessMetaDoc({
  name: "economy",
  prose: file(import.meta.url, "./harness-meta/economy.md"),
});

export const skill_harness_meta = skill({
  name: "harness-meta",
  description:
    "Intake & maintenance for the harness itself — rules, CLAUDE.md, skills. Use when adding, moving, demoting, or auditing harness guidance: 'where does this belong', 'add a directive', 'demote this convention', 'harness audit', 'is this rule earning its place'.",
  "allowed-tools": ["Read", "Write", "Edit", "Grep", "Glob", "Bash", "AskUserQuestion"],
  satisfies: ["harness-governance"],
  content: [
    orientation({
      prose: text`Run this when the harness itself is the work — new guidance to place, a rule to demote, an audit.`,
    }),
    directive({
      prose: text`The harness is a projection of the temper program at \`.temper/\`: edit the owning module and run \`temper emit\` — a direct edit to \`CLAUDE.md\` or \`.claude/\` is drift, and the guard refuses it. \`temper check\` holds the structure.`,
    }),
    reference({
      prose: text`The doctrine — what belongs in a harness at all, the filing algorithm, and the judgments no clause can hold. Read it, file accordingly, and let the gate verify the wiring`,
      cite: doc_economy,
    }),
  ],
});

// ── check-logs ──

export const skill_check_logs = skill({
  name: "check-logs",
  description:
    "Check application logs for errors and debug issues. Use when debugging failures, investigating errors, or monitoring application behavior. Triggers: 'check logs', 'error logs', 'what went wrong', 'debug issue'.",
  "allowed-tools": ["Bash"],
  satisfies: ["ops-diagnostics"],
  content: [
    orientation({
      prose: text`Logs live at \`\\\\{WEB_SERVER}\\c$\\Connect_Logs\\CLS\\\`, one JSON file per app per day: \`{AppName}.{ProcessId}-log-{yyyyMMdd}.json\`. Fields: \`@t\` timestamp, \`@m\` message, \`@l\` level, \`@x\` exception, \`Job\`/\`JobId\` Hangfire identity.`,
    }),
    step({ prose: text`List recent logs: \`Get-ChildItem "\\\\\\\\$env:WEB_SERVER\\\\c$\\\\Connect_Logs\\\\CLS\\\\" -Filter "*Optimus*-log-*.json" | Sort-Object LastWriteTime -Descending | Select-Object -First 5\`` }),
    step({ prose: text`Read the tail: \`Get-Content <file> -Tail 30\`` }),
    step({ prose: text`Search errors: \`Select-String -Path <file> -Pattern '"@l":"Error"' | Select-Object -Last 10\`` }),
  ],
});
