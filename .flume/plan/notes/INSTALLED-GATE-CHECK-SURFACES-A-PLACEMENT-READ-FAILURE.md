# Installed-gate check surfaces a placement read failure

Shipped as named. Now, when `gate_installed` hits an `Err` from `evaluate_placements` or `place_settings_only`, it returns one `warn` diagnostic under `install.gate-installed`: "temper's gate could not be evaluated — <error chain>". It no longer returns an empty vec. The level stays `warn` so the doc's "always warn, never error" contract holds. Whether a gate that cannot be evaluated should be `error` is open if plan wants it raised.

The test makes the projection unreadable by putting a directory where the file was. A chmod would not work as root.
