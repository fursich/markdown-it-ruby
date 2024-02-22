//! Emphasis and strong emphasis
//!
//! looks like `*this*` or `__that__`
//!
//! <https://spec.commonmark.org/0.30/#emphasis-and-strong-emphasis>
use markdown_it::generics::inline::emph_pair;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct Highlighted {
    pub marker: char,
}

impl NodeValue for Highlighted {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.open("mark", &node.attrs);
        fmt.contents(&node.children);
        fmt.close("mark");
    }
}

#[derive(Debug)]
pub struct RedColored {
    pub marker: char,
}

impl NodeValue for RedColored {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("class", "font-color-red".into()));
        fmt.open("strong", &attrs);
        fmt.contents(&node.children);
        fmt.close("strong");
    }
}

#[derive(Debug)]
pub struct Underlined {
    pub marker: char,
}

impl NodeValue for Underlined {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.open("u", &node.attrs);
        fmt.contents(&node.children);
        fmt.close("u");
    }
}

pub fn add(md: &mut MarkdownIt) {
    emph_pair::add_with::<'+', 2, false>(md, || Node::new(RedColored { marker: '+' }));
    emph_pair::add_with::<'=', 2, false>(md, || Node::new(Highlighted { marker: '=' }));
    emph_pair::add_with::<'_', 2, false>(md, || Node::new(Underlined { marker: '_' }));
}
