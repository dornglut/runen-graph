# RunenGraph agent guide

## Start here

Read README.md, ARCHITECTURE.md, TESTING.md, spec/README.md,
spec/semantic-model.md, and BOOTSTRAP.md before changing the repository. The
cross-repository boundary is established by Engineering ADR 0010.

## Scope

dornglut/runen-graph is the standalone Rust framework authority for reusable
graph and relationship semantics over caller-owned identities.

R0 establishes explicit graph membership, directed and symmetric structural
relationship sets, explicit self-relationship policy, atomic operation-level
mutation, typed rejection, and deterministic direct observation.

The normative contract lives under spec/. Implementation and tests realize that
contract but do not silently extend it.

## Rules

- Preserve caller-owned public identity; do not introduce a graph-issued public
  node or edge identity without separately accepted specification work.
- Do not transfer, copy, or migrate graph source from Runenwerk.
- Do not add RunenECS, RunenUI, Runenwerk, RunenRender, or future
  RunenKnowledge dependencies.
- Do not add hierarchy, cardinality schemas, edge payloads, graph algorithms,
  multi-operation transactions, persistence, reflection, macros, query
  languages, ports, or backend dependencies without issue-owned authorization.
- Keep one product package plus the local xtask unless a concrete accepted
  boundary requires a split.
- Keep normative spec files self-contained; external sources may inform issues
  and research but do not become implicit semantic authority.
- Preserve unsafe_code = "forbid" and the stable minimal toolchain.
- Contribution classification remains owner-only until an accepted inbound
  mechanism preserves commercial relicensing rights.
- Keep cargo validate as the canonical repository-owned command.
- Keep CI a thin read-only caller of repository-owned validation.

## Required workflow

1. Read current repository authority and the accepted Engineering boundary.
2. Re-resolve current main and the owning issue before implementation.
3. Keep changes bounded to the accepted RunenGraph-owned issue.
4. Update specification before or with implementation when semantic behavior
   changes.
5. Run cargo validate from a clean Rust executor when available.
6. Validate the exact reviewed feature head through repository-owned CI.
7. Reconcile the complete final diff with current main and merge only the exact
   reviewed head.

BOOTSTRAP.md is historical provenance, not active design guidance.
