
/// The ID helper struct which manages IDs used throughout this crate.
/// The reason that this is used over something like a UUID is because
/// a simple counter has less overhead
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u64);

impl Id {
    /// Creates a new ID given a tree id and an item id
    pub(crate) fn new(tree_id: u64, item_id: u64) -> Self {
        let internal = (tree_id << 54) | (item_id & 0x00FF_FFFF_FFFF_FFFF);
        Self(internal)
    }

    /// Gets the tree id from a given ID
    pub fn get_tree_id(self) -> u64 {
        (self.0 & 0xFF00_0000_0000_0000) >> 54
    }

    /// Gets the item id from a given ID
    pub fn get_item_id(self) -> u64 {
        self.0 & 0x00FF_FFFF_FFFF_FFFF
    }
}

/// The ID for a tree node
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TreeNodeId(Id);

/// The ID for a piece of owned data
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedDataId(Id);

/// The ID for a reference path
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefPathId(Id);

macro_rules! impl_id_disp {
    ($name:ty) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:#016x}", self.0.0)
            }
        }
    }
}

impl_id_disp!(TreeNodeId);
impl_id_disp!(OwnedDataId);
impl_id_disp!(RefPathId);