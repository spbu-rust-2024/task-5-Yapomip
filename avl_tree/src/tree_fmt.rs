use std::fmt;
use std::fmt::Write;

use crate::{Tree, TreeElement};

impl<K, D> TreeElement<K, D>
where
    K: fmt::Display + Eq + Ord,
    D: fmt::Display + Copy,
{
    fn print(&self, level: usize) -> String {
        let spaces = "  ";
        let mut out = String::new();

        if let Some(r) = &self.right {
            out += &r.print(level + 1);
        } else {
            write!(&mut out, "{}-", spaces.repeat(level + 1)).unwrap();
        }

        write!(
            &mut out,
            "\n{}{} {}\n",
            spaces.repeat(level),
            self.key,
            self.data
        )
        .unwrap();

        if let Some(l) = &self.left {
            out += &l.print(level + 1);
        } else {
            write!(&mut out, "{}-", spaces.repeat(level + 1)).unwrap();
        }

        out
    }
}

impl<K, D> fmt::Display for TreeElement<K, D>
where
    K: fmt::Display + Eq + Ord,
    D: fmt::Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(0))
    }
}

impl<K, D> Tree<K, D>
where
    K: fmt::Display + Eq + Ord,
    D: fmt::Display + Copy,
{
    fn print(&self) -> String {
        if let Some(root) = &self.root {
            root.print(0)
        } else {
            "No vertex".to_string()
        }
    }
}

impl<K, D> fmt::Display for Tree<K, D>
where
    K: fmt::Display + Eq + Ord,
    D: fmt::Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::*;

    #[ignore]
    #[test]
    fn test_element_tree_print() {
        // let tree_element_right =
        //     RefCell::new(Box::new(TreeElement::new(3_i64, 10_i64)));
        // let tree_element_left = RefCell::new(Box::new(TreeElement::new(1_i64, 15_i64)));
        // let tree_element = TreeElement::new(
        //     2_i64,
        //     20_i64,
        // );
        let out = String::new()
            + "    -"
            + "\n"
            + "  3 10"
            + "\n"
            + "    -"
            + "\n"
            + "2 20"
            + "\n"
            + "    -"
            + "\n"
            + "  1 15"
            + "\n"
            + "    -";

        // assert_eq!(tree_element.print(0), out);
    }

    #[test]
    fn test_tree_print() {
        let mut tree = Tree::<i64, i64>::new();
        assert_eq!(tree.print(), "No vertex");

        tree.add(1, 2);

        let out = String::new() + "  -" + "\n" + "1 2" + "\n" + "  -";

        assert_eq!(tree.print(), out);
    }
}
