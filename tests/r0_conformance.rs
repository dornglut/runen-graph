use runen_graph::{
    Change, DirectedGraph, NodeRemoval, RelationshipError, SelfRelationshipPolicy, SymmetricGraph,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Key(&'static str);

fn keys(values: &[&'static str]) -> Vec<Key> {
    values.iter().copied().map(Key).collect()
}

#[test]
fn directed_graph_preserves_caller_identity_and_direction() {
    let a = Key("a");
    let b = Key("b");
    let mut graph = DirectedGraph::new(SelfRelationshipPolicy::Forbid);
    assert_eq!(graph.insert_node(b.clone()), Change::Changed);
    assert_eq!(graph.insert_node(a.clone()), Change::Changed);
    assert_eq!(
        graph.nodes().cloned().collect::<Vec<_>>(),
        keys(&["a", "b"])
    );

    assert_eq!(graph.insert_relationship(&a, &b), Ok(Change::Changed));
    assert!(graph.contains_relationship(&a, &b));
    assert!(!graph.contains_relationship(&b, &a));
    assert_eq!(graph.insert_relationship(&a, &b), Ok(Change::Unchanged));
}

#[test]
fn symmetric_graph_is_orientation_independent_and_canonical() {
    let a = Key("a");
    let b = Key("b");
    let mut graph = SymmetricGraph::new(SelfRelationshipPolicy::Forbid);
    graph.insert_node(a.clone());
    graph.insert_node(b.clone());

    assert_eq!(graph.insert_relationship(&b, &a), Ok(Change::Changed));
    assert!(graph.contains_relationship(&a, &b));
    assert!(graph.contains_relationship(&b, &a));
    assert_eq!(graph.insert_relationship(&a, &b), Ok(Change::Unchanged));
    assert_eq!(
        graph
            .relationships()
            .map(|(left, right)| (left.clone(), right.clone()))
            .collect::<Vec<_>>(),
        vec![(a, b)]
    );
}

#[test]
fn self_relationship_policy_is_explicit() {
    let a = Key("a");
    let mut forbidden = DirectedGraph::new(SelfRelationshipPolicy::Forbid);
    forbidden.insert_node(a.clone());
    assert_eq!(
        forbidden.insert_relationship(&a, &a),
        Err(RelationshipError::SelfRelationshipForbidden(a.clone()))
    );
    assert_eq!(forbidden.relationship_count(), 0);

    let mut allowed = SymmetricGraph::new(SelfRelationshipPolicy::Allow);
    allowed.insert_node(a.clone());
    assert_eq!(allowed.insert_relationship(&a, &a), Ok(Change::Changed));
    assert_eq!(allowed.relationship_count(), 1);
    assert_eq!(
        allowed.neighbors(&a).unwrap().cloned().collect::<Vec<_>>(),
        vec![a]
    );
}

#[test]
fn endpoint_validation_is_deterministic_and_atomic() {
    let a = Key("a");
    let b = Key("b");
    let mut graph = DirectedGraph::new(SelfRelationshipPolicy::Forbid);

    assert_eq!(
        graph.insert_relationship(&a, &b),
        Err(RelationshipError::UnknownNode(a.clone()))
    );
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.relationship_count(), 0);

    graph.insert_node(a.clone());
    assert_eq!(
        graph.insert_relationship(&a, &b),
        Err(RelationshipError::UnknownNode(b.clone()))
    );
    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.relationship_count(), 0);
}

#[test]
fn rejected_removal_uses_the_same_validation_contract() {
    let a = Key("a");
    let b = Key("b");
    let mut graph = DirectedGraph::new(SelfRelationshipPolicy::Forbid);
    graph.insert_node(a.clone());
    graph.insert_node(b.clone());
    graph.insert_relationship(&a, &b).unwrap();

    let missing = Key("missing");
    assert_eq!(
        graph.remove_relationship(&missing, &b),
        Err(RelationshipError::UnknownNode(missing))
    );
    assert!(graph.contains_relationship(&a, &b));
    assert_eq!(
        graph.remove_relationship(&a, &a),
        Err(RelationshipError::SelfRelationshipForbidden(a.clone()))
    );
    assert!(graph.contains_relationship(&a, &b));
}

#[test]
fn node_removal_atomically_cleans_incident_relationships() {
    let a = Key("a");
    let b = Key("b");
    let c = Key("c");
    let mut graph = DirectedGraph::new(SelfRelationshipPolicy::Allow);
    for node in [&a, &b, &c] {
        graph.insert_node(node.clone());
    }
    graph.insert_relationship(&a, &b).unwrap();
    graph.insert_relationship(&b, &a).unwrap();
    graph.insert_relationship(&b, &b).unwrap();
    graph.insert_relationship(&b, &c).unwrap();
    graph.insert_relationship(&a, &c).unwrap();

    assert_eq!(
        graph.remove_node(&b),
        NodeRemoval {
            removed: true,
            relationships_removed: 4,
        }
    );
    assert_eq!(graph.relationship_count(), 1);
    assert!(graph.contains_relationship(&a, &c));
    assert_eq!(
        graph.remove_node(&b),
        NodeRemoval {
            removed: false,
            relationships_removed: 0,
        }
    );
}

#[test]
fn directed_observation_is_canonical_and_distinguishes_missing_nodes() {
    let a = Key("a");
    let b = Key("b");
    let c = Key("c");
    let d = Key("d");
    let mut graph = DirectedGraph::new(SelfRelationshipPolicy::Forbid);
    for node in [&d, &c, &b, &a] {
        graph.insert_node(node.clone());
    }
    graph.insert_relationship(&c, &a).unwrap();
    graph.insert_relationship(&a, &d).unwrap();
    graph.insert_relationship(&a, &b).unwrap();
    graph.insert_relationship(&c, &b).unwrap();

    assert_eq!(
        graph.nodes().cloned().collect::<Vec<_>>(),
        keys(&["a", "b", "c", "d"])
    );
    assert_eq!(
        graph
            .relationships()
            .map(|(source, target)| (source.clone(), target.clone()))
            .collect::<Vec<_>>(),
        vec![
            (a.clone(), b.clone()),
            (a.clone(), d.clone()),
            (c.clone(), a.clone()),
            (c.clone(), b.clone()),
        ]
    );
    assert_eq!(
        graph.outgoing(&a).unwrap().cloned().collect::<Vec<_>>(),
        vec![b.clone(), d]
    );
    assert_eq!(
        graph.incoming(&b).unwrap().cloned().collect::<Vec<_>>(),
        vec![a, c]
    );
    assert_eq!(graph.outgoing(&b).unwrap().count(), 0);
    assert!(graph.outgoing(&Key("missing")).is_none());
}

#[test]
fn symmetric_neighbors_are_canonical() {
    let a = Key("a");
    let b = Key("b");
    let c = Key("c");
    let d = Key("d");
    let mut graph = SymmetricGraph::new(SelfRelationshipPolicy::Allow);
    for node in [&d, &c, &b, &a] {
        graph.insert_node(node.clone());
    }
    graph.insert_relationship(&c, &b).unwrap();
    graph.insert_relationship(&b, &a).unwrap();
    graph.insert_relationship(&d, &b).unwrap();
    graph.insert_relationship(&b, &b).unwrap();

    assert_eq!(
        graph.neighbors(&b).unwrap().cloned().collect::<Vec<_>>(),
        vec![a, b, c, d]
    );
    assert!(graph.neighbors(&Key("missing")).is_none());
}
