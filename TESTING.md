# Testing and validation

## Canonical command

```text
cargo validate
```

This command is implemented by the repository-local `xtask` and is the
RunenGraph merge-readiness baseline.

## Bootstrap checks

Validation fails closed when:

- a required RunenGraph authority file is missing;
- active template identity remains in the repository;
- package and license metadata do not represent RunenGraph and GPL-3.0-only;
- the GPLv3 license or historical licensing record is incomplete;
- Rust formatting is not clean;
- workspace tests fail;
- Clippy emits warnings;
- rustdoc emits warnings;
- Git whitespace checks fail; or
- validation changes repository state.

The validator starts from a clean repository and verifies that the repository
remains unchanged after the checks.

## CI

`.github/workflows/validation.yml` is intentionally thin. It pins the accepted
`dornglut/github-workflows` reusable Rust validation workflow to an immutable
commit and delegates meaning to `cargo +stable validate`.

Independent repository-owned CI must validate the exact reviewed feature head
before merge. Accepted-main push validation verifies the exact squash result
after merge.

## Current boundary

No graph-specific test matrix, conformance workload, backend proof, consumer
integration, or semantic behavior is part of this bootstrap baseline. Those
checks require separately accepted RunenGraph-owned work.
