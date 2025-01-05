use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;

// Color enum for Red-Black tree nodes
#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,
    Black,
}

// Type alias for node links using Option and reference counting
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/**
 * Node structure for Red-Black tree
 * Uses Rc<RefCell<>> for interior mutability and reference counting
 */
#[derive(Clone, Debug)]
struct Node<T> {
    data: T,
    color: Color,
    left: Link<T>,
    right: Link<T>,
    parent: Link<T>,
}

// Node implementation for comparable types
impl<T: Ord + Debug + Clone> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            color: Color::Red,
            left: None,
            right: None,
            parent: None,
        }))
    }
}

/**
 * Red-Black Tree implementation
 * Properties:
 * 1. Every node is either red or black
 * 2. Root is always black
 * 3. No two adjacent red nodes
 * 4. Every path from root to leaf has same number of black nodes
 */

pub struct RedBlackTree<T> {
    root: Link<T>,
    nil: Rc<RefCell<Node<T>>>,
}

impl<T: Ord + Debug + Clone + Default> RedBlackTree<T> {
    pub fn new() -> Self {
        let nil = Rc::new(RefCell::new(Node {
            data: T::default(),
            color: Color::Black,
            left: None,
            right: None,
            parent: None,
        }));

        RedBlackTree { root: None, nil }
    }
     /// Inserts a new value into the tree while maintaining Red-Black properties
    /// Steps:
    /// 1. Perform standard BST insertion
    /// 2. Color new node red
    /// 3. Fix Red-Black violations
    pub fn insert(&mut self, data: T) {
        let new_node = Node::new(data);
        new_node.borrow_mut().left = Some(self.nil.clone());
        new_node.borrow_mut().right = Some(self.nil.clone());
    
        if self.root.is_none() {
            println!("Inserting root: {:?}", new_node.borrow().data);
            self.root = Some(new_node.clone());
            new_node.borrow_mut().color = Color::Black;
            return;
        }
    
        let mut current = self.root.clone();
        let mut parent = None;
    
        while let Some(cur) = current {
            parent = Some(cur.clone());
            if new_node.borrow().data < cur.borrow().data {
                if Rc::ptr_eq(&cur.borrow().left.as_ref().unwrap(), &self.nil) {
                    break; // Insert here
                }
                current = cur.borrow().left.clone();
            } else {
                if Rc::ptr_eq(&cur.borrow().right.as_ref().unwrap(), &self.nil) {
                    break; // Insert here
                }
                current = cur.borrow().right.clone();
            }
        }
    
        new_node.borrow_mut().parent = parent.clone();
        if let Some(p) = parent {
            if new_node.borrow().data < p.borrow().data {
                println!(
                    "Inserting {:?} as left child of {:?}",
                    new_node.borrow().data,
                    p.borrow().data
                );
                p.borrow_mut().left = Some(new_node.clone());
            } else {
                println!(
                    "Inserting {:?} as right child of {:?}",
                    new_node.borrow().data,
                    p.borrow().data
                );
                p.borrow_mut().right = Some(new_node.clone());
            }
        }
    
        self.fix_insert(new_node);
    }
    

     /// Fixes Red-Black tree violations after insertion
    /// Cases:
    /// 1. Uncle is red -> Recolor
    /// 2. Uncle is black (triangle) -> Rotate
    /// 3. Uncle is black (line) -> Rotate and recolor
    fn fix_insert(&mut self, node: Rc<RefCell<Node<T>>>) {
        let mut current_node = node;

        while let Some(parent_rc) = {
            let borrow = current_node.borrow();
            borrow.parent.clone()
        } {
            if parent_rc.borrow().color == Color::Black {
                break;
            }

            let grandparent_rc = {
                let parent_borrow = parent_rc.borrow();
                parent_borrow.parent.clone()
            };

            if let Some(gp_rc) = grandparent_rc {
                let (uncle_rc, is_left_child) = {
                    let gp_borrow = gp_rc.borrow();
                    if Rc::ptr_eq(&parent_rc, gp_borrow.left.as_ref().unwrap()) {
                        (gp_borrow.right.clone(), true)
                    } else {
                        (gp_borrow.left.clone(), false)
                    }
                };

                if let Some(uncle_rc) = uncle_rc {
                    if uncle_rc.borrow().color == Color::Red {
                        println!("Recoloring parent, uncle, and grandparent...");
                        parent_rc.borrow_mut().color = Color::Black;
                        uncle_rc.borrow_mut().color = Color::Black;
                        gp_rc.borrow_mut().color = Color::Red;

                        current_node = gp_rc;
                        continue;
                    }
                }

                if is_left_child {
                    {
                        let parent_borrow = parent_rc.borrow();
                        if Rc::ptr_eq(&current_node, parent_borrow.right.as_ref().unwrap()) {
                            drop(parent_borrow);
                            current_node = parent_rc.clone();
                            println!("Left rotate at {:?}", current_node.borrow().data);
                            self.left_rotate(current_node.clone());
                        }
                    }
                    println!("Right rotate at {:?}", gp_rc.borrow().data);
                    parent_rc.borrow_mut().color = Color::Black;
                    gp_rc.borrow_mut().color = Color::Red;
                    self.right_rotate(gp_rc.clone());
                } else {
                    {
                        let parent_borrow = parent_rc.borrow();
                        if Rc::ptr_eq(&current_node, parent_borrow.left.as_ref().unwrap()) {
                            drop(parent_borrow);
                            current_node = parent_rc.clone();
                            println!("Right rotate at {:?}", current_node.borrow().data);
                            self.right_rotate(current_node.clone());
                        }
                    }
                    println!("Left rotate at {:?}", gp_rc.borrow().data);
                    parent_rc.borrow_mut().color = Color::Black;
                    gp_rc.borrow_mut().color = Color::Red;
                    self.left_rotate(gp_rc.clone());
                }
            }
        }

        if let Some(root_rc) = &self.root {
            root_rc.borrow_mut().color = Color::Black;
        }
    }

    /// Performs left rotation around given node
    /// Used for maintaining Red-Black tree properties
    ///
    ///     x                 y
    ///    / \              / \
    ///   a   y    =>     x   c
    ///      / \         / \
    ///     b   c       a   b
    fn left_rotate(&mut self, x: Rc<RefCell<Node<T>>>) {
        let y = x.borrow().right.clone().unwrap();
        x.borrow_mut().right = y.borrow().left.clone();

        if let Some(left) = y.borrow().left.clone() {
            left.borrow_mut().parent = Some(x.clone());
        }

        y.borrow_mut().parent = x.borrow().parent.clone();

        if x.borrow().parent.is_none() {
            self.root = Some(y.clone());
        } else if Rc::ptr_eq(&x, &x.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap()) {
            x.borrow().parent.as_ref().unwrap().borrow_mut().left = Some(y.clone());
        } else {
            x.borrow().parent.as_ref().unwrap().borrow_mut().right = Some(y.clone());
        }

        y.borrow_mut().left = Some(x.clone());
        x.borrow_mut().parent = Some(y.clone());
    }

    /// Performs right rotation around given node
    /// Used for maintaining Red-Black tree properties
    ///
    ///       y           x
    ///      / \         / \
    ///     x   c  =>   a   y
    ///    / \             / \
    ///   a   b           b   c
    fn right_rotate(&mut self, y: Rc<RefCell<Node<T>>>) {
        let x = y.borrow().left.clone().unwrap();
        y.borrow_mut().left = x.borrow().right.clone();

        if let Some(right) = x.borrow().right.clone() {
            right.borrow_mut().parent = Some(y.clone());
        }

        x.borrow_mut().parent = y.borrow().parent.clone();

        if y.borrow().parent.is_none() {
            self.root = Some(x.clone());
        } else if Rc::ptr_eq(&y, &y.borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap()) {
            y.borrow().parent.as_ref().unwrap().borrow_mut().right = Some(x.clone());
        } else {
            y.borrow().parent.as_ref().unwrap().borrow_mut().left = Some(x.clone());
        }

        x.borrow_mut().right = Some(y.clone());
        y.borrow_mut().parent = Some(x.clone());
    }
     /// Performs in-order traversal of the tree
    /// Visits nodes in ascending order
    pub fn inorder(&self) {
        self.inorder_helper(self.root.clone());
    }

    fn inorder_helper(&self, node: Link<T>) {
        if let Some(n) = node {
            if Rc::ptr_eq(&n, &self.nil) {
                return;
            }
            self.inorder_helper(n.borrow().left.clone());
            println!("{:?} ({:?})", n.borrow().data, n.borrow().color);
            self.inorder_helper(n.borrow().right.clone());
        }
    }
}

fn main() {
    let mut tree = RedBlackTree::new();
    tree.insert(20);
    tree.insert(15);
    tree.insert(25);
    tree.insert(10);
    tree.insert(5);
    tree.insert(1);
    tree.insert(30);
    tree.insert(18);

    println!("In-order traversal of the Red-Black Tree:");
    tree.inorder();
}
