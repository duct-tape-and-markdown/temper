<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

#16 — CRLF checkouts: `frontmatter_matter`'s opening-delimiter test strips
only `---\n` (`src/frontmatter.rs:363`), so on a Windows checkout
(`core.autocrlf=true`, `---\r\n`) every frontmatter-carrying projection reads
as frontmatterless. Three cascades: `install::project_note` returns `None` and
the `.or_else` falls through to `project_banner`, whose own literal
`starts_with("---\n")` guard misses the same bytes and composes an HTML banner
ahead of the frontmatter block — `check` then reports false
`install.gate-installed` drift on every such target; `project_modeline`
silently never places or verifies the schema modeline; and
`placement::placement_lines` returns empty, so **`emit` deletes install's
managed-by note** from every frontmatter projection — content loss, not EOL
churn. The write path is shielded only by ordering: `run_represented` emits
(LF) before it evaluates placements. Field report (centercode-platform,
Windows 11, temper 0.0.13) diagnosed it as `install.rs`'s zero uses of
`canonicalize_eol`; that is a red herring — `desired` is derived from the
on-disk source, so its EOL already matches `real`, and `place`'s byte compare
is correct once detection works. Reproduced on a CRLF copy of this repo at
43743407: `check` names the 9 frontmatter targets (banner-form `CLAUDE.md` /
`collaboration.md` stay clean, `str::lines()` strips `\r`), and `temper emit`
alone drops the note line from those same 9. Entry
`FRONTMATTER-CRLF-OPEN-DELIMITER` filed directly, gate `open` — verify and
route it, don't re-derive. Adjacent, deliberately not filed: two homes for one
mechanic (`hash::canonicalize_eol(&[u8])` and `drift::normalize_lf(&str)`),
and this repo's `.gitattributes` pins only fixtures/snapshots to `eol=lf`, not
the harness surface a consumer's checkout would filter. observed at 43743407.

#16a — a second report from the same consumer files "`emit` strips the
managed-by note `install` places" as an independent defect, and proposes
moving note ownership into `emit` or declaring it in the lock. **Refuted, no
second entry.** It is cascade three of #16. Verified at 43743407 on an LF
copy: a real re-emit of a rule (authored body edited) rewrites the body and
leaves the note line untouched, `check` clean, no intervening `install`;
baseline is `0 emitted, 12 unchanged`. The report's own `41 emitted, 0
unchanged` is the CRLF tell — every projection differing from its LF desired
— and its "25 frontmatter kinds stripped, 17 frontmatterless survive" is
exactly #16's split, not two code paths of differing quality. `emit` already
preserves the note by design: `placement::placement_lines`, which the report
never found. Two true remainders, both folded into
`FRONTMATTER-CRLF-OPEN-DELIMITER`: `project_note`'s doc
(`src/install.rs:1646-1649`) states "the note rides `install`, never `emit`"
without naming the preservation half, which is what sent a careful reader
into a redesign; and the suite covers install→install but never
install→edit→emit→gate, so the composition had no pin at either EOL. Their
`.temper/README.md` "Changing it" procedure needs no change — it is correct
on LF and correct everywhere once #16 lands. observed at 43743407.
