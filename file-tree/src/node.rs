use std::collections::HashSet;
use std::fmt;

use crate::unique_id::*;

pub struct NodeChildInsertError;

impl fmt::Debug for NodeChildInsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The child was already a part of the node!")
    }
}

/// The node structure for the file tree that only contains IDs
pub struct TreeNode {
    parent_key:     TreeNodeId,
    owned_key:      OwnedDataId,
    ref_path_key:   RefPathId,
    children:       HashSet<TreeNodeId>,
}

impl TreeNode {
    /// Returns the parent TreeNodeId
    pub fn parent(&self) -> TreeNodeId {
        self.parent_key
    }

    /// Returns the OwnedDataId
    pub fn owned(&self) -> OwnedDataId {
        self.owned_key
    }

    /// Returns the RefPathId for this node's path
    pub fn path(&self) -> RefPathId {
        self.ref_path_key
    }

    /// Checks if this node contains a specified child TreeNodeId
    /// ## Arguments
    /// - `child` - The child [TreeNodeId] is being checked
    /// ## Returns
    /// Whether or not this node contains the child node
    pub fn contains_child(&self, child: TreeNodeId) -> bool {
        self.children.contains(&child)
    }

    /// Inserts the specified child TreeNodeId
    /// ## Arguments
    /// - `child` - The child [TreeNodeId] to insert
    /// ## Returns
    /// - `Ok(())`: This is the first time the child was inserted
    /// - `Err(NodeChildInsertError)`: The child had been inserted previously
    pub fn insert_child(&mut self, child: TreeNodeId) -> Result<(), NodeChildInsertError> {
        if !self.children.insert(child) {
            Err(NodeChildInsertError)
        } else {
            Ok(())
        }
    }
}