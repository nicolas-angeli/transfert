mod pre_implemented;

pub struct LinkedList<T>(Option<Box<Node<T>>>);

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
    prev: *mut Node<T>,
}

pub struct Cursor<'a, T> {
    current: *mut Node<T>,
    marker: std::marker::PhantomData<&'a mut T>,
}

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
    marker: std::marker::PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut current = &self.0;
        while let Some(node) = current {
            len += 1;
            current = &node.next;
        }
        len
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.0.as_deref_mut().unwrap_or_else(|| std::ptr::null_mut()),
            marker: std::marker::PhantomData,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let mut current = &mut self.0;
        while let Some(node) = &mut current.as_mut().unwrap().next {
            current = &mut node.next;
        }
        Cursor {
            current: current.as_deref_mut().unwrap_or_else(|| std::ptr::null_mut()),
            marker: std::marker::PhantomData,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.0.as_deref(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if !self.current.is_null() {
            unsafe { Some(&mut (*self.current).element) }
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<&mut T> {
        if !self.current.is_null() {
            let next = unsafe { &mut (*self.current).next };
            self.current = next.as_deref_mut()?.as_mut();
            Some(&mut (*self.current).element)
        } else {
            None
        }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        if !self.current.is_null() {
            let prev = unsafe { &mut (*self.current).prev };
            if !prev.is_null() {
                self.current = prev;
                Some(unsafe { &mut (*self.current).element })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn take(&mut self) -> Option<T> {
        if self.current.is_null() {
            return None;
        }
        let current = unsafe { Box::from_raw(self.current) };
        if !current.prev.is_null() {
            unsafe { (*current.prev).next = current.next };
        }
        if let Some(next) = &mut current.next {
            next.prev = current.prev;
        }
        Some(current.element)
    }

    pub fn insert_after(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: None,
            prev: self.current,
        });
        if !self.current.is_null() {
            let next = unsafe { &mut (*self.current).next };
            new_node.next = next.take();
            if let Some(next_node) = &mut new_node.next {
                next_node.prev = &mut *new_node;
            }
            *next = Some(new_node);
        }
    }

    pub fn insert_before(&mut self, element: T) {
        if !self.current.is_null() {
            let prev = unsafe { &mut (*self.current).prev };
            let new_node = Box::new(Node {
                element,
                next: None,
                prev: *prev,
            });
            if !prev.is_null() {
                let prev_node = unsafe { &mut **prev };
                prev_node.next = Some(new_node.clone());
            }
            new_node.next = Some(Box::new(unsafe { *self.current }));
            self.current = Box::into_raw(new_node);
        } else {
            self.insert_after(element);
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(node) = self.current {
            self.current = node.next.as_deref();
            Some(&node.element)
        } else {
            None
        }
    }
}

#[test]
fn is_generic() {
    struct Foo;
    LinkedList::<Foo>::new();
}

// Tests for Step 1: push / pop at front and back
#[test]
fn basics_empty_list() {
    let list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn basics_single_element_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(5);

    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop_back(), Some(5));

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn basics_push_pop_at_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in (0..10).rev() {
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_back().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn basics_single_element_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_front(5);

    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop_front(), Some(5));

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn basics_push_pop_at_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_front(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in (0..10).rev() {
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_front().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[