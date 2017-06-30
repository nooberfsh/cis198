use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::Deref;
use std::fmt::Debug;
use std::iter::IntoIterator;
use std::marker::PhantomData;

type Link<T> = Rc<RefCell<Node<T>>>;

fn new_link<T: Ord>(data: T) -> Link<T> {
    Rc::new(RefCell::new(Node::new(data)))
}

#[derive(Eq, Debug)]
struct Node<T> {
    data: T,
    parent: Option<Link<T>>,
    left: Option<Link<T>>,
    right: Option<Link<T>>,
}

impl<T: PartialEq> PartialEq for Node<T> {
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

    fn into_inner(self) -> T {
        self.data
    }
}

impl<T: Ord> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

#[derive(Debug, Default)]
pub struct BST<T> {
    root: Option<Link<T>>,
}

impl<T> Drop for BST<T> {
    fn drop(&mut self) {
        if let Some(ref mut root) = self.root {
            BST { root: root.borrow_mut().left.take() };
            BST { root: root.borrow_mut().right.take() };
        }
    }
}

impl<T: Ord> BST<T> {
    pub fn insert(&mut self, data: T) -> bool {
        let link = new_link(data);
        if self.root.is_none() {
            self.root = Some(link);
            return true;
        }
        let mut node = self.root.clone().unwrap();
        loop {
            if link < node {
                if node.borrow().left.is_none() {
                    link.borrow_mut().parent = Some(node.clone());
                    node.borrow_mut().left = Some(link);
                    return true;
                } else {
                    let tmp = node.borrow().left.clone().unwrap();
                    node = tmp;
                }
            } else if link > node {
                if node.borrow().right.is_none() {
                    link.borrow_mut().parent = Some(node.clone());
                    node.borrow_mut().right = Some(link);
                    return true;
                } else {
                    let tmp = node.borrow().right.clone().unwrap();
                    node = tmp
                }
            } else {
                return false;
            }
        }
    }

    pub fn search(&self, data: &T) -> bool {
        unimplemented!()
    }

    fn min(&self) -> Option<Link<T>> {
        self.root.as_ref().map(|n| min_link(n.clone()))
    }

    fn remove_min(&mut self) {
        if let Some(ref min) = self.min() {
            let mmin = min.borrow_mut();
            match mmin.parent.clone() {
                Some(p) => {
                    match mmin.right.clone() {
                        Some(r) => {
                            p.borrow_mut().left = Some(r.clone());
                            r.borrow_mut().parent = Some(p);
                        }
                        None => p.borrow_mut().left = None,
                    }
                }
                None => {
                    self.root = mmin.right.clone();
                    if let Some(ref n) = self.root {
                        n.borrow_mut().parent = None
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.min(),
            _marker: Default::default(),
        }
    }
}

fn min_link<T: Ord>(mut parent: Link<T>) -> Link<T> {
    let mut left = parent.borrow().left.clone();
    while let Some(ln) = left.clone() {
        parent = ln;
        left = parent.borrow().left.clone();
    }
    parent
}

fn next_link<T: Ord>(current: Link<T>) -> Option<Link<T>> {
    let currentb = current.borrow();
    match currentb.right {
        Some(ref right) => Some(min_link(right.clone())),
        None => {
            let mut parent = currentb.parent.clone();
            while let Some(p) = parent.clone() {
                if p > current {
                    return Some(p.clone());
                }
                parent = p.borrow().parent.clone();
            }
            None
        }
    }
}

impl<T: Ord + Debug> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            min: self.min(),
            bst: self,
        }
    }
}

pub struct IntoIter<T: Ord> {
    min: Option<Link<T>>,
    bst: BST<T>,
}

pub struct Iter<'a, T: Ord + 'a> {
    next: Option<Link<T>>,
    _marker: PhantomData<&'a T>,
}

impl<T: Ord + Debug> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.min.take().map(|n| {
            self.bst.remove_min();
            assert_eq!(1, Rc::strong_count(&n));
            self.min = self.bst.min();
            Rc::try_unwrap(n).unwrap().into_inner().into_inner()
        })
    }
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|next| {
            self.next = next_link(next.clone());
            unsafe { &**next.as_ptr() }
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate env_logger;

    use super::*;

    #[test]
    fn test_bst() {}

    #[test]
    fn test_drop() {}
}
