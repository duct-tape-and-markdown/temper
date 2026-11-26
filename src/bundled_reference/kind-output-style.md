<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `output-style` kind

The kind declares no counsel.

## Locus

Each member is its own document, at:

- `**/.claude/output-styles/*.md`

## Edges

### In

- `settings`.`outputStyle` → here: no intent declared.

## Clauses

- `unshadowed` (advisory): A bare `outputStyle` name resolves to the style nearest the directory a session starts in, so a nested style sharing a shallower style's name replaces it for every session started at or below the nested folder. Rename one of them if both should stay selectable there; keep the shared name if the override is the point.
