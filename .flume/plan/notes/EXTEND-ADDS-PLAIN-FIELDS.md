# extend places plain fields: region or render

Shipped in `sdk/src/kind.ts`: `KindExtension.fields` (field -> "region" |
"render") and `render(values) => Prose`. `placedBody` checks each placement
when `extend` runs. Construction drops placed fields from `Member.fields`
and builds the body through the render. The placement is held on
`KindDefinition.body`, so a re-extension keeps it and composes renders (the
base's render runs first and becomes the outer render's `prose`).

Gaps plan should weigh:
- "region" is admitted only where the base's layout declares a `field` slot
  of that name. A layout slot is `keyof` the base's own T, so in practice a
  region can place only a field the base already types. A genuinely new
  field in a body region would need a layout delta. That needs a ruling and
  is not built.
- A plain field that repeats a base field is caught only for framework
  keys, identity, edge fields and earlier plain fields. Erased base facts
  carry no field list (rule declares no `leaves`), so a redeclared `paths`
  passes.
- The subkind's `leaves` witness still rides the base's unchanged. Plain
  fields never reach the lock row, which the acceptance asks for, but
  `schema`/`explain` will not list them.
- A render-only delta (no field placed) is refused. A body template by
  itself is not what this face is for.
- The engine has no idea placed fields exist. `check` cannot gate `owner`
  until the body's form is typed (a layout), so for now the render's output
  is just prose to the gate.
