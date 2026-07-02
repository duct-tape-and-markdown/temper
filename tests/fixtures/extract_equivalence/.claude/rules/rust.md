---
paths:
  - "src/**/*.rs"
  - "tests/**/*.rs"
---
# Rust conventions

Prefer a clone over a lifetime fight. This tool is I/O-bound over kilobyte
files; zero-copy buys nothing and costs readability.

## Errors & diagnostics

Model errors with `thiserror`, surface them with `miette`.
