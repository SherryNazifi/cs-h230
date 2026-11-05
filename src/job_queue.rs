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
}

// Frees every node when the list goes out of scope
// Dropping the first node drops the node after it, and so on down the list
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Take the first node out of the list and drop it
        // Its next_node field is dropped next, which repeats this for the whole chain
        drop(self.first_node.take());
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
        assert_eq!(list.len(), 0);
    }

    // Adding elements should increase the length
    #[test]
    fn add_increases_length() {
        let mut list = LinkedList::new();
        list.add("JOB-001");
        list.add("JOB-002");
        list.add("JOB-003");
        assert_eq!(list.len(), 3);
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
