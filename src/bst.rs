use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Deref;

type Link<T> = Rc<RefCell<Node<T>>>;

fn new_link<T: Ord>(data: T) -> Link<T> {
    Rc::new(RefCell::new(Node::new(data)))
}

#[derive(Eq, Debug)]
struct Node<T: Ord> {
    data: T,
    parent: Option<Link<T>>,
    left: Option<Link<T>>,
    right: Option<Link<T>>,
}

impl<T: Ord> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.data == other.data
    }
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<T: Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            parent: None,
            left: None,
            right: None,
        }
    }
}

impl<T: Ord> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

#[derive(Debug)]
pub struct BST<T: Ord> {
    root: Option<Link<T>>,
}

impl<T: Debug + Ord> BST<T> {
    pub fn new() -> Self {
        BST { root: Default::default() }
    }

    pub fn walk(&mut self, ) {
    }
}

impl<T: Ord> Drop for BST<T> {
    fn drop(&mut self) {
        match self.root {
            Some(ref mut root) => {
                BST{root: root.borrow_mut().right.take()};
                BST{root: root.borrow_mut().right.take()};
            }
            None => {}
        }
    }
}

impl<T: Ord> BST<T> {
    fn insert_link(&mut self, link: Link<T>) -> bool {
        match self.root {
            Some(ref mut root) => {
                if *root < link {
                    let right = &mut root.borrow_mut().right;
                    if right.is_none() {
                        link.borrow_mut().parent = Some(root.clone());
                        *right = Some(link);
                        return true;
                    } else {
                        BST { root: right.clone() }.insert_link(link)
                    }
                } else if *root > link {
                    let left = &mut root.borrow_mut().left;
                    if left.is_none() {
                        link.borrow_mut().parent = Some(root.clone());
                        *left = Some(link);
                        return true;
                    } else {
                        BST { root: left.clone() }.insert_link(link)
                    }
                } else {
                    return false;
                }
            }
            None => {
                self.root = Some(link);
                true
            }
        }
    }

    pub fn insert(&mut self, data: T) -> bool {
        let link = new_link(data);
        self.insert_link(link)
    }

    pub fn search(&self, data: &T) -> bool {
        match self.root {
            Some(ref root) => {
                let mut sub = BST { root: None };
                if **root.borrow() < *data {
                    sub.root = root.borrow().right.clone();
                } else if **root.borrow() > *data {
                    sub.root = root.borrow().left.clone();
                } else {
                    return true;
                }
                sub.search(data)
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate env_logger;

    use super::*;

    #[test]
    fn test_bst() {
        let mut bst = BST::new();
        bst.insert(50);
        for _ in 0..100 {
            bst.insert(rand::random::<u32>() % 100);
        }
        bst.walk();
    }


    #[derive(Debug, Ord, Default, PartialEq, PartialOrd, Eq, Clone)]
    struct Td (u64);

    impl Drop for Td {
        fn drop(&mut self) {
            info!("begin panic");
            panic!("haha");
        }
    }

    #[test]
    fn test_drop() {
        let _ = env_logger::init();
        let mut bst = BST::new();
        //bst.insert(Td(50));
        for i in 0..100 {
            bst.insert(Td(i));
        }

        
        //info!("{:#?}", bst);
        //debug!("{:?}", bst);
    }
}
