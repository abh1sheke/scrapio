use bytes::Bytes;
use std::ops::Range;

use crate::dom;

#[derive(Debug)]
enum TagKind {
    Open(Range<usize>),
    Close(Range<usize>),
    SelfClosing(Range<usize>),
    Void(Range<usize>),
    Comment(Range<usize>),
}

fn extract_tag(html: &Bytes, ptr: &mut usize) -> TagKind {
    let start = *ptr;
    loop {
        let tkn = html[*ptr];
        *ptr += 1;
        if tkn == b'>' {
            break;
        }
    }
    let end = *ptr;

    match &html[start..end] {
        [b'<', b'/', ..] => TagKind::Close(start..end),
        [.., b'/', b'>'] => TagKind::SelfClosing(start..end),
        [b'<', b'!', b'-', b'-', ..] => TagKind::Comment(start..end),
        [b'<', b'b', b'r', ..] => TagKind::Void(start..end),
        [b'<', b'h', b'r', ..] => TagKind::Void(start..end),
        [b'<', b'c', b'o', b'l', ..] => TagKind::Void(start..end),
        [b'<', b'i', b'm', b'g', ..] => TagKind::Void(start..end),
        [b'<', b'w', b'b', b'r', ..] => TagKind::Void(start..end),
        [b'<', b'a', b'r', b'e', b'a', ..] => TagKind::Void(start..end),
        [b'<', b'b', b'a', b's', b'e', ..] => TagKind::Void(start..end),
        [b'<', b'l', b'i', b'n', b'k', ..] => TagKind::Void(start..end),
        [b'<', b'm', b'e', b't', b'a', ..] => TagKind::Void(start..end),
        [b'<', b'e', b'm', b'b', b'e', b'd', ..] => TagKind::Void(start..end),
        [b'<', b'i', b'n', b'p', b'u', b't', ..] => TagKind::Void(start..end),
        [b'<', b'p', b'a', b'r', b'a', b'm', ..] => TagKind::Void(start..end),
        [b'<', b't', b'r', b'a', b'c', b'k', ..] => TagKind::Void(start..end),
        [b'<', b'k', b'e', b'y', b'g', b'e', b'n', ..] => TagKind::Void(start..end),
        [b'<', b's', b'o', b'u', b'r', b'c', b'e', ..] => TagKind::Void(start..end),
        [b'<', b'c', b'o', b'm', b'm', b'a', b'n', b'd', ..] => TagKind::Void(start..end),
        _ => TagKind::Open(start..end),
    }
}

fn push_self_closer(tree: &mut dom::DomTree, html: &Bytes, t: Range<usize>) {
    let mut node = dom::Node::new();
    node.push_text(dom::PushText::Slice(&html[t.clone()]));
    tree.push_current(node);
    tree.pop_current().unwrap();
}

pub fn parse(html: &Bytes) -> Result<dom::DomTree, dom::Error> {
    let mut tree = dom::DomTree::new();
    tree.set_text(html.clone());

    let len = html.len();
    let mut ptr: usize = 0;
    while ptr < len {
        let tkn = html[ptr];
        match tkn {
            b'<' => {
                let tag = extract_tag(&html, &mut ptr);
                match tag {
                    TagKind::Open(t) => {
                        let mut node = dom::Node::new();
                        node.push_text(dom::PushText::Slice(&html[t]));
                        tree.push_current(node);
                    }
                    TagKind::Close(t) => {
                        if let Some(current) = tree.get_current() {
                            current.set_closing(&html[t]);
                        }
                        tree.pop_current()?;
                    }
                    TagKind::SelfClosing(t) => {
                        push_self_closer(&mut tree, &html, t);
                    }
                    TagKind::Void(t) => {
                        push_self_closer(&mut tree, &html, t);
                    }
                    _ => {}
                }
            }
            _ => {
                if let Some(current) = tree.get_current() {
                    current.push_text(dom::PushText::Byte(tkn));
                }
                ptr += 1
            }
        }
    }

    return Ok(tree);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use std::time::Instant;

    #[test]
    fn test_parser() -> Result<(), Box<dyn std::error::Error>> {
        let example_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/example.html");
        let mut f = fs::File::open(example_path)?;

        let mut b: Vec<u8> = Vec::with_capacity(f.metadata()?.len() as usize);
        f.read_to_end(&mut b)?;

        let b = Bytes::from(b);

        let start = Instant::now();
        let _ = parse(&b)?;
        println!("took: {:?}", start.elapsed());

        return Ok(());
    }

    #[test]
    fn test_parser_perf() -> Result<(), Box<dyn std::error::Error>> {
        let example_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/example.html");
        let mut f = fs::File::open(example_path)?;

        let mut b: Vec<u8> = Vec::with_capacity(f.metadata()?.len() as usize);
        f.read_to_end(&mut b)?;

        let b = Bytes::from(b);

        let runs = 20;
        let mut total_time: u128 = 0;
        for _ in 0..runs {
            let start = Instant::now();
            let _ = parse(&b)?;
            total_time += start.elapsed().as_millis();
        }
        let avg: f64 = total_time as f64 / runs as f64;
        println!("avg duration: {avg}");

        return Ok(());
    }
}
