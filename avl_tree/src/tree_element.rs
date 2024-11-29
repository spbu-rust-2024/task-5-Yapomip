use std::cmp::*;
use std::rc::*;
use std::ptr::*;
use std::sync::Arc;

use crate::TreeElement;

impl<K, D> TreeElement<K, D>
where
    K: Eq + Ord,
    D: Copy,
{
    pub fn new(key: K, data: D) -> TreeElement<K, D> {
        TreeElement {
            key,
            data,
            parent: None,
            right: None,
            left: None,
            diff: 0,
        }
    }

    pub fn set_parent(&mut self, parent: Option<Arc<TreeElement<K, D>>>) {
        self.parent = parent;
    }

    // true if it balanced
    // false if not balanced
    pub fn balance(&mut self) -> bool {
        false
    }
}

impl<K, D> PartialOrd for TreeElement<K, D>
where
    K: Eq + Ord,
    D: Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K, D> Ord for TreeElement<K, D>
where
    K: Eq + Ord,
    D: Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K, D> PartialEq for TreeElement<K, D>
where
    K: Eq + Ord,
    D: Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K, D> Eq for TreeElement<K, D>
where
    K: Eq + Ord,
    D: Copy,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp() {
        assert!(TreeElement::new(2, 2) < TreeElement::new(4, 0));

        assert!(TreeElement::new(3, 2) < TreeElement::new(4, 0));

        assert!(TreeElement::new(4, 2) == TreeElement::new(4, 0));
    }
}
