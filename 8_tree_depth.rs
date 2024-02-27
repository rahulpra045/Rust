pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            let left_depth = max_depth(node.left.clone());
            let right_depth = max_depth(node.right.clone());
            1 + std::cmp::max(left_depth, right_depth)
        }
        None => 0,
    }
}

fn main() {


    let mut root = TreeNode::new(3);
    let left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    let right_left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    let right_right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    root.left = left;
    root.right = right;
    root.right.as_ref().unwrap().borrow_mut().left = right_left;
    root.right.as_ref().unwrap().borrow_mut().right = right_right;

    let depth = max_depth(Some(Rc::new(RefCell::new(root))));
    println!("Maximum depth of the tree: {}", depth);
}
