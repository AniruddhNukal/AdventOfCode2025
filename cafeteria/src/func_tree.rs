use super::{Int, Range};

#[derive(Clone, Debug, PartialEq)]
struct RangeNode {
    value: Range,
    left: Option<Box<RangeNode>>,
    right: Option<Box<RangeNode>>,
}

impl RangeNode {
    fn new(val: Range, left: Option<Box<RangeNode>>, right: Option<Box<RangeNode>>) -> Self {
        Self {
            value: val,
            left,
            right,
        }
    }

    fn insert_left(node: Option<Box<Self>>, val: Int) -> (Option<Box<Self>>, Int) {
        match node {
            None => (None, val),
            Some(node) => node.as_ref().insert_present_left(val),
        }
    }

    fn insert_present_left(&self, val: Int) -> (Option<Box<Self>>, Int) {
        if self.value.is_in_range(val) {
            (self.left.clone(), self.value.start)
        } else if val < self.value.start {
            Self::insert_left(self.left.clone(), val)
        } else {
            let (right, replacer) = Self::insert_left(self.right.clone(), val);
            (Some(Box::new(Self::new(self.value.clone(), self.left.clone(), right))), replacer)
        }
    }

    fn insert_right(node: Option<Box<Self>>, val: Int) -> (Option<Box<Self>>, Int) {
        match node {
            None => (None, val),
            Some(node) => node.as_ref().insert_present_right(val),
        }
    }

    fn insert_present_right(&self, val: Int) -> (Option<Box<Self>>, Int) {
        if self.value.is_in_range(val) {
            (self.right.clone(), self.value.end)
        } else if val > self.value.end {
            Self::insert_right(self.right.clone(), val)
        } else {
            let (left, replacer) = Self::insert_right(self.left.clone(), val);
            (Some(Box::new(Self::new(self.value.clone(), left, self.right.clone()))), replacer)
        }
    }

    fn insert_node(&self, val: Range) -> Self {
        // Base "do nothing" case
        if self.value.is_in_range(val.start) && self.value.is_in_range(val.end) {
            self.clone()
        // Redirect to the left or right nodes
        } else if val.end < self.value.start {
            let left = match self.left {
                None => Self::new(val, None, None),
                Some(ref node) => node.insert_node(val),
            };
            Self::new(self.value.clone(), Some(Box::new(left)), self.right.clone())
        } else if val.start > self.value.end {
            let right = match self.right {
                None => Self::new(val, None, None),
                Some(ref node) => node.insert_node(val),
            };
            Self::new(self.value.clone(), self.left.clone(), Some(Box::new(right)))
        // Handle cases where the terminus is here
        } else {
            let (left, replace_start) = if val.start < self.value.start {
                RangeNode::insert_left(self.left.clone(), val.start)
            } else {
                (self.left.clone(), self.value.start)
            };

            let (right, replace_end) = if val.end > self.value.end {
                RangeNode::insert_right(self.right.clone(), val.end)
            } else {
                (self.right.clone(), self.value.end)
            };

            Self::new(Range::new(replace_start, replace_end), left, right)
        }
    }

    fn contains(node: Option<Box<Self>>, val: Int) -> bool {
        match node {
            None => false,
            Some(node) => if node.value.is_in_range(val) {
                true
            } else if val < node.value.start {
                Self::contains(node.left, val)
            } else {
                Self::contains(node.right, val)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeTree {
    root: Option<Box<RangeNode>>,
}

impl RangeTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn add(&self, val: Range) -> Self {
        match self.root {
            None => Self::with_root(Box::new(RangeNode {
                value: val,
                left: None,
                right: None,
            })),
            Some(ref root) => Self::with_root(Box::new(root.insert_node(val))),
        }
    }

    pub fn contains(&self, val: Int) -> bool {
        RangeNode::contains(self.root.clone(), val)
    }
}

impl RangeTree {
    fn with_root(node: Box<RangeNode>) -> Self {
        Self { root: Some(node) }
    }
}

impl RangeTree {
    pub fn span(&self) -> Int {
        RangeNode::span(self.root.clone())
    }
}

impl RangeNode {
    fn span(node: Option<Box<Self>>) -> Int {
        match node {
            None => 0,
            Some(node) => node.value.span() + Self::span(node.left) + Self::span(node.right)
        }
    }
}
