use super::{Int, Range};

#[derive(PartialEq)]
enum Path {
    Right,
    Left,
    Hit,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RangeNode {
    value: Range,
    left: Option<usize>,
    right: Option<usize>,
}

impl RangeNode {
    pub fn new(value: Range) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn with_children(value: Range, left: Option<usize>, right: Option<usize>) -> Self {
        Self { value, left, right }
    }

    fn peek(&self, x: Int) -> Path {
        if x < self.value.start {
            Path::Left
        } else if x > self.value.end {
            Path::Right
        } else {
            Path::Hit
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RangeTree {
    nodes: Vec<RangeNode>,
}

impl RangeTree {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn with_nodes(nodes: Vec<RangeNode>) -> Self {
        Self { nodes }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut RangeNode> {
        if idx < self.nodes.len() {
            Some(&mut self.nodes[idx])
        } else {
            None
        }
    }

    pub fn add(&mut self, value: Range) {
        match self.nodes.len() {
            0 => self.nodes.push(RangeNode::new(value)),
            _ => self.insert_node(self.get_mut(0).unwrap(), value),
        }
    }

    pub fn addx(&mut self, value: Range) {
        match self.root {
            None => self.root = Some(Box::new(RangeNode::new(value))),
            Some(ref mut root_node) => Self::insert_node(root_node, value),
        }
    }

    fn insert_node(&mut self, current: &mut RangeNode, value: Range) {
        // Checking where the start and end values of the range would land.
        let start_side = current.peek(value.start);
        let end_side = current.peek(value.end);

        // How to handle them both going in the same direction.
        if start_side == end_side {
            match start_side {
                // They're both lesser than the values in the current range
                Path::Left => match current.left {
                    // There is no node to the left of the current node
                    None => {
                        // Create a node and add it to the arena.
                        let idx = self.nodes.len();
                        self.nodes.push(RangeNode::new(value));
                        // Rewrites the current node to point to the newly created node
                        (*current).left = Some(idx);
                    }
                    // There is a node to the left already.
                    // Use recursion to add it to the that subtree.
                    Some(idx) => self.insert_node(&mut self.get(idx).unwrap(), value),
                },
                // The Right logic is identical (but flipped) to the Left logic.
                Path::Right => match current.right {
                    None => {
                        let idx = self.nodes.len();
                        self.nodes.push(RangeNode::new(value));
                        (*current).right = Some(idx);
                    }
                    Some(idx) => self.insert_node(&mut self.get(idx).unwrap(), value),
                },
                // Both values fall into the current node's range.
                // All values contained in the new range are already represented in the current range.
                // Hence, do nothing.
                Path::Hit => (),
            }
        }
        // The hard part.
        // How to handle the values diverging paths.
        else {
            if start_side == Path::Left {}
        }
    }

    fn insert_side(&mut self, current: &mut RangeNode, side: Path, x: Int) -> Int {
        match side {
            Path::Left => match current.left {
                None =>
            },
        }
    }

    fn insert_nodex(current: &mut Box<RangeNode>, value: Range) {
        let start_side = current.peek(value.start);
        let end_side = current.peek(value.end);
        if start_side == end_side {
            match start_side {
                Path::Left => match current.left.as_mut() {
                    None => current.left = Some(Box::new(RangeNode::new(value))),
                    Some(left) => Self::insert_node(left, value),
                },
                Path::Right => match current.right.as_mut() {
                    None => current.right = Some(Box::new(RangeNode::new(value))),
                    Some(right) => Self::insert_node(right, value),
                },
                Path::Hit => (),
            }
            return;
        }

        if start_side == Path::Left {
            let replace_start = Self::insert_side(current, start_side, value.start);
            current.value.start = replace_start;
        }
        if end_side == Path::Right {
            let replace_end = Self::insert_side(current, end_side, value.end);
            current.value.end = replace_end;
        }
    }

    fn insert_sidex(current: &mut Box<RangeNode>, side: Path, x: Int) -> Int {
        match side {
            Path::Hit => unreachable!(),
            Path::Left => match current.peek(x) {
                Path::Hit => current.value.start,
                Path::Left => match &current.left {
                    None => x,
                    Some(_) => {
                        current.value.start = current.left.as_ref().unwrap().value.start;
                        (*current).left = current.left.as_mut().unwrap().left.take();
                        Self::insert_side(current, side, x)
                    }
                },
                Path::Right => match &current.right {
                    None => x,
                    Some(_) => Self::insert_side(current.right.as_mut().unwrap(), side, x),
                },
            },
            Path::Right => match current.peek(x) {
                Path::Hit => current.value.end,
                Path::Right => match &current.right {
                    None => x,
                    Some(_) => {
                        current.value.end = current.right.as_ref().unwrap().value.end;
                        (*current).right = current.right.as_mut().unwrap().right.take();
                        Self::insert_side(current, side, x)
                    }
                },
                Path::Left => match &current.left {
                    None => x,
                    Some(_) => Self::insert_side(current.left.as_mut().unwrap(), side, x),
                },
            },
        }
    }

    pub fn search(&self, x: Int) -> bool {
        match self.root {
            None => false,
            Some(ref root) => Self::search_node(root, x),
        }
    }

    fn search_node(current: &Box<RangeNode>, x: Int) -> bool {
        match current.peek(x) {
            Path::Hit => true,
            Path::Left => match current.left.as_ref() {
                None => false,
                Some(left) => Self::search_node(left, x),
            },
            Path::Right => match current.right.as_ref() {
                None => false,
                Some(right) => Self::search_node(right, x),
            },
        }
    }
}
