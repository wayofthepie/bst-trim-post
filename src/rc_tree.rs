use std::{cell::RefCell, rc::Rc};

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
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::visit(root, low, high)
    }

    fn visit(
        subtree: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let subtree = subtree?;
        {
            let mut subtree_ref = subtree.borrow_mut();
            let right = subtree_ref.right.take();
            if subtree_ref.val < low {
                return Self::visit(right, low, high);
            }
            let left = subtree_ref.left.take();
            if subtree_ref.val > high {
                return Self::visit(left, low, high);
            }
            if subtree_ref.val > low {
                subtree_ref.left = Self::visit(left, low, high);
            }
            if subtree_ref.val < high {
                subtree_ref.right = Self::visit(right, low, high);
            }
        }
        Some(subtree)
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    /// Build a tree from a vec specifying nodes in level order.
    pub fn build_tree(mut values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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
        let root = Rc::new(RefCell::new(TreeNode::new(initial)));
        queue.push_back(root.clone());
        for _ in 1..height {
            while let Some(node) = queue.pop_front() {
                let mut node_ref = node.borrow_mut();
                construct_subtree(&mut values, &mut node_ref.left, &mut queue);
                construct_subtree(&mut values, &mut node_ref.right, &mut queue);
            }
        }
        Some(root)
    }

    /// Build the root of a new subtree.
    fn construct_subtree(
        values: &mut Vec<Option<i32>>,
        subtree_ref: &mut Option<Rc<RefCell<TreeNode>>>,
        queue: &mut VecDeque<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(Some(value)) = values.pop() {
            let node = Rc::new(RefCell::new(TreeNode::new(value)));
            let node_ref = node.clone();
            *subtree_ref = Some(node);
            queue.push_back(node_ref);
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
        let expected = build_tree(vec![Some(10), Some(8), Some(11), None, Some(9), None, None]);
        assert_eq!(
            answer, expected,
            "\nexpected tree \n{:#?}\n got \n{:#?}",
            expected, answer
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
        let expected = build_tree(vec![Some(5), None, Some(6)]);
        assert_eq!(
            answer, expected,
            "expected tree \n{:#?}\n got \n{:#?}",
            expected, answer
        );
    }
}
