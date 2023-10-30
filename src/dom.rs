use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct Node {
    text: BytesMut,
    closing: BytesMut,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Debug)]
pub enum PushText<'a> {
    Slice(&'a [u8]),
    Byte(u8),
}

impl Node {
    pub fn new() -> Node {
        return Node {
            text: BytesMut::new(),
            closing: BytesMut::new(),
            parent: None,
            children: Vec::new(),
        };
    }
    pub fn append_children(&mut self, idx: usize) {
        self.children.push(idx);
    }
    pub fn push_text(&mut self, s: PushText) {
        match s {
            PushText::Slice(t) => self.text.put_slice(t),
            PushText::Byte(t) => self.text.put_u8(t),
        }
    }
    pub fn set_closing(&mut self, tag: &[u8]) {
        self.closing.put_slice(tag);
    }
}

#[derive(Debug)]
pub struct DomTree {
    size: usize,
    root: Option<usize>,
    current: Option<usize>,
    text: Bytes,
    nodes: Vec<Node>,
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
    pub fn push_current(&mut self, mut node: Node) {
        let idx = self.len();
        if let Some(current) = self.current {
            let parent = self.nodes.get_mut(current).unwrap();
            node.parent = Some(current);
            parent.append_children(idx);
        } else {
        }
        if self.root.is_none() {
            self.root = Some(idx);
        }
        self.nodes.push(node);
        self.current = Some(idx);
        self.size += 1;
    }
    pub fn pop_current(&mut self) -> Result<(), Error> {
        return match self.current {
            Some(current) => {
                self.current = self.nodes.get_mut(current).unwrap().parent;
                Ok(())
            }
            None => Err(Error::EmptyTree),
        };
    }
    pub fn get_current(&mut self) -> Option<&mut Node> {
        if let Some(current) = self.current {
            let node = self.nodes.get_mut(current).unwrap();
            return Some(node);
        } else {
            return None;
        }
    }
}
