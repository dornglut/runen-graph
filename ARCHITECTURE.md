# Architecture

## Boundary

RunenGraph is the standalone foundational Runen-family framework authority for
reusable graph and relationship semantics over caller-owned identities. It is
greenfield and does not receive source authority from Runenwerk's graph-shaped
systems.

Engineering ADR 0010 owns the cross-repository boundary. The normative semantic
contract lives under [spec/](spec/README.md).

## Semantic versus realization authority

The R0 specification owns caller-key identity, explicit graph membership,
directed and symmetric relationship meaning, self-relationship policy, mutation
atomicity, failure selection, and deterministic public observation.

The Rust crate realizes that contract. Private storage layout, collection choice,
module layout, allocation, and implementation complexity do not define semantics
and may change without changing the normative contract.

R0 uses no third-party graph backend or Runen-family production dependency.

## Current repository topology

The repository has one non-published product package and one local validation
package:

    repository
    ├── root package: runen-graph / runen_graph
    │   ├── public R0 semantic surface
    │   ├── private ordered storage realization
    │   └── public-surface conformance tests
    └── xtask: repository-owned validation authority

No product crate split is established.

## R0 relationship model

Each graph instance owns one structural relationship set over explicitly admitted
caller keys.

DirectedGraph preserves source-to-target orientation. SymmetricGraph treats the
two endpoint orientations as the same relationship and publishes the canonical
ordered pair.

The caller key remains public identity. No public graph-issued node or edge
identity exists in R0.

The key's total order defines canonical structural observation order only. It does
not define application priority, execution order, hierarchy order, or insertion
order.

## Dependency direction

RunenGraph is an independent semantic authority. Explicit consumers may depend
on RunenGraph, but RunenGraph must not depend upward on RunenECS, RunenUI,
Runenwerk, RunenRender, or any future RunenKnowledge repository. Consumer
adoption and any integration correspondence are separate consumer-owned work.

The local validation package owns the meaning of cargo validate. Shared CI only
invokes that command and does not define product or validation semantics.

## Documentation authority

| Concern | Canonical location |
| --- | --- |
| normative graph/relationship semantics | spec/ |
| repository/system boundary | ARCHITECTURE.md |
| merge-readiness and evidence | TESTING.md plus repository validator |
| executor rules | AGENTS.md |
| public landing and navigation | README.md |
| bootstrap provenance | BOOTSTRAP.md |
| current licensing representation | LICENSE and LICENSING.md |
| implementation and tests | Rust source/tests when accepted |
| live work state | GitHub issues, pull requests, and Projects |

Do not create roadmap, status, ADR, report, or other documentation taxonomies
without a real separately owned need.
