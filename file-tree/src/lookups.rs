use std::path::{
    Path,
    PathBuf
};

use crate::unique_id::*;
use crate::node::TreeNode;

use sealed::sealed;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LookupError {
    #[error("The owned data for the ID {0} is missing")]
    OwnedDataMissing(OwnedDataId),
    #[error("The tree node for the ID {0} is missing")]
    TreeNodeMissing(TreeNodeId),
    #[error("The path for the ID {0} is missing")]
    RefPathMissingId(RefPathId),
    #[error("The path {0:?} is missing")]
    RefPathMissingPath(PathBuf)
}

pub type LookupResult<T> = std::result::Result<T, LookupError>;

/// This trait is designed to support a variety of different kinds of lookups
/// and also allow different implementations for testing/optimizations depending on the situation
/// 
/// # Implementation
/// The following lookups are **required** for an implementor of FilesystemLookups:
/// - The ability to get the owned data (PathBuf) from the OwnedDataId
/// - The ability to get the tree node from the TreeNodeId
/// - The ability to get the referential path from the RefPathId
/// - The ability to get the RefPathId from the path
/// - The ability to get the tree node from the RefPathId
pub trait FilesystemLookups {
    /// Gets the owned data from the specified ID, returning an error if it's missing
    fn get_owned_data_from_id<'a>(&'a self, id: OwnedDataId) -> LookupResult<&'a PathBuf>;

    /// Gets the tree node from the specified ID, returning an error if it's missing
    fn get_tree_node_from_id<'a>(&'a self, id: TreeNodeId) -> LookupResult<&'a TreeNode>;

    /// Gets the path from the specified ID, returning an error if it's missing
    fn get_path_from_id<'a>(&'a self, id: RefPathId) -> LookupResult<&'a Path>;

    /// Gets the id from the specified path, returning an error if it's missing
    fn get_id_from_path(&self, path: &Path) -> LookupResult<RefPathId>;

    /// Gets the tree node from the path ID, returning an error if it's missing
    fn get_tree_node_id_from_ref_path_id(&self, id: RefPathId) -> LookupResult<TreeNodeId>;

    /// Gets the path from the specified tree node ID
    fn get_path_from_tree_node_id<'a>(&'a self, id: TreeNodeId) -> LookupResult<&'a Path> {
        let node = self.get_tree_node_from_id(id)?;
        self.get_path_from_id(node.path())
    }

    /// Gets the owned data from the specified tree node ID
    fn get_owned_data_from_tree_node_id<'a>(&'a self, id: TreeNodeId) -> LookupResult<&'a PathBuf> {
        let node = self.get_tree_node_from_id(id)?;
        self.get_owned_data_from_id(node.owned())
    }

    /// Gets the tree node from the specified path
    fn get_tree_node_from_path<'a>(&'a self, path: &Path) -> LookupResult<&'a TreeNode> {
        let id = self.get_id_from_path(path)?;
        let node = self.get_tree_node_id_from_ref_path_id(id)?;
        self.get_tree_node_from_id(node)
    }

    /// Gets the owned data from the specified path
    fn get_owned_data_from_path<'a>(&'a self, path: &Path) -> LookupResult<&'a PathBuf> {
        let node = self.get_tree_node_from_path(path)?;
        self.get_owned_data_from_id(node.owned())
    }
}

#[derive(Error, Debug)]
pub enum InsertionError {
    #[error("The path '{0:?}' is already found in the filesystem")]
    PathExists(PathBuf)
}

pub type InsertionResult<T> = std::result::Result<T, InsertionError>;

#[sealed]
pub trait FilesystemInsertions {
    /// Inserts the nodes into the filesystem
    fn insert_path(&mut self, path: &Path) -> LookupResult<()>;
}