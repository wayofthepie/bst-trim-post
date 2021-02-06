#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
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

pub struct Solution;

impl Solution {
    pub fn trim_bst(root: Option<Box<TreeNode>>, low: i32, high: i32) -> Option<Box<TreeNode>> {
        Self::visit(root, low, high)
    }

    fn visit(subtree: Option<Box<TreeNode>>, low: i32, high: i32) -> Option<Box<TreeNode>> {
        let mut subtree = subtree?;
        let right = subtree.right.take();
        if subtree.val < low {
            return Self::visit(right, low, high);
        }
        let left = subtree.left.take();
        if subtree.val > high {
            return Self::visit(left, low, high);
        }
        if subtree.val > low {
            subtree.left = Self::visit(left, low, high);
        }
        if subtree.val < high {
            subtree.right = Self::visit(right, low, high);
        }
        Some(subtree)
    }
}
