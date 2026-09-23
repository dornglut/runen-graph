# RunenGraph agent guide

## Start here

Read `README.md`, `ARCHITECTURE.md`, `TESTING.md`, and `BOOTSTRAP.md` before
changing the repository. The cross-repository boundary is established by
[Engineering ADR 0010](https://github.com/dornglut/engineering/blob/main/adrs/0010-establish-runen-graph-boundary.md).

## Scope

`dornglut/runen-graph` is a standalone Rust framework authority for reusable
graph and relationship semantics over caller-owned identities. This revision is
the repository bootstrap baseline. It does not define or implement graph
semantics.

## Rules

- Keep the repository greenfield and bootstrap-bounded until a separate
  RunenGraph semantic-kernel issue is accepted.
- Do not transfer, copy, or migrate graph source from Runenwerk.
- Do not add graph types, algorithms, storage, backends, examples, semantic
  specifications, consumer integrations, or third-party graph dependencies.
- Keep one product package plus the local `xtask`; do not introduce speculative
  crate splits.
- RunenGraph may be depended on by explicit consumers, but it must not depend
  upward on RunenECS, RunenUI, Runenwerk, RunenRender, or future RunenKnowledge.
- Preserve the `unsafe_code = "forbid"` baseline and stable minimal toolchain.
- Contribution classification is `owner-only` until an accepted inbound
  mechanism preserves commercial relicensing rights.
- Keep `cargo validate` as the canonical repository-owned command.
- Keep CI a thin read-only caller of repository-owned validation.

## Required workflow

1. Read the current repository authority and the accepted Engineering boundary.
2. Keep changes bounded to the accepted RunenGraph-owned issue.
3. Run `cargo validate` from a clean checked-out Rust executor when available.
4. Validate the exact reviewed feature head through repository-owned CI.
5. Reconcile the complete final diff with current `main` and merge only the
   exact reviewed head.

The bootstrap record is historical provenance. It is not an ongoing template
guidance document or a synchronization mechanism.
