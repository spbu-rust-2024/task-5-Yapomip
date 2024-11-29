mod tree;
mod tree_element;
mod tree_fmt;

use std::sync::Arc;
use std::ptr::*;

struct TreeElement<K, D>
where
    K: Eq + Ord,
    D: Copy,
{
    key: K,
    data: D,

    parent: Option<Arc<TreeElement<K, D>>>,

    right: Option<Arc<TreeElement<K, D>>>,
    left: Option<Arc<TreeElement<K, D>>>,

    // different about left - right
    diff: i8,
}

pub struct Tree<K, D>
where
    K: Eq + Ord,
    D: Copy,
{
    root: Option<Arc<TreeElement<K, D>>>,
}
