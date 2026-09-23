use std::collections::BTreeSet;

use crate::common::{
    Change, NodeRemoval, NodeSet, RelationshipError, SelfRelationshipPolicy,
};

/// A symmetric structural relationship set over caller-owned node keys.
///
/// Public relationship pairs are canonicalized by the caller key's Ord
/// implementation.
pub struct SymmetricGraph<K> {
    nodes: NodeSet<K>,
    relationships: BTreeSet<(K, K)>,
    self_relationship_policy: SelfRelationshipPolicy,
}

impl<K: Ord + Clone> SymmetricGraph<K> {
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

    /// Returns the number of unique symmetric relationships.
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
            .retain(|(left, right)| left != node && right != node);
        NodeRemoval::removed(before - self.relationships.len())
    }

    /// Iterates admitted node keys in ascending key order.
    pub fn nodes(&self) -> impl Iterator<Item = &K> {
        self.nodes.iter()
    }

    /// Inserts one orientation-independent relationship.
    pub fn insert_relationship(
        &mut self,
        first: &K,
        second: &K,
    ) -> Result<Change, RelationshipError<K>> {
        self.nodes
            .validate_relationship(first, second, self.self_relationship_policy)?;
        if self.relationships.insert(canonical_pair(first, second)) {
            Ok(Change::Changed)
        } else {
            Ok(Change::Unchanged)
        }
    }

    /// Removes one orientation-independent relationship.
    pub fn remove_relationship(
        &mut self,
        first: &K,
        second: &K,
    ) -> Result<Change, RelationshipError<K>> {
        self.nodes
            .validate_relationship(first, second, self.self_relationship_policy)?;
        if self.relationships.remove(&canonical_pair(first, second)) {
            Ok(Change::Changed)
        } else {
            Ok(Change::Unchanged)
        }
    }

    /// Returns whether the orientation-independent relationship is present.
    ///
    /// Unknown endpoints are observed as not present.
    pub fn contains_relationship(&self, first: &K, second: &K) -> bool {
        self.nodes.contains(first)
            && self.nodes.contains(second)
            && self.relationships.contains(&canonical_pair(first, second))
    }

    /// Iterates canonical endpoint pairs in lexicographic key order.
    pub fn relationships(&self) -> impl Iterator<Item = (&K, &K)> {
        self.relationships
            .iter()
            .map(|(left, right)| (left, right))
    }

    /// Returns directly related neighbors in ascending key order.
    ///
    /// None means the supplied node is not admitted. An admitted node with no
    /// neighbors returns an empty iterator.
    pub fn neighbors<'a>(
        &'a self,
        node: &'a K,
    ) -> Option<impl Iterator<Item = &'a K> + 'a> {
        self.nodes.contains(node).then(|| {
            self.relationships
                .iter()
                .filter_map(move |(left, right)| {
                    if left == node {
                        Some(right)
                    } else if right == node {
                        Some(left)
                    } else {
                        None
                    }
                })
        })
    }
}

fn canonical_pair<K: Ord + Clone>(first: &K, second: &K) -> (K, K) {
    if first <= second {
        (first.clone(), second.clone())
    } else {
        (second.clone(), first.clone())
    }
}
