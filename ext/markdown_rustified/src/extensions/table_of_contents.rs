use markdown_it::common::utils::escape_html;
use markdown_it::parser::core::CoreRule;
use markdown_it::parser::extset::RenderExtSet;
use markdown_it::plugins::cmark::block::heading::ATXHeading;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct ToCList {
    pub elements: Vec<ToCElement>,
}

#[derive(Debug)]
pub struct ToCElement {
    pub id: String,
    pub level: u32,
    pub text: String,
}

#[derive(Debug, Default)]
/// Render as plain text (stripping all tags)
pub(crate) struct PlainTextRenderer {
    result: String,
    pub ext: RenderExtSet,
}

impl PlainTextRenderer {
    pub fn new() -> Self {
        Self {
            result: String::new(),
            ext: RenderExtSet::default(),
        }
    }

    pub fn render(&mut self, node: &Node) {
        node.node_value.render(node, self);
    }
}

impl From<PlainTextRenderer> for String {
    fn from(f: PlainTextRenderer) -> Self {
        #[cold]
        fn replace_null(input: String) -> String {
            input.replace('\0', "\u{FFFD}")
        }

        if f.result.contains('\0') {
            // U+0000 must be replaced with U+FFFD as per commonmark spec,
            // we do it at the very end in order to avoid messing with byte offsets
            // for source maps (since "\0".len() != "\u{FFFD}".len())
            replace_null(f.result)
        } else {
            f.result
        }
    }
}

impl Renderer for PlainTextRenderer {
    fn open(&mut self, _tag: &str, _attrs: &[(&str, String)]) {
        // do nothing
    }

    fn close(&mut self, _tag: &str) {
        // do nothing
    }

    fn self_close(&mut self, _tag: &str, _attrs: &[(&str, String)]) {
        // do nothing
    }

    fn contents(&mut self, nodes: &[Node]) {
        for node in nodes.iter() {
            self.render(node);
        }
    }

    fn cr(&mut self) {
        // do nothing
    }

    fn text(&mut self, text: &str) {
        self.result.push_str(&escape_html(text));
    }

    fn text_raw(&mut self, text: &str) {
        self.result.push_str(text);
    }

    fn ext(&mut self) -> &mut RenderExtSet {
        // do nothing
        &mut self.ext
    }
}

// This defines how your custom node should be rendered.
impl NodeValue for ToCList {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();

        // add a custom class attribute
        attrs.push(("class", "table-of-contents".into()));

        fmt.open("h2", &attrs);
        fmt.text("Table of Contents");
        fmt.close("h2");

        fmt.cr(); // linebreak, multiples get merged
        fmt.open("ul", &attrs);
        fmt.cr();
        for element in self.elements.iter() {
            fmt.open("li", &attrs);
            fmt.open(
                "a",
                &[
                    ("href", format!("#{}", element.id)),
                    ("class", format!("toc-level-{}", element.level)),
                ],
            );
            fmt.text(&element.text);
            fmt.close("a");
            fmt.close("li");
            fmt.cr();
        }
        fmt.cr();
        fmt.close("ul");
        fmt.cr();
        fmt.cr();
    }
}

// This is an extension for the markdown parser.
struct ToCCounterRule;

impl CoreRule for ToCCounterRule {
    // This is a custom function that will be invoked once per document.
    //
    // It has `root` node of the AST as an argument and may modify its
    // contents as you like.
    //
    fn run(root: &mut Node, _: &MarkdownIt) {
        let mut counter = 0;
        let mut toc_list = Vec::new();

        // walk through AST recursively and count the number of two
        // custom nodes added by other two rules
        root.walk_mut(|node, _| {
            if let Some(heading) = node.cast::<ATXHeading>() {
                if heading.level > 1 && heading.level <= 4 {
                    let level = heading.level;

                    let id = format!("heading-{}", counter);
                    node.attrs.push(("id", id.clone().into()));

                    let mut fmt = PlainTextRenderer::new();
                    fmt.render(&node);
                    let text = String::from(fmt);

                    let element = ToCElement {
                        id,
                        level: level.into(),
                        text,
                    };
                    toc_list.push(element);
                    counter += 1;
                }
            }
        });

        // append a counter to the root as a custom node
        root.children
            .push(Node::new(ToCList { elements: toc_list }));
    }
}

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into parser
    md.add_rule::<ToCCounterRule>();
}
