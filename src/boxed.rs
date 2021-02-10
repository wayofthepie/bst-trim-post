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

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};
    use std::collections::VecDeque;

    /// Build a tree from a vec specifying nodes in level order.
    pub fn build_tree(mut values: Vec<Option<i32>>) -> Option<Box<TreeNode>> {
        if values.is_empty() {
            return None;
        }
        let mut queue = VecDeque::new();
        let length = values.len();
        let height = if length > 1 {
            (length as f32).log2().floor() as usize + 1
        } else {
            1
        };
        values.reverse();
        let initial = values.pop().unwrap()?;
        let mut root = Box::new(TreeNode::new(initial));
        queue.push_front(&mut root);
        for _ in 1..height {
            while let Some(node) = queue.pop_back() {
                construct_subtree(&mut values, &mut node.left, &mut queue);
                construct_subtree(&mut values, &mut node.right, &mut queue);
            }
        }
        Some(root)
    }

    /// Build the root of a new subtree.
    fn construct_subtree<'value, 'node, 'queue>(
        values: &'value mut Vec<Option<i32>>,
        subtree_ref: &'node mut Option<Box<TreeNode>>,
        queue: &'queue mut VecDeque<&'node mut Box<TreeNode>>,
    ) {
        if let Some(Some(value)) = values.pop() {
            let node = Box::new(TreeNode::new(value));
            *subtree_ref = Some(node);
            queue.push_front(subtree_ref.as_mut().unwrap());
        }
    }

    #[test]
    fn example1() {
        let root = build_tree(vec![
            Some(10),
            Some(8),
            Some(11),
            Some(7),
            Some(9),
            None,
            Some(13),
        ]);
        let answer = Solution::trim_bst(root, 8, 12);
        assert_eq!(
            answer,
            build_tree(vec![Some(10), Some(8), Some(11), None, Some(9), None, None])
        );
    }

    #[test]
    fn example2() {
        let root = build_tree(vec![
            Some(10),
            Some(5),
            Some(15),
            Some(3),
            Some(6),
            None,
            Some(17),
            Some(2),
            None,
            None,
            None,
            Some(16),
            Some(18),
        ]);
        let answer = Solution::trim_bst(root, 4, 8);
        assert_eq!(answer, build_tree(vec![Some(5), None, Some(6)]));
    }
}
