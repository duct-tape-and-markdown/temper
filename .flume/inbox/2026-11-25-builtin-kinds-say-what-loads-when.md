# Each built-in kind's guidance says what reaches context, and when

From Cascade (2026-11-25): the fact that decided their whole standards design, rule over skill, took a lab to learn, because the SDK says a rule "registers a path scope" and never what reaches the agent or when. The facts, observed on Claude Code 2.1.332 (Cascade lab; recorded in 0091) and cited in the skills docs (code.claude.com/docs/en/skills, retrieved 2026-11-25):

- an unscoped rule's content loads at launch;
- a path rule's content loads on the first Read, Write or Edit of a match, once per session;
- a skill's description is in context and its body loads on invocation;
- a paths-gated skill is only listed on touch;
- a supporting doc loads when the skill's body sends the agent to it.

Each built-in kind's `guidance` states its load semantics, cited, so that `schema`, `explain` and the `.temper/**` counsel rule (7f280c31) deliver it (intent invariant 8). Also carry the `render` hook on an embedded kind: an adopter read `blocks()` as rendering embedded values only as a TOML fence, and so hand-rendered markdown. A one-line render hook produces byte-identical prose (proved on Cascade's architecture standard).
