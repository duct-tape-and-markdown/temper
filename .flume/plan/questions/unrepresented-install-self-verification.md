# Does `check` verify an unrepresented install's gate?

`specs/distribution.md`, "The placements and their enforcement modes": "`check` verifies its own gate is installed and undrifted." `gate_installed` (`src/install.rs`) returns nothing unless `.temper/` exists. An unrepresented install never creates `.temper/`, so on the tree it installed, `check` verifies neither the `SessionStart` hook nor the CDN modelines. The verdict is vacuous there.

The corpus allows two readings:
- `check` judges the unrepresented gate whenever it finds a temper `SessionStart` hook seated: stale is a finding, missing is silence. This needs no state, but a hook that was removed passes.
- `check` judges it on every tree that holds no program: a missing hook is a finding. But then a bare stranger `check` on a never-installed harness warns about a gate nobody installed, and "no invisible state" rules out a marker recording that install ran.

If the gate is verified, a further question: does it include the modelines? That requires a discovery walk inside `check`.

Raised by build note UNREPRESENTED-INSTALL-PLACES-THE-CDN-MODELINE.
