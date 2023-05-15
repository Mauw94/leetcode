#[derive(Debug, PartialEq, Eq)]
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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        match root {
            Some(node) => {
                cmp::max(
                    max_depth(node.borrow().left.clone()),
                    max_depth(node.borrow().right.clone()),
                ) + 1
            },
            None => 0
        }    
}

pub fn run() {
    // println!("{:?}", max_depth(root));
}