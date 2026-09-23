use std::{collections::BTreeSet, error::Error, fmt};

/// Policy controlling whether a node may relate to itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelfRelationshipPolicy {
    /// Self relationships are valid.
    Allow,
    /// Self relationships are rejected.
    Forbid,
}

/// Outcome of an idempotent structural mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Change {
    /// Authoritative graph state changed.
    Changed,
    /// The requested state was already present or absent.
    Unchanged,
}

/// Outcome of removing a node and its incident relationships.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeRemoval {
    /// Whether the node was admitted before removal.
    pub removed: bool,
    /// Number of unique incident relationships removed with the node.
    pub relationships_removed: usize,
}

impl NodeRemoval {
    pub(crate) const fn absent() -> Self {
        Self {
            removed: false,
            relationships_removed: 0,
        }
    }

    pub(crate) const fn removed(relationships_removed: usize) -> Self {
        Self {
            removed: true,
            relationships_removed,
        }
    }
}

/// Rejection returned when a relationship mutation violates R0 invariants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationshipError<K> {
    /// The caller-supplied key is not admitted to the graph.
    UnknownNode(K),
    /// The graph forbids a relationship from a node to itself.
    SelfRelationshipForbidden(K),
}

impl<K: fmt::Debug> fmt::Display for RelationshipError<K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownNode(node) => write!(formatter, "unknown graph node: {node:?}"),
            Self::SelfRelationshipForbidden(node) => {
                write!(formatter, "self relationship is forbidden for node: {node:?}")
            }
        }
    }
}

impl<K: fmt::Debug> Error for RelationshipError<K> {}

pub(crate) struct NodeSet<K> {
    nodes: BTreeSet<K>,
}

impl<K: Ord + Clone> NodeSet<K> {
    pub(crate) const fn new() -> Self {
        Self {
            nodes: BTreeSet::new(),
        }
    }

    pub(crate) fn insert(&mut self, node: K) -> Change {
        if self.nodes.insert(node) {
            Change::Changed
        } else {
            Change::Unchanged
        }
    }

    pub(crate) fn contains(&self, node: &K) -> bool {
        self.nodes.contains(node)
    }

    pub(crate) fn remove(&mut self, node: &K) -> bool {
        self.nodes.remove(node)
    }

    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &K> {
        self.nodes.iter()
    }

    pub(crate) fn validate_relationship(
        &self,
        first: &K,
        second: &K,
        policy: SelfRelationshipPolicy,
    ) -> Result<(), RelationshipError<K>> {
        if !self.contains(first) {
            return Err(RelationshipError::UnknownNode(first.clone()));
        }
        if !self.contains(second) {
            return Err(RelationshipError::UnknownNode(second.clone()));
        }
        if first == second && policy == SelfRelationshipPolicy::Forbid {
            return Err(RelationshipError::SelfRelationshipForbidden(first.clone()));
        }
        Ok(())
    }
}
