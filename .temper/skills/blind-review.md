# Blind review

A blind review gets a reading of a design, a set of names, or a proposal
that is not anchored on this session's reasoning. A reviewer handed your
conclusion tends to hand it back confirmed. Use one when the session is about
to encode something it argued itself into, or when the real question is how a
stranger would read it.

## Brief

The brief is the reviewer's only context, so it carries the whole setup:

1. **Primary sources, not ours.** Point the reviewer at the external facts it
   should read itself: the current Claude Code docs pages, by URL. Tell it
   not to read the repository, since its docs may be stale and the point is a
   fresh read.
2. **A fresh definition.** Write what temper is in your own words, current as
   of today, in one or two paragraphs. Never paste or link the repo's README
   or spec as the definition.
3. **The unit, never your answer.** Give the one thing under review: the base
   pattern, the bare names, the proposal. Withhold your variations, your
   ranking and your preferred names. They are what you are testing.
4. **Questions that can disagree with you.** Ask whether the unit is right,
   what varies, what to prioritize, what to call it, and where it breaks.
   Avoid any question whose only good answer is yes.
5. **Evidence rules.** Ask for a doc URL on every runtime claim, and for
   UNVERIFIED on anything the docs do not settle. Set a length cap.

## Read the result

- Where it agrees with you, note it as independent confirmation.
- Where it diverges, the divergence is the finding. Weigh it on its
  evidence, never on whose idea it was.
- Its UNVERIFIED marks are a list of facts to cite before anything is built
  on them.
- A result that only restates your brief means the brief leaked your answer.
  Re-brief.

## Variants

- **Name test.** Give bare names plus a one-paragraph definition. Ask the
  reviewer to guess each meaning and flag confusion. A guess that diverges
  from the real meaning is the rename evidence.
- **Pattern review.** Give the base unit and ask for its axes, its uses
  ranked, a name and its risks.
