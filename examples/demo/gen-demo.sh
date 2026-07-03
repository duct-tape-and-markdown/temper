#!/usr/bin/env bash
# Regenerate the README hero asset (examples/demo/demo.svg).
#
# The capture pipeline behind specs/intent/55-offering.md's hero-visual Decision: the demo
# is a *projection* of `temper`'s real output, never a hand-curated screenshot that
# drifts. It runs the shipped binary over the committed demo harness and renders the
# exact diagnostic it emits into a terminal-card SVG. Rerun whenever the output
# changes; the SVG is deterministic, so an unchanged run reproduces identical bytes.
#
# The demo harness (examples/demo/harness/) is a realistic, gate-installed Claude
# Code project carrying one real bug: a rule authored with Cursor's `globs` key,
# which Claude Code silently ignores — the exact silent-inert failure temper hunts.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../.." && pwd)"
cd "$root"

harness="examples/demo/harness"
caption="temper check --harness $harness"

cargo build --quiet

# `check` exits non-zero on findings — that is the whole point of the demo — so the
# non-zero status is expected and must not abort the capture.
output="$(./target/debug/temper check --harness "$harness" || true)"

printf '%s\n' "$output" | python3 "$here/render_svg.py" "$caption" > "$here/demo.svg"
echo "wrote $harness/../demo.svg ($(wc -l < "$here/demo.svg") lines)"
