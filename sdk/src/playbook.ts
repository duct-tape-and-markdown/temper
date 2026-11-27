/**
 * The playbook — one area of guidance emitted across the surfaces Claude Code loads for
 * it: a `rule` holding the area's standing facts, and a `skill` whose `SKILL.md` routes to
 * one `supporting-doc` topic per procedure, every reference between them a typed mention.
 * The consumer supplies the content (the name, the facts, the description, the topics);
 * the shape supplies the wiring and the wording, whose router is a default the consumer
 * replaces.
 *
 * Its axes: the trigger the area loads on, who may invoke its skill, its topics, its
 * enforcement attachments, its execution and the agents that preload it. Execution and
 * preloading are typed so an author sees what the shape could do, and refused until they
 * are offered.
 */

import { GUARD_MATCHER, gateHook, rule, skill, supportingDoc } from "./builtins.js";
import type { Clause, Requirement } from "./contract.js";
import type { Member } from "./kind.js";
import { memberAddress, nestedAddress } from "./member-address.js";
import { bash, capability } from "./needs.js";
import type { Capability } from "./needs.js";
import { blocks, joined, linked, linkedText, mentionOf } from "./prose.js";
import type { Mentionable, Prose, Text } from "./prose.js";
import type { AreaMode, AreaRow } from "./generated/index.js";

/** Where a not-yet-offered axis is asked for. */
const REQUEST_AT = "https://github.com/duct-tape-and-markdown/temper/issues";

/**
 * Who may invoke the playbook's skill: `"model-and-user"` (the default) leaves both
 * channels live; `"user"` withholds it from Claude, so only `/name` runs it; `"model"`
 * hides it from the `/` menu, so only Claude loads it.
 */
export type PlaybookInvocation = "model-and-user" | "user" | "model";

/**
 * A playbook bound to the files of its area: its rule loads the first time Claude reads a
 * file its `paths` match.
 */
export interface PathsTrigger {
  /** The globs naming the area's files — the rule's `paths`. */
  readonly paths: readonly string[];
  /**
   * When the skill joins the skill listing: `"always"` (the default), so a question asked
   * before any file is read can find it, or `"on-touch"`, gating the listing on the same
   * `paths` — it spends no listing budget until the area is read, and is undiscoverable
   * until then.
   */
  readonly listing?: "always" | "on-touch";
}

/**
 * A playbook nested in a subdirectory: its rule lands in `<dir>/.claude/rules/` and its
 * skill in `<dir>/.claude/skills/`, both loaded on demand once Claude works in that
 * directory (code.claude.com/docs/en/memory, "Organize rules with `.claude/rules/`";
 * code.claude.com/docs/en/skills, "Load skills in monorepos and subdirectories";
 * retrieved 2026-11-24/25). Both members are named by the directory's segments joined
 * with `:` ahead of the playbook's name — `apps/web` and `deploy` are `apps:web:deploy`.
 */
export interface NestedTrigger {
  /** The subdirectory, relative to the harness root — `apps/web`. */
  readonly nested: string;
}

/**
 * What a playbook loads on — a {@link PathsTrigger} or a {@link NestedTrigger}; absent,
 * its rule loads at launch.
 */
export type PlaybookTrigger = PathsTrigger | NestedTrigger;

/**
 * How the skill runs once invoked — inline in the session, or forked to an agent whose
 * prompt its body becomes. Declared but not yet offered: setting it refuses.
 */
export type PlaybookExecution = "inline" | { readonly fork: string };

/** One procedure the skill routes to — its own file beside `SKILL.md`. */
export interface PlaybookTopic {
  /** The topic's file stem — `<name>.md` beside `SKILL.md`. */
  readonly name: string;
  /** The moment the router sends Claude to it — "deploying to staging". */
  readonly when: string;
  /** The procedure itself. */
  readonly prose: Prose;
}

/** What the router renderer receives: the skill's name and a link per topic. */
export interface RouterView {
  readonly name: string;
  readonly topics: readonly {
    readonly name: string;
    readonly when: string;
    /** The topic's path from `SKILL.md` — a typed mention, never a hand-typed path. */
    readonly link: Mentionable;
  }[];
}

/** What the pointer renderer receives: who may invoke the skill, and a mention of it. */
export interface PointerView {
  readonly invocation: PlaybookInvocation;
  readonly skill: Mentionable;
}

/**
 * The playbook's wording: `router` renders the skill's `SKILL.md` body, `pointer` the
 * line the rule closes on, sending Claude from the area's facts to its procedures.
 */
export interface PlaybookRenderer {
  readonly router: (view: RouterView) => Text;
  readonly pointer: (view: PointerView) => Text;
}

/** The requirements a playbook's rule and its skill each fill — the consumer's own domains. */
export interface PlaybookSatisfies {
  readonly rule?: readonly string[];
  readonly skill?: readonly string[];
}

/**
 * The guard attachment: temper's own `guard` reads every pending edit's path against the
 * area's paths, and `"point"` sends an edit there to the area's rule while `"refuse"`
 * denies it naming the rule.
 */
export interface PlaybookGuard {
  /** What the guard does with an edit on the area. */
  readonly mode: AreaMode;
  /**
   * The {@link tool} the guard hook runs temper as — the same tool the harness's own gate
   * hooks run, so the two hooks are one. Absent, the temper package's bin.
   */
  readonly tool?: Member;
}

/**
 * What must always be true of the area, held by the harness rather than by a prose MUST.
 * Permission rule syntax is Claude Code's (code.claude.com/docs/en/permissions, retrieved
 * 2026-11-26).
 */
export interface PlaybookAttachments {
  /**
   * Clauses judged over the area's members — its rule, its skill and its topics. They
   * attach to a requirement the playbook publishes, named `playbook:<name>`, that every
   * one of those members satisfies, so the lock records them over exactly that selection.
   */
  readonly clauses?: readonly Clause[];
  /**
   * The paths that need an owner present: each gitignore-style glob, relative to the
   * harness root, derives the `ask` rule `Edit(/<glob>)` — `Edit` covers every built-in
   * tool that edits files, and a leading `/` anchors the pattern at the project
   * ("Read and Edit", "How path patterns match").
   */
  readonly fence?: readonly string[];
  /**
   * The area's task scripts, each the command exactly as its procedure invokes it —
   * `./scripts/deploy.sh staging`. Each derives the allow rule `Bash(<command>)`, which
   * matches that one command and nothing wider ("Wildcard patterns": a rule with no `*`
   * matches one exact command).
   */
  readonly tasks?: readonly string[];
  /**
   * The guard hook over the area's paths — a `PreToolUse` gate hook over the file-writing
   * tools, the same hook the harness's own gates place, so the edge still runs one
   * command. Needs an area with paths: a paths or a nested trigger.
   */
  readonly guard?: PlaybookGuard;
}

/** What {@link playbook} takes. */
export interface PlaybookInit {
  /** The area's name — the rule's file stem and the skill's directory. */
  readonly name: string;
  /**
   * The skill's description — how a session finds it. Required even for a path-triggered
   * playbook, since its rule loads only on a read and never before.
   */
  readonly description: string;
  /** The area's standing facts — the rule's body, before its pointer to the skill. */
  readonly facts: Text;
  /** The procedures, one topic file each. At least one. */
  readonly topics: readonly PlaybookTopic[];
  /** What the area loads on; absent, its rule loads at launch. */
  readonly trigger?: PlaybookTrigger;
  /** Who may invoke the skill; absent, `"model-and-user"`. */
  readonly invocation?: PlaybookInvocation;
  /**
   * The requirements the area fills, per surface — each the `satisfies` edge of that
   * member. Per surface because a requirement may bind the kind that fills it: the
   * standing facts fill one domain, the procedures another. Absent, none.
   */
  readonly satisfies?: PlaybookSatisfies;
  /** Replaces the default wording, either half. */
  readonly render?: Partial<PlaybookRenderer>;
  /** What the harness holds true of the area; absent, nothing. */
  readonly attachments?: PlaybookAttachments;
  /** Declared but not yet offered — setting it refuses. */
  readonly execution?: PlaybookExecution;
  /** The agents that preload the skill. Declared but not yet offered — setting it refuses. */
  readonly preloadedBy?: readonly Member[];
}

/** The playbook's members — spread `members` into `harness({ members })`. */
export interface Playbook {
  readonly rule: Member;
  readonly skill: Member;
  readonly topics: readonly Member[];
  /** The guard attachment's hook, when the playbook declares one. */
  readonly guard?: Member;
  /** The rule, the skill, its topics, then its guard hook. */
  readonly members: readonly Member[];
}

/** Each guarded playbook's rule mapped to the area its guard binds — the fact
 * {@link playbookArea} reads, keyed by the member so the rule's fields stay exactly its
 * frontmatter. */
const guardedAreas = new WeakMap<Member, AreaRow>();

/**
 * The area a guarded playbook's rule records on the lock, or `undefined` for every other
 * member. The one reader is the declaration erasure (`declarations.ts`).
 */
export function playbookArea(member: Member): AreaRow | undefined {
  return guardedAreas.get(member);
}

/**
 * The paths a guard attachment binds: a paths trigger's own globs, or everything beneath a
 * nested trigger's directory.
 *
 * # Throws
 * If the playbook has no trigger, since an area loaded at launch names no paths to guard.
 */
function guardedPaths(name: string, trigger: PlaybookTrigger | undefined): string[] {
  if (trigger !== undefined && "paths" in trigger) return [...trigger.paths];
  if (trigger !== undefined) return [`${trigger.nested}/**`];
  throw new Error(
    `playbook \`${name}\`: a guard attachment binds the edits on the area's paths, and a playbook ` +
      "with no trigger names none — give it a `paths` or `nested` trigger, or drop `guard`.",
  );
}

/** The default router: a heading, then one line per topic naming its moment and its file. */
function defaultRouter(view: RouterView): Text {
  return linkedText(
    joined(
      [
        `# ${view.name}\n\nOpen the one topic the step at hand needs:\n`,
        ...view.topics.map((topic) => linked`- When ${topic.when}: read [${topic.name}](${topic.link}).`),
        "",
      ],
      "\n",
    ),
  );
}

/**
 * The default pointer. A user-only skill is withheld from Claude, so the rule asks the
 * user to run it rather than sending Claude to a skill it cannot load.
 */
function defaultPointer(view: PointerView): Text {
  // A nested skill's mention already renders its invocation, slash and all (`/apps/web:x`).
  const slash = view.skill.display.startsWith("/") ? "" : "/";
  const line =
    view.invocation === "user"
      ? linked`This area's procedures run only when the user invokes them: when one is needed, ask the user to run \`${slash}${view.skill}\`.`
      : linked`For a procedure in this area, use the \`${view.skill}\` skill: it routes to one topic per procedure.`;
  return linkedText(line);
}

/** The refusal of an axis the shape declares but does not offer yet. */
function notYetOffered(name: string, axis: string, field: string): Error {
  return new Error(
    `playbook \`${name}\`: ${axis} is declared but not yet offered — remove \`${field}\`, ` +
      `or request it at ${REQUEST_AT}.`,
  );
}

/** The skill fields an invocation sets. */
function invocationFields(invocation: PlaybookInvocation): Record<string, boolean> {
  switch (invocation) {
    case "model-and-user":
      return {};
    case "user":
      return { "disable-model-invocation": true };
    case "model":
      return { "user-invocable": false };
  }
}

/** Where a playbook's members land: their identity prefix and declared locations. */
interface Placement {
  readonly prefix: string;
  readonly rule?: string;
  readonly skill?: string;
}

/**
 * The placement a trigger gives the playbook named `name`: a nested trigger prefixes both
 * members with its directory's segments and declares where each lands under it.
 *
 * # Throws
 * If the nested directory is no plain relative path, or names a `.claude` folder.
 */
function placementOf(name: string, trigger: PlaybookTrigger | undefined): Placement {
  if (trigger === undefined || !("nested" in trigger)) return { prefix: "" };
  const segments = trigger.nested.split("/");
  const plain = segments.every(
    (segment) => segment !== "" && segment !== "." && segment !== ".." && segment !== ".claude" && !/[\\:*?]/.test(segment),
  );
  if (!plain) {
    throw new Error(
      `playbook \`${name}\`: nested directory \`${trigger.nested}\` is no plain relative path — ` +
        "name the subdirectory from the harness root, `/`-joined, with no `.`, `..`, `.claude`, `:` or glob segment.",
    );
  }
  const dir = segments.join("/");
  return {
    prefix: `${segments.join(":")}:`,
    rule: `${dir}/.claude/rules/${name}.md`,
    skill: `${dir}/.claude/skills/${name}/SKILL.md`,
  };
}

/**
 * The `ask` capabilities a fence derives, one `Edit(/<glob>)` rule per path.
 *
 * # Throws
 * If a path is blank, anchored (`/`, `//`, `~`), or climbs out of the harness root (`..`).
 */
function fenceNeeds(name: string, fence: readonly string[]): Capability[] {
  return fence.map((path) => {
    const relative = path.trim() !== "" && !/^[/~]/.test(path) && !path.split("/").includes("..");
    if (!relative) {
      throw new Error(
        `playbook \`${name}\`: fence path \`${path}\` is no glob relative to the harness root — ` +
          "write it the way a `paths` trigger names files, with no leading `/`, `~` or `..` segment.",
      );
    }
    return capability(`Edit(/${path})`, { list: "ask" });
  });
}

/**
 * The allow capabilities the task scripts derive, one exact `Bash(<command>)` rule each.
 *
 * # Throws
 * If a command is blank, carries a `*` (which would widen the rule past the invocation), or
 * chains commands with a shell separator (Claude Code matches each subcommand on its own,
 * so a compound rule matches nothing — "Compound commands").
 */
function taskNeeds(name: string, tasks: readonly string[]): Capability[] {
  return tasks.map((command) => {
    if (command.trim() === "" || command !== command.trim()) {
      throw new Error(`playbook \`${name}\`: task \`${command}\` is blank or padded — write the command as invoked.`);
    }
    if (command.includes("*")) {
      throw new Error(
        `playbook \`${name}\`: task \`${command}\` carries a \`*\` — a task's allow rule is the command ` +
          "exactly as invoked, never a wildcard.",
      );
    }
    if (/[;|&\n]/.test(command)) {
      throw new Error(
        `playbook \`${name}\`: task \`${command}\` chains commands — Claude Code matches each subcommand ` +
          "on its own, so allowlist one script per task.",
      );
    }
    return bash(command);
  });
}

/**
 * Declare a playbook: its rule, its routing skill and its topics, with its attachments.
 *
 * # Throws
 * If `execution` or `preloadedBy` is set (not yet offered), if `description` is blank, if
 * there is no topic, if two topics share a name, if a nested directory is no plain
 * relative path, if a fence path or task command is malformed, or if a guard attachment is
 * declared on a playbook with no trigger or with a tool that is not a `tool(...)` member.
 */
export function playbook(init: PlaybookInit): Playbook {
  const { name } = init;
  if (init.execution !== undefined) throw notYetOffered(name, "execution", "execution");
  if (init.preloadedBy !== undefined) throw notYetOffered(name, "preloading", "preloadedBy");
  if (init.description.trim() === "") {
    throw new Error(
      `playbook \`${name}\`: \`description\` is how a session finds the skill before any file is read — write one.`,
    );
  }
  if (init.topics.length === 0) {
    throw new Error(`playbook \`${name}\`: a playbook routes to at least one topic — declare its procedures.`);
  }
  const seen = new Set<string>();
  for (const topic of init.topics) {
    if (seen.has(topic.name)) throw new Error(`playbook \`${name}\`: two topics are named \`${topic.name}\`.`);
    seen.add(topic.name);
  }

  const invocation = init.invocation ?? "model-and-user";
  const router = init.render?.router ?? defaultRouter;
  const pointer = init.render?.pointer ?? defaultPointer;
  const trigger = init.trigger;
  const byPaths = trigger !== undefined && "paths" in trigger ? trigger : undefined;
  const paths = byPaths === undefined ? undefined : [...byPaths.paths];
  const gated = paths !== undefined && byPaths?.listing === "on-touch";
  const placement = placementOf(name, trigger);
  const id = `${placement.prefix}${name}`;

  const attached = init.attachments ?? {};
  const fence = fenceNeeds(name, attached.fence ?? []);
  const tasks = taskNeeds(name, attached.tasks ?? []);
  const clauses = attached.clauses ?? [];
  // The area requirement exists only to carry the attached clauses over the area's members.
  const area = clauses.length === 0 ? undefined : `playbook:${id}`;
  const claims = (names: readonly string[] | undefined) => {
    const all = [...(names ?? []), ...(area === undefined ? [] : [area])];
    return all.length === 0 ? {} : { satisfies: all };
  };
  const needs = (list: Capability[]) => (list.length === 0 ? {} : { needs: list });
  const requires = (): { requires?: Record<string, Requirement> } =>
    area === undefined
      ? {}
      : {
          requires: {
            [area]: {
              prose: `What must always be true of the \`${id}\` playbook's area — its rule, its skill and its topics.`,
              clauses,
            },
          },
        };
  const fields = {
    name: id,
    description: init.description,
    ...claims(init.satisfies?.skill),
    ...needs(tasks),
    ...invocationFields(invocation),
    ...(gated ? { paths } : {}),
    ...(placement.skill === undefined ? {} : { location: placement.skill }),
  };
  const draft = skill(fields);
  const host = memberAddress(draft);
  const body = router({
    name,
    topics: init.topics.map((topic) => ({
      name: topic.name,
      when: topic.when,
      link: { address: nestedAddress(host, supportingDoc.key, topic.name), display: topic.name, form: "path" },
    })),
  });
  const routing = skill({ ...fields, prose: body });
  const topics = init.topics.map((topic) =>
    supportingDoc({ name: topic.name, host: routing, ...claims(undefined), prose: topic.prose }),
  );
  const standing = rule({
    name: id,
    ...claims(init.satisfies?.rule),
    ...requires(),
    ...needs(fence),
    ...(paths === undefined ? {} : { paths }),
    ...(placement.rule === undefined ? {} : { location: placement.rule }),
    prose: blocks(init.facts, pointer({ invocation, skill: mentionOf(routing) })),
  });
  if (attached.guard === undefined) {
    return { rule: standing, skill: routing, topics, members: [standing, routing, ...topics] };
  }
  const { mode, tool } = attached.guard;
  guardedAreas.set(standing, { rule: memberAddress(standing), paths: guardedPaths(name, trigger), mode });
  const guard = gateHook({ event: "PreToolUse", matcher: GUARD_MATCHER, ...(tool === undefined ? {} : { tool }) });
  return { rule: standing, skill: routing, topics, guard, members: [standing, routing, ...topics, guard] };
}
