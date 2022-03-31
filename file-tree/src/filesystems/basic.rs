use std::{collections::HashMap, path::{PathBuf, Path}};

pub use crate::{
    *,
    unique_id::*
};
use bimap::BiMap;
use sealed::sealed;

pub struct BasicFilesystem<'a> {
    owned_data: HashMap<OwnedDataId, PathBuf>,
    nodes: HashMap<TreeNodeId, node::TreeNode>,
    paths: BiMap<RefPathId, &'a Path>,
    
}

impl BasicFilesystem {

}

impl lookups::FilesystemLookups for BasicFilesystem {
    fn get_owned_data_from_id<'a>(&'a self, id: OwnedDataId) -> lookups::LookupResult<&'a std::path::PathBuf> {
        todo!()
    }

    fn get_tree_node_from_id<'a>(&'a self, id: TreeNodeId) -> lookups::LookupResult<&'a node::TreeNode> {
        todo!()
    }

    fn get_path_from_id<'a>(&'a self, id: RefPathId) -> lookups::LookupResult<&'a std::path::Path> {
        todo!()
    }

    fn get_id_from_path(&self, path: &std::path::Path) -> lookups::LookupResult<RefPathId> {
        todo!()
    }

    fn get_tree_node_id_from_ref_path_id(&self, id: RefPathId) -> lookups::LookupResult<TreeNodeId> {
        todo!()
    }
}