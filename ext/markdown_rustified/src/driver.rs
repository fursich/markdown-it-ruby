use crate::extensions;
use markdown_it::{MarkdownIt, Node};
use std::sync::OnceLock;

pub(super) struct MarkdownHandler {
    md: markdown_it::MarkdownIt,
    ast: OnceLock<Node>,
}

impl MarkdownHandler {
    pub(super) fn new() -> Self {
        // create markdown parser
        let mut md = MarkdownIt::new();
        Self::prepare(&mut md);

        Self {
            md,
            ast: OnceLock::new(),
        }
    }

    pub(super) fn parse(&self, content: String) -> Result<(), Node> {
        let ast = self.md.parse(content.as_str());
        self.ast.set(ast)
    }

    pub(super) fn render(&self) -> String {
        let ast = self.ast.get();
        if let Some(ast) = ast {
            ast.render()
        } else {
            String::new()
        }
    }

    fn prepare(md: &mut MarkdownIt) {
        markdown_it::plugins::html::add(md);
        // add commonmark syntax, you almost always want to do that
        markdown_it::plugins::cmark::add(md);
        markdown_it::plugins::extra::strikethrough::add(md);
        markdown_it::plugins::extra::tables::add(md);

        // add custom three rules described above
        extensions::emphasis::add(md);
        extensions::headings::add(md);
        extensions::link::add(md);
        extensions::table_of_contents::add(md);
    }
}
