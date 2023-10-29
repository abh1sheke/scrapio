use bytes::{BufMut, BytesMut};

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
        self.text.put_slice(tag);
    }
}

#[derive(Debug)]
pub enum Error {
    TreeEmpty,
}

#[derive(Debug)]
pub struct DomTree {
    size: usize,
    root: Option<usize>,
    current: Option<usize>,
    text: BytesMut,
    nodes: Vec<Node>,
}

impl DomTree {
    pub fn new() -> Self {
        return DomTree {
            size: 0,
            root: None,
            current: None,
            text: BytesMut::new(),
            nodes: Vec::new(),
        };
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
            None => Err(Error::TreeEmpty),
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
