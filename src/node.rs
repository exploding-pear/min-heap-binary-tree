use std::fmt;
use std::rc::{Rc, Weak};
use std::cell::RefCell;


/// A node type in the binary minimum heap. This is a single 'piece' of a heap.
/// 
/// The parent is a reference to the parent node, and the children are an array of 2 nodes.
/// 
/// The node owns the children, and has a reference to the parent.
/// 
/// If the parent is deallocated, so are its children.
#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    /// creates a new node with no parents or children
    /// # Example
    /// ```
    /// let node = min_heap::node::Node::new_orphan(7);
    /// assert_eq!(node.get_value(), 7)
    /// ```
    pub fn new_orphan(value: i32) -> Rc<Node> {
        Rc::new(Node{
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    /// creates a new node from child_value with the parent being
    /// the passed in value.
    /// 
    /// Parent and Child relationship setup automatically
    /// # Example
    /// ```
    /// let parent = min_heap::node::Node::new_orphan(5);
    /// let val = vec![24];
    /// min_heap::node::Node::new_child(&parent, 24);
    /// assert_eq!(parent.get_child_values(), val)
    /// ```
    pub fn new_child(parent: &Rc<Node>, child_value: i32) {
        // creating a new node with the parent being the passed in node
        let child = Rc::new(Node{
            value : child_value,
            parent: RefCell::new(Rc::downgrade(&parent)),
            children: RefCell::new(vec![]),
        });

        // pushing a strong reference the of the new node
        // into the child vector of the parent node
        parent.children.borrow_mut().push(Rc::clone(&child));
    }

    /// swaps a parent with a child.
    /// this is done by simply swapping the values
    pub fn swap(parent: &Rc<Node>, child: &Rc<Node>) {
        unimplemented!();
    }

    /// returns the value field of the Node struct
    /// # Example
    /// ```
    /// let mut num = 3;
    /// let node1 = min_heap::node::Node::new_orphan(10);
    /// let node2 = min_heap::node::Node::new_orphan(7);
    /// num = num + node2.get_value();
    /// assert_eq!(num, node1.get_value())
    /// ```
    pub fn get_value(&self) -> i32 {
        self.value
    }

    /// returns an array of the node's child values
    /// # Example
    /// ```
    /// let node1 = min_heap::node::Node::new_orphan(1);
    /// let node2 = min_heap::node::Node::new_orphan(2);
    /// let node3 = min_heap::node::Node::new_orphan(3);
    /// 
    /// min_heap::node::Node::parent_child(&node1, &node2);
    /// min_heap::node::Node::parent_child(&node1, &node3);
    /// 
    /// let arr : Vec<i32> = vec![2, 3];
    /// 
    /// assert_eq!(arr, node1.get_child_values())
    /// ```
    pub fn get_child_values(&self) -> Vec<i32> {
        let mut result : Vec<i32> = Vec::new();
        
        for i in self.children.borrow().iter() {
            result.push(i.get_value())
        }
        result
    }

    /// An associated function of Node that takes two nodes
    /// that have a parent-child relationship and stores references to each
    pub fn parent_child(parent: &Rc<Node>, child: &Rc<Node>) {
        println!("child parent = {:?}", child.parent.borrow().upgrade());
    
        //storing a weak reference of parent in the child
        *child.parent.borrow_mut() = Rc::downgrade(&parent);
    
        //pushing a strong reference of the child into the parent
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node() {
        let node = Node::new_orphan(7);
        assert_eq!(node.get_value(), 7)
    }

    // test that parent has ref to child and child has ref to parent
    // testing th parent_child associated function of Node
    #[test]
    fn parent_child_relationship() {
        let leaf = Node::new_orphan(3);
        let branch = Node::new_orphan(5);

        Node::parent_child(&branch, &leaf);

        let branch_child = branch.children.borrow();
        match leaf.parent.borrow().upgrade() {
            None => panic!("child does not have a parent value"),
            Some(x) => assert_eq!(x.get_value(), branch.get_value())
        };
        assert_eq!(branch_child[0].value, leaf.get_value());
    }

    #[test]
    fn get_child_values() {
        let node1 = Node::new_orphan(1);
        let node2 = Node::new_orphan(2);
        let node3 = Node::new_orphan(3);
        
        Node::parent_child(&node1, &node2);
        Node::parent_child(&node1, &node3);

        let arr : Vec<i32> = vec![2, 3];
        assert_eq!(arr, node1.get_child_values())
    }

    #[test]
    fn one_new_child() {
        let parent = Node::new_orphan(5);
        let val = vec![24];
        Node::new_child(&parent, 24);
        assert_eq!(parent.get_child_values(), val)
    }
}
