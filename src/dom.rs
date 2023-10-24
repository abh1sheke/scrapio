use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

pub type Node = Rc<RefCell<DomNode>>;

#[derive(Debug, Clone)]
pub struct DomNode {
    pub value: String,
    pub closing: String,
    parent: Option<Node>,
    children: Vec<Node>,
}

#[derive(Debug)]
pub struct DomTree {
    size: usize,
    pub root: Option<Node>,
    current_node: Option<Node>,
}

#[derive(Debug)]
pub enum Error {
    RootExists,
    CurrentNone,
}

pub enum PushType {
    String(String),
    Char(char),
}

impl DomNode {
    pub fn new(value: String, parent: Option<Node>, children: Option<Vec<Node>>) -> Self {
        let children = if children.is_some() {
            children.unwrap()
        } else {
            vec![]
        };
        return DomNode {
            value,
            closing: String::new(),
            parent,
            children,
        };
    }
    pub fn from(node: DomNode) -> Node {
        return Rc::new(RefCell::new(node));
    }
    pub fn get_parent(&self) -> Option<Node> {
        if let Some(node) = self.parent.as_ref() {
            return Some(Rc::clone(node));
        } else {
            return None;
        }
    }
    pub fn append_children(&mut self, node: Node) {
        self.children.push(node)
    }
    pub fn iter_children(&self) -> Iter<Node> {
        return self.children.iter();
    }
}

impl DomTree {
    pub fn new() -> Self {
        return DomTree {
            size: 0,
            root: None,
            current_node: None,
        };
    }
    pub fn size(&self) -> usize {
        return self.size;
    }
    pub fn push_current(&mut self, node: DomNode) {
        let node_rc = Rc::new(RefCell::new(node));
        if let Some(current) = self.current_node.as_ref() {
            node_rc.borrow_mut().parent = Some(Rc::clone(current));
            current.borrow_mut().children.push(Rc::clone(&node_rc));
            if self.root.is_none() {
                self.root = Some(Rc::clone(&node_rc));
            }
        }
        self.current_node = Some(Rc::clone(&node_rc));
        self.size += 1;
    }
    pub fn pop_current(&mut self) -> Result<(), Error> {
        if self.current_node.is_none() {
            return Err(Error::CurrentNone);
        }
        let current = self.current_node.take();
        if let Some(current) = current.as_ref() {
            let value = current.borrow().value.clone();
            if let Some(parent) = current.borrow().parent.as_ref() {
                parent.borrow_mut().value.push_str(&value);
                self.current_node = Some(Rc::clone(parent));
            }
        } else {
            self.current_node = None;
        }
        return Ok(());
    }
    pub fn push_to_current(&mut self, tag: PushType) {
        if let Some(current) = self.current_node.as_ref() {
            match tag {
                PushType::String(s) => current.borrow_mut().value.push_str(&s),
                PushType::Char(c) => current.borrow_mut().value.push(c),
            }
        }
    }
    pub fn push_to_closing(&mut self, tag: String) {
        if let Some(current) = self.current_node.as_ref() {
            current.borrow_mut().closing = tag;
        }
    }
    fn to_string_aux(&self, node: Node) -> String {
        let mut s = String::new();
        let current = node.borrow();
        s.push_str(&current.value);
        if current.children.len() > 0 {
            current
                .iter_children()
                .for_each(|i| s.push_str(&self.to_string_aux(Rc::clone(i))));
        }
        return s;
    }
}

impl std::fmt::Display for DomTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            self.to_string_aux(Rc::clone(&self.root.as_ref().unwrap()))
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom() -> Result<(), Error> {
        return Ok(());
    }
}
