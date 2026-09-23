# R0 semantic model

Status: Accepted

R0 defines the foundational relationship layer. It deliberately does not define
hierarchy, cardinality schemas, edge payloads, graph algorithms, persistence, or
consumer integration.

## Caller-owned identity

**RG-ID-001 — Caller key authority.**
A RunenGraph graph MUST use the caller-supplied node key as the public identity
of an admitted node.

**RG-ID-002 — No graph-issued public node identity.**
RunenGraph MUST NOT require or expose a second public node identifier that
replaces the caller key.

**RG-ID-003 — Key equality and order.**
The caller key's equality and total ordering define structural identity equality
and canonical public observation order. That order MUST NOT be interpreted by
RunenGraph as domain priority, execution order, hierarchy order, or insertion
order.

The R0 Rust realization requires caller keys implementing Ord and Clone. Clone
permits the implementation to retain relationship endpoints without transferring
identity authority away from the caller key.

## Graph membership

**RG-MEM-001 — Explicit admission.**
A node key MUST be explicitly admitted before a relationship may reference it.

**RG-MEM-002 — Isolated nodes.**
An admitted node MAY exist without any relationship.

**RG-MEM-003 — Idempotent node admission.**
Admitting a node key already present MUST report an unchanged outcome and MUST
NOT create a second node.

**RG-MEM-004 — Node removal closure.**
Removing an admitted node MUST atomically remove that node and every relationship
incident to it. This operation changes only RunenGraph structural state; it MUST
NOT imply deletion or mutation of the caller-owned object represented by the key.

**RG-MEM-005 — Absent node removal.**
Removing a node that is not admitted MUST report that no node was removed and
zero relationships were removed.

## Relationship-set ownership

**RG-REL-001 — One relationship set per graph instance.**
Each R0 graph instance represents one structural relationship set over its
admitted nodes. RunenGraph does not assign domain meaning to that relationship
set.

**RG-REL-002 — No R0 relationship registry.**
R0 MUST NOT require a global or per-graph registry of named relationship kinds.
A consumer may give an instance domain meaning through its own type or field
ownership.

**RG-REL-003 — Set semantics.**
For a graph instance and endpoint pair, at most one relationship exists.
Re-inserting an existing relationship MUST report an unchanged outcome.

**RG-REL-004 — No public edge identity.**
R0 MUST NOT expose a graph-issued public edge identifier. Endpoint identity and
the graph instance identify an R0 relationship.

## Directed relationships

**RG-DIR-001 — Orientation.**
In a DirectedGraph, relationship (A, B) means source A relates to target B.
Relationship (A, B) and relationship (B, A) are distinct unless A equals B.

**RG-DIR-002 — Canonical observation.**
Directed relationships MUST be observed in lexicographic (source, target) caller
key order.

**RG-DIR-003 — Direct adjacency.**
Outgoing adjacency MUST expose targets related from the requested source in
ascending caller key order. Incoming adjacency MUST expose sources related to
the requested target in ascending caller key order.

## Symmetric relationships

**RG-SYM-001 — Orientation independence.**
In a SymmetricGraph, relationship between A and B is identical to relationship
between B and A.

**RG-SYM-002 — Canonical endpoint pair.**
A symmetric relationship MUST be publicly represented as (min(A, B), max(A, B))
under caller key order.

**RG-SYM-003 — Canonical observation.**
Canonical symmetric relationship pairs MUST be observed in lexicographic pair
order.

**RG-SYM-004 — Direct adjacency.**
Neighbor observation MUST expose directly related node keys in ascending caller
key order. A permitted self relationship contributes that node exactly once to
its own neighbor observation.

## Self relationships

**RG-SELF-001 — Explicit policy.**
Every R0 graph instance MUST be created with either Allow or Forbid
SelfRelationshipPolicy. The public API MUST NOT silently choose a default.

**RG-SELF-002 — Immutable policy.**
A graph's self-relationship policy MUST remain fixed for that graph instance.

**RG-SELF-003 — Forbidden self mutation.**
When policy is Forbid, insertion or removal of relationship (A, A) MUST fail with
SelfRelationshipForbidden after endpoint admission has been validated.

**RG-SELF-004 — Allowed self relationship.**
When policy is Allow, a relationship from A to A follows the same set semantics
as any other relationship.

## Mutation and failure

**RG-MUT-001 — Endpoint validation.**
Relationship insertion and removal MUST require both endpoint node keys to be
admitted. If the first call argument is absent, UnknownNode for that key MUST be
reported before inspecting the second endpoint for public failure selection. If
the first is present and the second is absent, UnknownNode for the second key
MUST be reported.

**RG-MUT-002 — Validation precedence.**
Endpoint admission MUST be validated before self-relationship policy.

**RG-MUT-003 — Atomic operation boundary.**
Every public mutation MUST be atomic at its operation boundary. A rejected
operation MUST leave the complete observable graph state equal to its
pre-operation state.

**RG-MUT-004 — Idempotent relationship removal.**
Removing a valid admitted endpoint pair with no relationship present MUST report
an unchanged outcome.

**RG-MUT-005 — No R0 multi-operation transaction.**
R0 defines no transaction spanning multiple public mutation calls.

## Observation

**RG-OBS-001 — Node order.**
Node iteration MUST use ascending caller key order and MUST be independent of
node insertion order.

**RG-OBS-002 — Relationship membership observation.**
Relationship membership observation MUST be non-failing. If either endpoint is
not admitted, the relationship MUST be observed as absent.

**RG-OBS-003 — Missing versus isolated adjacency.**
Direct adjacency lookup MUST distinguish an unknown node from an admitted node
with no direct relationships. The Rust realization uses None for the unknown
node and an empty iterator for the admitted isolated node.

**RG-OBS-004 — Storage independence.**
No public observation order or validity rule may depend on incidental hash
iteration, allocator state, internal indices, or another private storage detail.

## Deferred capabilities

The following are Deferred and are not part of R0:

- multiple relationship kinds inside one graph instance;
- graph-issued node or edge identities;
- edge payloads or parallel relationships;
- relationship cardinality schemas or exclusivity;
- hierarchy and ordered-child semantics;
- acyclicity, transitivity, or traversable traits;
- traversal beyond direct adjacency, including BFS, DFS, paths, and topological
  ordering;
- multi-operation structural transactions;
- change history or journals;
- serialization and persistence formats;
- reflection and proc macros;
- query languages and ports;
- backend-library commitments;
- RunenECS, RunenUI, Runenwerk, RunenRender, or RunenKnowledge integration.

Deferred capabilities require separately accepted RunenGraph or consumer-owned
work. Their absence does not authorize an implementation-defined public
extension.
