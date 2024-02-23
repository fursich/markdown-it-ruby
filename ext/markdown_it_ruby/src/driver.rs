mod options;

use crate::extensions;
use markdown_it::plugins::{cmark, extra, html};
use markdown_it::{MarkdownIt, Node};
pub use options::{InternalDomain, MarkdonwItOptions};
use std::collections::HashMap;
use std::sync::OnceLock;

pub(super) struct MarkdownDriver {
    md: MarkdownIt,
    source: OnceLock<String>,
    contents: OnceLock<Node>,
}

impl MarkdownDriver {
    pub(super) fn new(env: HashMap<String, String>) -> Self {
        // create markdown parser
        let mut md = MarkdownIt::new();
        let option = MarkdonwItOptions::new(env.clone());
        Self::prepare(&mut md, option);

        Self {
            md,
            source: OnceLock::new(),
            contents: OnceLock::new(),
        }
    }

    pub(super) fn parse(&self, contents: String) {
        if self.source.set(contents.clone()).is_err() {
            return;
        }

        let root = self.md.parse(contents.as_str());
        self.contents.set(root).unwrap();
    }

    pub(super) fn render(&self) -> String {
        let contents = self.contents.get();
        match contents {
            None => String::new(),
            Some(contents) => contents.render(),
        }
    }

    fn prepare(md: &mut MarkdownIt, option: MarkdonwItOptions) {
        html::add(md);
        cmark::add(md);
        extra::add(md);

        // add custom three rules described above
        extensions::add(md, &option);
        option.add(md);
    }
}
