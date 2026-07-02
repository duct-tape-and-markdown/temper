# Contributing to temper

Thanks for looking. This is a small, focused project with a single maintainer,
so this document sets expectations rather than inviting co-maintenance. The most
useful contribution is a good bug report; the second is a well-scoped fix for
something already discussed.

## How this codebase is authored

`temper` is largely **agent-built under human-authored specs**. The intent lives
in [`specs/`](../specs/); a build pipeline ships it to the trunk one gated,
validated commit at a time. Provenance is recorded in each commit's trailer, and
the full history is the audit trail. This is stated plainly because it is the
project's own thesis: the gate exists precisely because agents author harnesses,
and a project that discloses its own authorship has standing to ask the same of
contributors (see the AI-assisted contributions section below).

## Filing a good issue

- **Search first.** Check open and closed issues before opening a new one — the
  answer or the tracking issue may already exist.
- **Use the forms.** Bug reports and feature requests go through the issue forms,
  which ask for the version, OS, and reproduction we need to act. Blank issues
  are disabled on purpose.
- **A bug report reproduces.** The smallest set of files and the exact command
  that shows the wrong behavior is worth more than a paragraph describing it.

## Pull requests

- **Discussion precedes code.** Open or find an issue first and get agreement on
  the approach before writing a PR. Unsolicited large PRs are hard to accept and
  usually will not be — not because the work is unwelcome, but because a change
  nobody agreed to the shape of costs more to review than to write.
- **Keep it scoped.** One change per PR, matching the surrounding code's style,
  with the gates green (`cargo fmt`, `cargo clippy -D warnings`, `cargo test`).

## AI-assisted contributions

AI assistance is **welcome, with disclosure** — not banned. A ban would be
hypocrisy in an agent-built repo. But disclosure is required, and one rule
follows from it:

- **Note in the PR that you used an assistant**, and which parts.
- **You must understand and be able to defend the change without it.** If a
  reviewer's question sends you back to the assistant to answer, the change is
  not ready. Put another way: if the human effort is less than the review effort
  it imposes, please don't submit it — that asymmetry lands entirely on the
  maintainer.

This is the same bar the project holds itself to. Unverified, assistant-generated
output submitted as if understood is the fastest way to have a contribution
declined.
