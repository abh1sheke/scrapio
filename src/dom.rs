use bytes::Bytes;

pub mod node;

#[derive(Debug)]
pub struct DomTree {
    size: usize,
    root: Option<usize>,
    current: Option<usize>,
    text: Bytes,
    nodes: Vec<node::Node>,
}

#[derive(Debug)]
pub enum Error {
    EmptyTree,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyTree => writeln!(f, "Cannot perform action since tree is empty!"),
        }
    }
}

impl std::error::Error for Error {}

impl DomTree {
    pub fn new() -> Self {
        return DomTree {
            size: 0,
            root: None,
            current: None,
            text: Bytes::new(),
            nodes: Vec::with_capacity(10000),
        };
    }
    pub fn get_root(&self) -> Option<usize> {
        return self.root;
    }
    pub fn set_text(&mut self, text: Bytes) {
        self.text = text;
    }
    pub fn len(&self) -> usize {
        return self.nodes.len();
    }
    pub fn push_current(&mut self, mut _node: node::Node) {
        let idx = self.len();
        if let Some(current) = self.current {
            let parent = self.nodes.get_mut(current).unwrap();
            _node.set_parent(Some(current));
            parent.append_children(idx);
        } else {
        }
        if self.root.is_none() {
            self.root = Some(idx);
        }
        self.nodes.push(_node);
        self.current = Some(idx);
        self.size += 1;
    }
    pub fn pop_current(&mut self) -> Result<(), Error> {
        return match self.current {
            Some(current) => {
                self.current = self.nodes.get_mut(current).unwrap().get_parent();
                Ok(())
            }
            None => Err(Error::EmptyTree),
        };
    }
    pub fn get_current(&mut self) -> Option<&mut node::Node> {
        if let Some(current) = self.current {
            let node = self.nodes.get_mut(current).unwrap();
            return Some(node);
        } else {
            return None;
        }
    }
}
