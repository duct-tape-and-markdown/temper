---
name: coordinate
description: Use when driving a complex task through an agent team — parallel exploration across independent axes converging on a shared artifact, with an adversarial reviewer gate. NOT for single-axis or research-only work.
version: "0.3.0"
license: MIT
allowed-tools: ["Task", "Read", "Edit"]
---
# Coordinate

Drive a complex task through a team of agents. See the playbook for the full
reference on phases, fan-out, and the reviewer gate.

## When to use

Parallel exploration across three or more independent axes, converging on a
shared artifact, with an adversarial reviewer gate before shipping.

## Example

```sh
# not a heading — a shell comment inside a fence
temper coordinate --agents 6
```

### Caveats ###

NOT for single-axis or research-only work.
