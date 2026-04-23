#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd + Clone> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { root: None }
    }

    fn add(&mut self, value: T) {
        let new_value = TreeNode::new(value);

        match self.root {
            None => self.root = Some(Box::new(new_value)),
            Some(ref mut root_node) => {
                Self::insert_node(root_node, new_value);
            }
        }
    }

    fn insert_node(current: &mut Box<TreeNode<T>>, new_node: TreeNode<T>) {
        if new_node.value < current.value {
            match current.left {
                None => current.left = Some(Box::new(new_node)),
                Some(ref mut left_child) => {
                    Self::insert_node(left_child, new_node);
                }
            }
        } else {
            match current.right {
                None => current.right = Some(Box::new(new_node)),
                Some(ref mut right_node) => {
                    Self::insert_node(right_node, new_node);
                }
            }
        }
    }

    fn search(&mut self, value: &T) -> bool {
        match &self.root {
            None => false,
            Some(node) => Self::search_node(node, value),
        }
    }

    fn search_node(node: &Box<TreeNode<T>>, value: &T) -> bool {
        if value == &node.value {
            true
        } else if value < &node.value {
            match &node.left {
                None => false,
                Some(left) => Self::search_node(left, value),
            }
        } else {
            match &node.right {
                None => false,
                Some(right) => Self::search_node(right, value),
            }
        }
    }

    fn delete(&mut self, value: &T) {
        Self::delete_node(&mut self.root, value);
    }

    fn delete_node(node: &mut Option<Box<TreeNode<T>>>, value: &T) {
        if let Some(current) = node {
            if value < &current.value {
                Self::delete_node(&mut current.left, value);
            } else if value > &current.value {
                Self::delete_node(&mut current.right, value);
            } else {
                if current.left.is_none() {
                    *node = current.right.take();
                } else if current.right.is_none() {
                    *node = current.left.take();
                } else {
                    let min_val = Self::min_value(current.right.as_mut().unwrap());
                    current.value = min_val.clone();
                    Self::delete_node(&mut current.right, &min_val);
                }
            }
        }
    }

    fn min_value(node: &mut Box<TreeNode<T>>) -> T {
        match node.left.as_mut() {
            None => node.value.clone(),
            Some(left) => Self::min_value(left),
        }
    }
}
