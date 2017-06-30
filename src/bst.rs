use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::fmt::Debug;
use std::iter::IntoIterator;
use std::marker::PhantomData;
use std::collections::VecDeque;

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

impl<T: Ord> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

#[derive(Debug)]
pub struct BST<T> {
    root: Option<Link<T>>,
}

impl<T> Default for BST<T> {
    fn default() -> Self {
        BST { root: None }
    }
}

impl<T> Drop for BST<T> {
    fn drop(&mut self) {
        if let Some(root) = self.root.take() {
            let mut vec = VecDeque::new();
            vec.push_back(root);
            while let Some(node) = vec.pop_front() {
                if let Some(left) = node.borrow_mut().left.take() {
                    left.borrow_mut().parent = None;
                    vec.push_back(left);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    right.borrow_mut().parent = None;
                    vec.push_back(right);
                }
            }
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
        let mut tmp = self.root.clone();
        while let Some(t) = tmp.clone() {
            if **t.borrow() > *data {
                tmp = t.borrow().left.clone();
            } else if **t.borrow() < *data {
                tmp = t.borrow().right.clone();
            } else {
                return true;
            }
        }
        false
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

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
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

impl<'a, T: Ord + Debug> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Ord + Debug> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
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

pub struct IterMut<'a, T: Ord + 'a> {
    next: Option<Link<T>>,
    _marker: PhantomData<&'a mut T>,
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

impl<'a, T: Ord> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|next| {
            self.next = next_link(next.clone());
            unsafe { &mut **next.as_ptr() }
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate env_logger;

    use super::*;
    use std::cell::Cell;

    #[derive(Debug, Ord, Default, PartialEq, PartialOrd, Eq, Clone)]
    struct DropCounter(Cell<u64>);

    impl DropCounter {
        fn new() -> Self {
            DropCounter(Cell::new(0))
        }

        fn add_one(&self) {
            self.0.set(self.0.get() + 1);
        }

        fn get_count(&self) -> u64 {
            self.0.get()
        }
    }

    #[derive(Debug, Eq)]
    struct Td<'a> {
        dc: &'a DropCounter,
        id: u64,
    }

    impl<'a> Td<'a> {
        fn new<'b: 'a>(id: u64, dc: &'b DropCounter) -> Self {
            Td::<'b> { dc: dc, id: id }
        }
    }

    impl<'a> PartialEq for Td<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl<'a> PartialOrd for Td<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<'a> Ord for Td<'a> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.id.cmp(&other.id)
        }
    }

    impl<'a> Drop for Td<'a> {
        fn drop(&mut self) {
            self.dc.add_one();
        }
    }

    #[test]
    fn test_bst() {}

    #[test]
    fn test_drop() {
        env_logger::init().unwrap();
        let dc = DropCounter::new();
        {
            let mut bst: BST<_> = Default::default();
            bst.insert(Td::new(5, &dc));
            bst.insert(Td::new(3, &dc));
            bst.insert(Td::new(1, &dc));
            bst.insert(Td::new(4, &dc));
            bst.insert(Td::new(7, &dc));
            bst.insert(Td::new(6, &dc));
            bst.insert(Td::new(8, &dc));
        }
        assert_eq!(7, dc.get_count());
        info!("{}", dc.get_count());

        let dc = DropCounter::new();
        {
            let mut bst: BST<_> = Default::default();
            bst.insert(Td::new(5, &dc));
        }
        assert_eq!(1, dc.get_count());
    }

    #[test]
    fn test_search() {
        let dc = DropCounter::new();
        {
            let mut bst: BST<_> = Default::default();
            bst.insert(Td::new(5, &dc));
            bst.insert(Td::new(3, &dc));
            bst.insert(Td::new(1, &dc));
            bst.insert(Td::new(4, &dc));
            bst.insert(Td::new(7, &dc));
            bst.insert(Td::new(6, &dc));
            bst.insert(Td::new(8, &dc));

            assert_eq!(true, bst.search(&Td::new(5, &dc)));
            assert_eq!(false, bst.search(&Td::new(10, &dc)));

            let bst: BST<u32> = Default::default();
            assert_eq!(false, bst.search(&3));
        }
    }

    #[test]
    fn test_iter() {
        let mut bst: BST<_> = Default::default();
        bst.insert(5);
        bst.insert(3);
        bst.insert(1);
        bst.insert(4);
        bst.insert(7);
        bst.insert(6);
        bst.insert(8);

        let a = vec![1, 3, 4, 5, 6, 7, 8];

        let b: Vec<_> = bst.iter().map(|t| *t).collect();
        assert_eq!(a, b);

        let mut c = vec![];
        for t in &b {
            c.push(*t);
        }
        assert_eq!(a, c);

        let mut b: Vec<_> = bst.iter_mut().map(|t| *t).collect();
        assert_eq!(a, b);

        let mut c = vec![];
        for t in &mut b {
            *t += 1;
        }
        for t in & b {
            c.push(*t);
        }
        let d: Vec<_> = a.clone().into_iter().map(|t| t+1).collect();
        assert_eq!(c,d);

        let b: Vec<_> = bst.into_iter().collect();
        assert_eq!(a, b);

        let mut bst: BST<i32> = Default::default();
        assert_eq!(0, bst.iter().count());
    }
}
