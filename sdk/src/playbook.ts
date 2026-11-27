/**
 * The playbook — one area of guidance emitted across the surfaces Claude Code loads for
 * it: a `rule` holding the area's standing facts, and a `skill` whose `SKILL.md` routes to
 * one `supporting-doc` topic per procedure, every reference between them a typed mention.
 * The consumer supplies the content (the name, the facts, the description, the topics);
 * the shape supplies the wiring and the wording, whose router is a default the consumer
 * replaces.
 *
 * Its axes: the trigger the area loads on, who may invoke its skill, its topics, its
 * execution and the agents that preload it. Execution and preloading are typed so an
 * author sees what the shape could do, and refused until they are offered.
 */

import { rule, skill, supportingDoc } from "./builtins.js";
import type { Member } from "./kind.js";
import { memberAddress, nestedAddress } from "./member-address.js";
import { blocks, joined, linked, linkedText, mentionOf } from "./prose.js";
import type { Mentionable, Prose, Text } from "./prose.js";

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

/** What a playbook loads on — a {@link PathsTrigger}; absent, its rule loads at launch. */
export type PlaybookTrigger = PathsTrigger;

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
  /** Replaces the default wording, either half. */
  readonly render?: Partial<PlaybookRenderer>;
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
  /** The rule, the skill, then its topics. */
  readonly members: readonly Member[];
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
  const line =
    view.invocation === "user"
      ? linked`This area's procedures run only when the user invokes them: when one is needed, ask the user to run \`/${view.skill}\`.`
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

/**
 * Declare a playbook: its rule, its routing skill and its topics.
 *
 * # Throws
 * If `execution` or `preloadedBy` is set (not yet offered), if `description` is blank, if
 * there is no topic, or if two topics share a name.
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
  const paths = init.trigger === undefined ? undefined : [...init.trigger.paths];
  const gated = paths !== undefined && init.trigger?.listing === "on-touch";

  const fields = { description: init.description, ...invocationFields(invocation), ...(gated ? { paths } : {}) };
  const draft = skill({ name, ...fields });
  const host = memberAddress(draft);
  const body = router({
    name,
    topics: init.topics.map((topic) => ({
      name: topic.name,
      when: topic.when,
      link: { address: nestedAddress(host, supportingDoc.key, topic.name), display: topic.name, form: "path" },
    })),
  });
  const routing = skill({ name, ...fields, prose: body });
  const topics = init.topics.map((topic) => supportingDoc({ name: topic.name, host: routing, prose: topic.prose }));
  const standing = rule({
    name,
    ...(paths === undefined ? {} : { paths }),
    prose: blocks(init.facts, pointer({ invocation, skill: mentionOf(routing) })),
  });
  return { rule: standing, skill: routing, topics, members: [standing, routing, ...topics] };
}
