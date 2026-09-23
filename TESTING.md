# Testing and validation

## Canonical command

    cargo validate

This command is implemented by the repository-local xtask and is the single
RunenGraph merge-readiness baseline.

## Baseline checks

Validation fails closed when:

- a required RunenGraph authority or R0 specification file is missing;
- active template identity remains in the repository;
- package and license metadata do not represent RunenGraph and GPL-3.0-only;
- the GPLv3 license or historical licensing record is incomplete;
- the normative specification loses its sole-authority declaration or required
  R0 requirement anchors;
- normative spec content links outside the spec authority;
- Rust formatting is not clean;
- workspace tests fail;
- Clippy emits warnings;
- rustdoc emits warnings;
- Git whitespace checks fail; or
- validation changes repository state.

## R0 conformance

tests/r0_conformance.rs exercises only the public crate surface and proves:

- caller-owned identity and isolated membership;
- directed orientation;
- symmetric orientation independence and canonical pair representation;
- relationship set/idempotence semantics;
- explicit self-relationship policy;
- deterministic endpoint-validation precedence and atomic rejection;
- atomic incident-relationship cleanup on node removal;
- deterministic node, relationship, and direct-adjacency ordering;
- the distinction between an unknown node and an admitted isolated node;
- insertion/removal agreement on endpoint and self-policy validation.

These tests are conformance evidence for the accepted spec. They do not replace
the normative specification and do not authorize deferred capabilities.

## CI and exact-head evidence

.github/workflows/validation.yml is intentionally thin. It pins the accepted
Dornglut reusable Rust validation workflow to an immutable commit and delegates
meaning to cargo +stable validate.

Independent repository-owned CI must validate the exact reviewed feature head
before merge. Accepted-main push validation verifies the exact squash result
after merge.

Focused commands may be useful during implementation but do not replace cargo
validate.
