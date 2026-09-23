# RunenGraph bootstrap provenance

This document records how the standalone RunenGraph repository was established.
It is historical provenance for the one-time bootstrap and is not active
template guidance or a synchronization mechanism.

## Source and generated revision

The repository was generated from the accepted `dornglut/rust-framework-template`
`main` revision `500461d51fe155febc806e288e5bc013e413a785`.

The generated RunenGraph repository began at initial revision
`f395f2bc2839b56a18c1a4bff4f6b77625b7209c`. Its initial tree matched the
template tree and therefore represented bootstrap provenance only; it did not
represent RunenGraph semantic implementation.

## Product identity decisions

- Repository: `dornglut/runen-graph`
- Package: `runen-graph`
- Crate: `runen_graph`
- Version: `0.0.0`
- Edition: Rust 2024
- Published package: no
- Profile: `rust-framework`
- Lifecycle: `active`
- Contribution: `owner-only`
- Canonical validation: `cargo validate`

RunenGraph is a standalone greenfield framework authority for reusable graph
and relationship semantics over caller-owned identities. No graph semantics,
consumer relationship, or source transfer was accepted as part of bootstrap.

## License transition

The generated initial revision carried the Rust framework template's Apache-2.0
license and associated template grant. The current RunenGraph product
representation is prospectively GPL-3.0-only, with the separate commercial path
described in `LICENSING.md`.

The product transition does not revoke, narrow, or reinterpret rights granted on
the template-origin revision. The historical Apache-2.0 grant remains historical
provenance for that revision. Third-party material, if later incorporated,
retains its own licensing obligations.

## Toolchain posture

RunenGraph uses the stable Rust toolchain with the existing minimal profile and
`rustfmt`/`clippy` components. No MSRV is claimed because current product
evidence does not establish one. The `unsafe_code = "forbid"` baseline is
preserved.

## Intentional deviations from the template

- Active repository, package, crate, workflow, and documentation identity was
  replaced with RunenGraph identity.
- Product licensing was changed from the template's Apache-2.0 baseline to the
  GPL-3.0-only product-library class.
- `LICENSING.md` was added for the separate commercial licensing path and
  historical template grant.
- The template's `rust-version` claim was removed rather than adopted as an
  MSRV.
- Validation guards now reject stale active template identity and enforce the
  RunenGraph licensing representation.
- The repository profile, lifecycle, contribution classification, and GitHub
  bootstrap settings were reconciled to the current rust-framework target.

No graph types, examples, semantic specification, backend, third-party graph
dependency, roadmap, status document, or consumer integration was created.
