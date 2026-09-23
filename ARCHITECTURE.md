# Architecture

## Boundary

RunenGraph is a standalone foundational Runen-family framework authority for
reusable graph and relationship semantics over caller-owned identities. It is
greenfield and does not receive source authority from Runenwerk's graph-shaped
systems.

This bootstrap revision establishes the repository boundary only. It does not
define the graph model, identity representation, relationship rules, storage,
algorithms, backend, or public semantic API.

## Repository topology

The repository has one non-published product package and one local validation
package:

```text
repository
├── root package: runen-graph / runen_graph
└── xtask: repository-owned validation authority
```

No speculative product crate split is established by the bootstrap.

## Dependency direction

RunenGraph is an independent semantic authority. Explicit consumers may depend
on RunenGraph, but RunenGraph must not depend upward on RunenECS, RunenUI,
Runenwerk, RunenRender, or any future RunenKnowledge repository. Consumer
adoption and any integration correspondence are separate consumer-owned work.

The local validation package owns the meaning of `cargo validate`. Shared CI
only invokes that command and does not define product or validation semantics.

## Documentation authority

The cross-repository boundary is owned by [Engineering ADR
0010](https://github.com/dornglut/engineering/blob/main/adrs/0010-establish-runen-graph-boundary.md)
and the [Runen-family architecture](https://github.com/dornglut/engineering/blob/main/architecture/runen-family.md).
This document is the concise repository-local boundary map. `README.md`,
`TESTING.md`, `BOOTSTRAP.md`, and `AGENTS.md` own their documented concerns.
