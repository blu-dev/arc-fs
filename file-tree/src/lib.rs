mod filesystems;
mod lookups;
mod node;
mod unique_id;

/* PSEUDO CODE FOR THE INSERTION ALGORITHM

def insert_path(tree, path, root):
    // Check if the path is already contained in our filesystem, if it is we don't want to include it
    if path in tree.existing_paths:
        return

    // Insert the parent path of this file, and if it exists it will get handled by our check above
    insert_path(tree, path.parent())

    // Attempt to get the root key of the filesystem
    root_key = get_key_for_root(tree, root)
    if root_key == None:
        root_key = insert_root(tree, root)
    
    parent_key = get_path_key(tree, path.parent())
    next_key = get_next_key(tree)
    insert_path_for_key(next_key, path)
    node = create_node(root_key, parent_key, next_key)
*/