// A single node in the linked list
// Each node owns its value and the rest of the list after it
struct Node<T> {
    value: T,
    next_node: Option<Box<Node<T>>>,
}

// A singly linked list that stores values of any type T
// The list owns the first node, and each node owns the next one
pub struct LinkedList<T> {
    first_node: Option<Box<Node<T>>>,
    count: usize,
}

impl<T> LinkedList<T> {
    // Creates an empty list with no nodes
    pub fn new() -> Self {
        LinkedList {
            first_node: None,
            count: 0,
        }
    }

    // Adds an element to the end of the list
    // The list takes ownership of the element
    pub fn add(&mut self, element: T) {
        // Wrap the element in a new node that has nothing after it yet
        let fresh_node = Box::new(Node {
            value: element,
            next_node: None,
        });

        // cursor is a mutable reference to a link in the chain
        // It starts at the link that points to the first node
        let mut cursor = &mut self.first_node;

        // Move the cursor forward until it points at an empty link
        while let Some(node) = cursor {
            cursor = &mut node.next_node;
        }

        // The empty link at the end now points to the new node
        *cursor = Some(fresh_node);
        self.count += 1;
    }

    // Returns how many elements are in the list
    pub fn len(&self) -> usize {
        self.count
    }

    // Returns true when the list has no elements
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    // Returns an iterator that borrows the values in order
    // The list cannot be changed while the iterator is alive
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            upcoming: self.first_node.as_deref(),
        }
    }
}

// Borrowing iterator over the list
// 'a is how long the list stays borrowed
pub struct Iter<'a, T> {
    upcoming: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // Returns a reference to the current value, then moves to the next node
    fn next(&mut self) -> Option<Self::Item> {
        self.upcoming.map(|node| {
            self.upcoming = node.next_node.as_deref();
            &node.value
        })
    }
}

// Frees every node when the list goes out of scope
// Nodes are detached one at a time so a very long list
// cannot overflow the stack with nested drop calls
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Take the first node out of the list, leaving None behind
        let mut remaining = self.first_node.take();

        while let Some(mut node) = remaining {
            // Detach the rest of the list before this node is dropped
            remaining = node.next_node.take();
            // node (and the value it owns) is dropped here
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    // A new list should contain nothing
    #[test]
    fn new_list_is_empty() {
        let list: LinkedList<u32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    // Elements should come back in the order they were added
    #[test]
    fn add_keeps_order() {
        let mut list = LinkedList::new();
        list.add("JOB-001");
        list.add("JOB-002");
        list.add("JOB-003");
        assert_eq!(list.len(), 3);
        assert_eq!(
            list.iter().copied().collect::<Vec<_>>(),
            vec!["JOB-001", "JOB-002", "JOB-003"]
        );
    }

    // Counts how many times a value is dropped
    // The counter is shared so it can be checked after the list is gone
    struct DropCounter(Rc<Cell<u32>>);

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    // Every node must be dropped when the list goes out of scope
    #[test]
    fn drop_frees_every_node() {
        let drops = Rc::new(Cell::new(0));
        {
            let mut list = LinkedList::new();
            for _ in 0..5 {
                list.add(DropCounter(Rc::clone(&drops)));
            }
            // Nothing is dropped while the list is still alive
            assert_eq!(drops.get(), 0);
        }
        // The list is out of scope, so all 5 values were dropped
        assert_eq!(drops.get(), 5);
    }
}
