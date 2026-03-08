/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 递归插入节点（TreeNode级）
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // 重复值：不插入（符合测试用例要求）
            Ordering::Equal => (),
            // 插入值更小：处理左子树
            Ordering::Less => match &mut self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(left_node) => left_node.insert(value),
            },
            // 插入值更大：处理右子树
            Ordering::Greater => match &mut self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(right_node) => right_node.insert(value),
            },
        }
    }

    // 递归搜索节点（TreeNode级，参数用引用避免消耗值）
    fn search(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            // 匹配到值：返回true
            Ordering::Equal => true,
            // 搜索值更小：递归搜左子树（无左子树则false）
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search(value)),
            // 搜索值更大：递归搜右子树（无右子树则false）
            Ordering::Greater => self.right.as_ref().map_or(false, |right| right.search(value)),
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 插入值到BST（入口方法）
    fn insert(&mut self, value: T) {
        match &mut self.root {
            // 空树：直接创建根节点
            None => self.root = Some(Box::new(TreeNode::new(value))),
            // 非空树：递归插入到子节点
            Some(root_node) => root_node.insert(value),
        }
    }

    // 搜索值是否存在（入口方法，参数用引用更通用）
    fn search(&self, value: &T) -> bool {
        // 空树返回false，非空则递归搜索
        self.root.as_ref().map_or(false, |root| root.search(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 空树搜索1：返回false
        assert_eq!(bst.search(&1), false);

        // 插入节点
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 搜索存在的节点：返回true
        assert_eq!(bst.search(&5), true);
        assert_eq!(bst.search(&3), true);
        assert_eq!(bst.search(&7), true);
        assert_eq!(bst.search(&2), true);
        assert_eq!(bst.search(&4), true);

        // 搜索不存在的节点：返回false
        assert_eq!(bst.search(&1), false);
        assert_eq!(bst.search(&6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 插入两次1
        bst.insert(1);
        bst.insert(1);

        // 搜索1：返回true
        assert_eq!(bst.search(&1), true);

        // 验证根节点无左右子节点（重复值未插入）
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}