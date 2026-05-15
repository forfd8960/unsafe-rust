use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(t: T) -> Self {
        Self {
            element: t,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct DList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    /*
    The PhantomData<T> is a marker used to inform the compiler that the DoublyLinkedList struct logically acts as if it owns values of type T, even though it only stores pointers.[6, 15] This is essential for correct drop-checking and variance.
        */
    _marker: PhantomData<T>,
}

impl<T> DList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    /*
    When adding a node, we allocate it using Box::new and then convert that Box into a raw pointer via Box::into_raw. This transfers ownership of the memory from the compiler's management to our manual logic
    */
    pub fn push_front(&mut self, element: T) {
        let new_node = Box::new(Node::new(element));
        let node_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) };

        unsafe {
            match self.head {
                None => {
                    self.head = Some(node_ptr);
                    self.tail = Some(node_ptr);
                }
                Some(old_head) => {
                    (*node_ptr.as_ptr()).next = Some(old_head);
                    (*old_head.as_ptr()).prev = Some(node_ptr);
                    self.head = Some(node_ptr);
                }
            }
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, element: T) {
        let new_node = Box::new(Node::new(element));
        let node_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) };

        unsafe {
            match self.tail {
                None => {
                    self.head = Some(node_ptr);
                    self.tail = Some(node_ptr);
                }
                Some(old_tail) => {
                    (*node_ptr.as_ptr()).prev = Some(old_tail);
                    (*old_tail.as_ptr()).next = Some(node_ptr);
                    self.tail = Some(node_ptr);
                }
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node_ptr| unsafe {
            let node = Box::from_raw(node_ptr.as_ptr());
            self.head = node.next;
            match self.head {
                Some(new_head) => (*new_head.as_ptr()).prev = None,
                None => self.tail = None,
            }
            self.len -= 1;
            node.element
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node_ptr| unsafe {
            let node = Box::from_raw(node_ptr.as_ptr());
            self.tail = node.prev;
            match self.tail {
                Some(new_tail) => (*new_tail.as_ptr()).next = None,
                None => self.head = None,
            }
            self.len -= 1;
            node.element
        })
    }
}
