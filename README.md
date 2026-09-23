# RunenGraph

`dornglut/runen-graph` is a standalone reusable graph and relationship
framework over caller-owned identities.

## Maturity

This repository is at bootstrap maturity. The repository authority, validation
baseline, licensing representation, and ownership boundary are established, but
the graph semantic model and public API have not yet been designed or
implemented.

RunenGraph is greenfield. It is not an extraction of Runenwerk graph source,
and existing graph-shaped systems are not automatically RunenGraph consumers.

## Repository authority

- [Architecture](ARCHITECTURE.md)
- [Testing and validation](TESTING.md)
- [Bootstrap provenance](BOOTSTRAP.md)
- [Agent guide](AGENTS.md)
- [Engineering ADR 0010](https://github.com/dornglut/engineering/blob/main/adrs/0010-establish-runen-graph-boundary.md)
- [Runen-family architecture](https://github.com/dornglut/engineering/blob/main/architecture/runen-family.md)

## Validation

The canonical repository-owned merge-readiness command is:

```text
cargo validate
```

## License

The current RunenGraph revision is available under the [GNU General Public
License v3.0-only](LICENSE). The separate commercial licensing path and the
historical template-license provenance are documented in [LICENSING.md](LICENSING.md).
