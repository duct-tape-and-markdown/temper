# Example source kind retired to inputs; projections re-emitted from scratch

Shipped: the `source` kind, its binding, and `system`'s `implemented-by`
edge field are gone from examples/base-harness. scanner/renderer declare
`src/scan.js`/`src/render.js` as inputs, and verify-summary declares
`src/main.js` (the `source:main` mention is now plain prose). `system`
dropped `format: "yaml-frontmatter"` because no typed field is left.

This time docs/systems/*.md were deleted and re-emitted in a temp copy
(SDK vendored there, never in the checkout), and the lock was taken from
that same emit. The scanner row is now 22ab…, not the stale 53cd…. Both
shipped_example tests and layout_kind pass on the committed tree. Both
README demos (root.fresh on an edited src/scan.js, dangling_import on a
deleted src/main.js) were re-run against the built binary and match.

The stale `# temper:` note carried on overwrite is still unfixed. It is
tracked as EMIT-DROPS-A-NOTE-THE-NEW-SHAPE-CANNOT-HOLD.
