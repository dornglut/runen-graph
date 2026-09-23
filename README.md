# RunenGraph

dornglut/runen-graph is a standalone reusable graph and relationship framework
over caller-owned identities.

## Maturity

RunenGraph has an accepted R0 foundational semantic kernel for explicit graph
membership plus directed and symmetric structural relationship sets. The
normative contract is under [spec/](spec/README.md).

R0 deliberately excludes hierarchy, cardinality schemas, payloads, graph
algorithms, persistence, multi-operation transactions, and consumer integration.
Those capabilities require separately accepted work.

RunenGraph remains greenfield. It is not an extraction of Runenwerk graph source,
and existing graph-shaped systems are not automatically RunenGraph consumers.

## R0 public surface

The public crate currently provides:

- DirectedGraph over caller-owned ordered keys;
- SymmetricGraph over caller-owned ordered keys;
- explicit node admission and atomic node removal;
- explicit allow/forbid self-relationship policy;
- set-valued relationship insertion/removal;
- typed relationship-mutation rejection;
- deterministic node, relationship, and direct-adjacency observation.

The public identity remains the caller key. RunenGraph does not issue a public
node or edge identifier in R0.

## Repository authority

- [Normative specification](spec/README.md)
- [Architecture](ARCHITECTURE.md)
- [Testing and validation](TESTING.md)
- [Bootstrap provenance](BOOTSTRAP.md)
- [Agent guide](AGENTS.md)
- [Engineering ADR 0010](https://github.com/dornglut/engineering/blob/main/adrs/0010-establish-runen-graph-boundary.md)
- [Runen-family architecture](https://github.com/dornglut/engineering/blob/main/architecture/runen-family.md)

## Validation

The canonical repository-owned merge-readiness command is:

    cargo validate

## License

The current RunenGraph revision is available under the [GNU General Public
License v3.0-only](LICENSE). The separate commercial licensing path and the
historical template-license provenance are documented in [LICENSING.md](LICENSING.md).
