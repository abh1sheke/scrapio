use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct Node {
    kind: Bytes,
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
            kind: Bytes::new(),
            text: BytesMut::new(),
            closing: BytesMut::new(),
            parent: None,
            children: Vec::new(),
        };
    }
    pub fn get_kind(&self) -> &Bytes {
        return &self.kind;
    }
    pub fn set_parent(&mut self, parent: Option<usize>) {
        self.parent = parent;
    }
    pub fn get_parent(&self) -> Option<usize> {
        return self.parent;
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
