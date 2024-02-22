use markdown_it::parser::core::CoreRule;
use markdown_it::plugins::cmark::block::heading::ATXHeading;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
struct PlainTextElement {
    text: String,
}

impl NodeValue for PlainTextElement {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.text(&self.text);
        fmt.contents(&node.children);
        fmt.cr();
    }
}

// This is an extension for the markdown parser.
struct HeadingRule;

impl CoreRule for HeadingRule {
    // This is a custom function that will be invoked once per document.
    //
    // It has `root` node of the AST as an argument and may modify its
    // contents as you like.
    //
    fn run(root: &mut Node, _: &MarkdownIt) {
        // walk through AST recursively and count the number of two
        // custom nodes added by other two rules
        root.walk_mut(|node, _| {
            if let Some(heading) = node.cast::<ATXHeading>() {
                if heading.level == 1 || heading.level > 4 {
                    node.replace(PlainTextElement {
                        text: "#".repeat(heading.level as usize),
                    });
                }
            }
        });
    }
}

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into parser
    md.add_rule::<HeadingRule>();
}
