## Surface

`sdk/src/prose.ts`'s `defersToGate` (:78-80) reads the host-address grammar by
hand — `address.includes("/")`, then `indexOf(":")` with a `colon > 0` test —
beside the module that now owns it (`sdk/src/member-address.ts`'s
`parseHostAddress`). SDK-MEMBER-ADDRESS-GRAMMAR-ONE-HOME consolidated the
twenty-one *writers*; this reader stayed out because routing it through the
parser is a behaviour change, not an extraction: `parseHostAddress` refuses
`kind:` (an address names exactly one thing or it names nothing), while
`defersToGate` today defers on it, so a mention of `rule:` would move from
deferring to `check` to a loud emit refusal.

The Rust half has the matching shape and no such holdout — every reader there
routes through `src/member_address.rs` (`rg "split_once(':')" src/` hits it and
nowhere else).

## Observed at

d6b1740f (HEAD when observed)

## Suggested consolidation

`defersToGate` reads `parseHostAddress`, and the entry that does it owns the
verdict on `kind:`: refusing a name-less address at emit is the reading both
ends of the seam already take, so the fix is one line plus the refusal test
that pins the new verdict.
