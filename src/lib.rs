//! RunenGraph foundational relationship semantic kernel.
//!
//! RunenGraph stores structural relationships over identities supplied by the
//! caller. The repository spec directory is the normative authority for the
//! semantics realized by this crate.

mod common;
mod directed;
mod symmetric;

pub use common::{Change, NodeRemoval, RelationshipError, SelfRelationshipPolicy};
pub use directed::DirectedGraph;
pub use symmetric::SymmetricGraph;
