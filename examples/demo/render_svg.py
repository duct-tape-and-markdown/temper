#!/usr/bin/env python3
"""Render captured `temper` output into a terminal-card SVG — the README hero asset.

The demo is a *projection* of the tool's real output (specs/intent/55-offering.md, the
hero-visual Decision), never a hand-curated screenshot. This script reads the exact
bytes `temper check` emitted on stdin and paints them into a dark terminal card,
colouring each line by miette's own diagnostic structure (the severity glyph, the
`help:` line, the colocated guidance). It invents no text: every character shown is
the tool's. Deterministic — no timestamps, no randomness — so re-running over
unchanged output reproduces an identical SVG.

Usage: temper check --harness <path> | render_svg.py "<caption command>" > demo.svg
"""

import sys
from xml.sax.saxutils import escape

# A dark terminal palette (GitHub-dark-adjacent) — one system, severity-carrying.
BG = "#0d1117"
BAR = "#161b22"
FG = "#c9d1d9"
DIM = "#8b949e"
GREEN = "#3fb950"
RED = "#f85149"
YELLOW = "#d29922"

FONT = "ui-monospace, 'SF Mono', 'Cascadia Code', Menlo, Consolas, monospace"
FONT_SIZE = 14
CHAR_W = 8.4      # advance width of the monospace font at FONT_SIZE
LINE_H = 20
PAD = 20
BAR_H = 32


def spans(text, color):
    """One coloured tspan carrying `text` (already the literal output bytes)."""
    return f'<tspan fill="{color}">{escape(text)}</tspan>'


def render_line(raw):
    """Map one raw output line to (coloured tspans, plain-text length).

    Faithful to miette's terminal rendering: the ASCII severity markers the String
    reporter emits (`x`, `!`) are shown as the `×` / `⚠` glyphs a real TTY draws,
    the diagnostic code is severity-coloured, and the `help:`/guidance prose is
    dimmed — the same visual a user sees when they run the command themselves.
    """
    stripped = raw.strip()
    # A blank separator line between diagnostics.
    if stripped == "":
        return "", 0
    # An indented finding line: `  x <message>` (error) or `  ! <message>` (warn).
    if raw.startswith("  x "):
        msg = raw[4:]
        return f'  {spans("×", RED)} {spans(msg, FG)}', len(raw)
    if raw.startswith("  ! "):
        msg = raw[4:]
        return f'  {spans("⚠", YELLOW)} {spans(msg, FG)}', len(raw)
    # The help line and its wrapped guidance continuation — dimmed prose.
    if raw.lstrip().startswith("help:") or raw.startswith("        "):
        return spans(raw, DIM), len(raw)
    # A bare, unindented token is the diagnostic code (e.g. `forbidden_keys`).
    if raw == stripped and " " not in stripped:
        return spans(raw, RED), len(raw)
    return spans(raw, FG), len(raw)


def main():
    caption = sys.argv[1] if len(sys.argv) > 1 else "temper check --harness ."
    body = sys.stdin.read().rstrip("\n")
    raw_lines = body.split("\n")

    # The command prompt above the output — context for the projection.
    prompt = f'{spans("$", GREEN)} {spans(caption, FG)}'
    lines = [(prompt, len(caption) + 2), ("", 0)]
    for raw in raw_lines:
        lines.append(render_line(raw))

    max_len = max((length for _, length in lines), default = 0)
    width = int(PAD * 2 + max_len * CHAR_W)
    height = BAR_H + PAD * 2 + len(lines) * LINE_H

    out = []
    out.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
        f'viewBox="0 0 {width} {height}" font-family="{FONT}" font-size="{FONT_SIZE}">'
    )
    out.append(f'<rect width="{width}" height="{height}" rx="8" fill="{BG}"/>')
    out.append(f'<rect width="{width}" height="{BAR_H}" rx="8" fill="{BAR}"/>')
    out.append(f'<rect y="{BAR_H - 8}" width="{width}" height="8" fill="{BAR}"/>')
    # The three traffic-light dots — the terminal-window signifier.
    for i, dot in enumerate(("#ff5f56", "#ffbd2e", "#27c93f")):
        out.append(f'<circle cx="{20 + i * 20}" cy="{BAR_H // 2}" r="6" fill="{dot}"/>')

    y = BAR_H + PAD + FONT_SIZE
    for tspans, _ in lines:
        if tspans:
            out.append(
                f'<text x="{PAD}" y="{y}" xml:space="preserve">{tspans}</text>'
            )
        y += LINE_H
    out.append("</svg>")
    sys.stdout.write("\n".join(out) + "\n")


if __name__ == "__main__":
    main()
