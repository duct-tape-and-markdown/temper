# `temper emit .` errors where `temper check .` takes the path

From Cascade (2026-11-25). `check` takes the harness path positionally, while `emit` takes none (only `--into`), so `temper emit .` errors. Two verbs over one harness spell its location two ways. Give `emit` the positional harness path `check` takes (resolving `.temper` under it), keep `--into` as is, and refuse both together only where they disagree.
