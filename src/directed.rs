use std::collections::BTreeSet;

use crate::common::{Change, NodeRemoval, NodeSet, RelationshipError, SelfRelationshipPolicy};

/// A directed structural relationship set over caller-owned node keys.
///
/// Public observation is ordered by the caller key's Ord implementation.
pub struct DirectedGraph<K> {
    nodes: NodeSet<K>,
    relationships: BTreeSet<(K, K)>,
    self_relationship_policy: SelfRelationshipPolicy,
}

impl<K: Ord + Clone> DirectedGraph<K> {
    /// Creates an empty graph with an explicit self-relationship policy.
    pub fn new(self_relationship_policy: SelfRelationshipPolicy) -> Self {
        Self {
            nodes: NodeSet::new(),
            relationships: BTreeSet::new(),
            self_relationship_policy,
        }
    }

    /// Returns the immutable self-relationship policy.
    pub const fn self_relationship_policy(&self) -> SelfRelationshipPolicy {
        self.self_relationship_policy
    }

    /// Returns the number of admitted nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of unique directed relationships.
    pub fn relationship_count(&self) -> usize {
        self.relationships.len()
    }

    /// Admits a caller-owned node key.
    pub fn insert_node(&mut self, node: K) -> Change {
        self.nodes.insert(node)
    }

    /// Returns whether a caller-owned node key is admitted.
    pub fn contains_node(&self, node: &K) -> bool {
        self.nodes.contains(node)
    }

    /// Removes a node and all of its incident relationships atomically.
    pub fn remove_node(&mut self, node: &K) -> NodeRemoval {
        if !self.nodes.remove(node) {
            return NodeRemoval::absent();
        }

        let before = self.relationships.len();
        self.relationships
            .retain(|(source, target)| source != node && target != node);
        NodeRemoval::removed(before - self.relationships.len())
    }

    /// Iterates admitted node keys in ascending key order.
    pub fn nodes(&self) -> impl Iterator<Item = &K> {
        self.nodes.iter()
    }

    /// Inserts one directed source-to-target relationship.
    pub fn insert_relationship(
        &mut self,
        source: &K,
        target: &K,
    ) -> Result<Change, RelationshipError<K>> {
        self.nodes
            .validate_relationship(source, target, self.self_relationship_policy)?;
        if self.relationships.insert((source.clone(), target.clone())) {
            Ok(Change::Changed)
        } else {
            Ok(Change::Unchanged)
        }
    }

    /// Removes one directed source-to-target relationship.
    pub fn remove_relationship(
        &mut self,
        source: &K,
        target: &K,
    ) -> Result<Change, RelationshipError<K>> {
        self.nodes
            .validate_relationship(source, target, self.self_relationship_policy)?;
        if self.relationships.remove(&(source.clone(), target.clone())) {
            Ok(Change::Changed)
        } else {
            Ok(Change::Unchanged)
        }
    }

    /// Returns whether the exact directed relationship is present.
    ///
    /// Unknown endpoints are observed as not present.
    pub fn contains_relationship(&self, source: &K, target: &K) -> bool {
        self.nodes.contains(source)
            && self.nodes.contains(target)
            && self
                .relationships
                .contains(&(source.clone(), target.clone()))
    }

    /// Iterates directed relationships in lexicographic source-target key order.
    pub fn relationships(&self) -> impl Iterator<Item = (&K, &K)> {
        self.relationships
            .iter()
            .map(|(source, target)| (source, target))
    }

    /// Returns outgoing targets in ascending key order.
    ///
    /// None means the supplied node is not admitted. An admitted node with no
    /// outgoing relationships returns an empty iterator.
    pub fn outgoing<'a>(&'a self, node: &'a K) -> Option<impl Iterator<Item = &'a K> + 'a> {
        self.nodes.contains(node).then(|| {
            self.relationships.iter().filter_map(
                move |(source, target)| {
                    if source == node { Some(target) } else { None }
                },
            )
        })
    }

    /// Returns incoming sources in ascending key order.
    ///
    /// None means the supplied node is not admitted. An admitted node with no
    /// incoming relationships returns an empty iterator.
    pub fn incoming<'a>(&'a self, node: &'a K) -> Option<impl Iterator<Item = &'a K> + 'a> {
        self.nodes.contains(node).then(|| {
            self.relationships.iter().filter_map(
                move |(source, target)| {
                    if target == node { Some(source) } else { None }
                },
            )
        })
    }
}
