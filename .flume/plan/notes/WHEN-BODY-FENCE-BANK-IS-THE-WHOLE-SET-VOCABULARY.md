# The predicate sample banks are held whole

`set_predicates()` now samples `reached-from`, so the four pins looping it
(admissible-anywhere, one-grain, when-body refusal, indeterminate-arm fence)
cover it. `vocabulary_slot` in engine.rs tests is an exhaustive match over
`Predicate`; `the_sample_banks_are_the_whole_vocabulary` asserts every slot is
sampled by a bank (or the fenced `dependency-exists`). A new variant fails to
compile until slotted, and the test fails until sampled. `VOCABULARY` (33) is a
hand count beside the match; the test catches it running behind, not ahead.
