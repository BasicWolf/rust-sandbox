use std::cmp::{Ord, Ordering};
use std::fmt;

type OptNode<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub left: OptNode<T>,
    pub right: OptNode<T>,
}

pub struct Tree<T> {
    root: OptNode<T>
}

pub struct TreeIterator<'a, T: 'a> {
    right_unvisited: Vec<&'a Node<T>>,
}

impl<T: Ord> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: value,
            left: None,
            right: None
        }
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let left_s = self.left.as_ref().map_or(String::from("--"), |vp| format!("{}", vp.value));

        write!(
            f, "({} [{}])",
            self.value,
            left_s
        )
    }
}


impl <T: Ord> Tree<T> {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn add(&mut self, val: T) {
        let mut node = &mut self.root;

        loop {
            node = match node.as_ref().map(|n| val.cmp(&n.value)) {
                Some(Ordering::Equal) => { return; }
                Some(Ordering::Less) => {
                    let tmp = node;
                    &mut tmp.as_mut().unwrap().left
                }
                Some(Ordering::Greater) => &mut {node}.as_mut().unwrap().right,
                None => {
                    *node = Some(Box::new(Node::new(val)));
                    return;
                }
            }
        }
    }

    pub fn contains(&self, x: &T) -> bool
    where T: PartialEq<T> {
        false
    }
}

impl <T: Ord> From<Vec<T>> for Tree<T> {
    fn from(vector: Vec<T>) -> Tree<T> {
        let mut tree: Tree<T> = Tree::new();
        for v in vector {
            tree.add(v);
        }
        tree
    }
}

impl <'a, T: Ord> Iterator for TreeIterator<'a, T> {
    type Item = Box<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Box::new(Node::new(10)))
    }
}

// impl<T: Ord + fmt::Display> fmt::Display for Tree<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let root = self.root.as_ref().map_or(String::from("--"), |vp| format!("{}", vp.value));

//         write!(
//             f, "{}",
//             root
//         )
//     }
// }



#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn test_smoke() {
        let mut t: Tree<i32> = Tree::new();
        t.add(10);
    }

    #[test]
    fn test_add_by_path() {
        let mut t: Tree<i32> = Tree::new();
        t.add(10);
        t.add(5);
        t.add(20);

        // assert_eq!(10, t.root.unwrap().value);
        // assert_eq!(5, t.root.left.unwrap().value);
        // assert_eq!(20, t.root.right.unwrap().value);
    }

    #[test]
    fn test_from_vector() {
        let mut t: Tree<i32> = Tree::from(vec![10, 5, 20]);
    }

    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
}
