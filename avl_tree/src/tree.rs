use std::cmp::Ordering;
use std::{borrow::Borrow, cell::RefCell};
use std::sync::Arc;

use crate::*;

impl<K, D> Tree<K, D>
where
    K: Eq + Ord,
    D: Copy,
{
    pub fn new() -> Self {
        Tree { root: None }
    }
}

impl<K, D> Tree<K, D>
where
    K: Eq + Ord + std::fmt::Display,
    D: Copy + std::fmt::Display,
{

    fn balance(element: &mut Option<Arc<TreeElement<K, D>>>) {
        loop {
            if let Some(element_to_balance) = element {
                if Arc::get_mut(element_to_balance).unwrap().balance() {
                }
            }
        }
    }

    pub fn find(&self, key: K) -> Option<D> {
        let mut choice = &self.root;

        while let Some(element) = choice {
            choice = match key.cmp(&element.key) {
                Ordering::Greater => &element.right,
                Ordering::Less => &element.left,
                Ordering::Equal => {
                    return Some(element.data);
                }
            };
        }

        return None;
    }

    pub fn add(&mut self, key: K, data: D) -> Option<D> {
        let mut parent: Option<Arc<TreeElement<K, D>>> = None;
        let mut choice_element: &mut Option<Arc<TreeElement<K, D>>> = &mut self.root;

        while let Some(element) = choice_element {
            parent = Some(element.clone());
            choice_element = match key.cmp(&element.key) {
                Ordering::Greater => {
                    Arc::get_mut(element).unwrap().diff -= 1;
                    &mut Arc::get_mut(element).unwrap().right
                },
                Ordering::Less => {
                    println!("{}", element);
                    Arc::get_mut(element).unwrap().diff += 1;
                    &mut Arc::get_mut(element).unwrap().left
                },
                Ordering::Equal => {
                    let save_data = element.data;
                    Arc::get_mut(element).unwrap().data = data;
                    return Some(save_data);
                }
            };
        }

        let mut new_eleemnt = TreeElement::new(key, data);
        new_eleemnt.set_parent(parent.clone());
        *choice_element = Some(Arc::new(new_eleemnt));

        
        if let Some(a) = parent {
            Tree::balance(&mut a.parent.clone());
        }
        
        return None;
    }
}

impl<K, D> Default for Tree<K, D>
where
    K: Eq + Ord,
    D: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::*;

    #[test]
    fn test_add() {
        let mut tree = Tree::new();

        assert_eq!(tree.add(2, 2), None);
        // println!("{}", &tree);
        assert_eq!(tree.add(1, 1), None);
        // println!("{}", &tree);
        // assert_eq!(tree.add(3, 3), None);
        // println!("{}", &tree);

        let a = format!("{}", &tree);
        let b = String::new()
            + "    -"
            + "\n"
            + "  3 3"
            + "\n"
            + "    -"
            + "\n"
            + "2 2"
            + "\n"
            + "    -"
            + "\n"
            + "  1 1"
            + "\n"
            + "    -";

        assert_eq!(a, b);
        assert_eq!(Some(3), tree.add(3, 6));
    }
}
