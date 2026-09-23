# RunenGraph normative specification

This directory is the sole normative authority for RunenGraph graph and
relationship semantics. Repository architecture, implementation, tests, issues,
and downstream integrations may explain or realize this contract but do not
create competing semantic rules.

## Requirement language

The keywords MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY express normative
strength:

- MUST / MUST NOT: required for conformance;
- SHOULD / SHOULD NOT: expected unless a stronger accepted reason is documented
  and conformance remains intact;
- MAY: permitted but not required.

Requirement identifiers are stable semantic references within this repository.
Changing an accepted requirement requires ordinary reviewed specification work;
implementation convenience alone does not change its meaning.

## Decision states

Normative sections use these states when useful:

- Accepted: part of the current RunenGraph semantic contract.
- Open: intentionally unresolved; no implementation choice is authorized by the
  absence of a decision.
- Deferred: outside the current contract and requires later accepted work before
  becoming supported.

An Open or Deferred item is not a generic extension point and does not grant an
implementation freedom to publish incompatible public semantics.

## Specification map

- [Semantic model](semantic-model.md) — R0 identity, graph membership,
  directed and symmetric relationships, mutation, failure, and deterministic
  observation semantics.
