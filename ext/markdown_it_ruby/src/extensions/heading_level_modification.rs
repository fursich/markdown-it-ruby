use crate::driver::MarkdonwItOptions;
use markdown_it::parser::core::CoreRule;
use markdown_it::plugins::cmark::block::heading::ATXHeading;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
struct PlainTextElement {
    text: String,
}

impl NodeValue for PlainTextElement {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.open("p", &[]);
        fmt.text(&self.text);
        fmt.contents(&node.children);
        fmt.close("p");
        fmt.cr();
    }
}

struct HeadingLevelModificationRule;

impl CoreRule for HeadingLevelModificationRule {
    fn run(root: &mut Node, md: &MarkdownIt) {
        let options = md.ext.get::<MarkdonwItOptions>();
        let heading_level_offset = match options {
            Some(options) => options
                .get_option_or_default("heading_level_offset", "0")
                .parse::<u8>()
                .unwrap_or(0),
            None => 0,
        };
        // walk through AST recursively and count the number of two
        // custom nodes added by other two rules
        root.walk_mut(|node, _| {
            if let Some(heading) = node.cast::<ATXHeading>() {
                // do not render level 1 and level > 4 headings (# and ####+)
                // illegal `headings` has to be treated as plain text
                let new_heading_level = heading.level + heading_level_offset;
                // if the new heading level is out of the accepted range (1-6), treat it as plain text
                if new_heading_level < 1 || new_heading_level > 6 {
                    node.replace(PlainTextElement {
                        text: "#".repeat(heading.level as usize) + " ",
                    });
                } else {
                    node.replace(ATXHeading {
                        level: new_heading_level,
                    });
                }
            }
        });
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<HeadingLevelModificationRule>();
}

#[test]
fn test_heading_modification() {
    use std::collections::HashMap;

    let mut md = MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut md);
    add(&mut md);

    {
        // without options
        {
            // for heading levels out of the accepted range (h7)
            let src = "####### heading 7";
            let html = md.parse(src).render();
            assert_eq!(html, "<p>####### heading 7</p>\n");
        }

        {
            // for heading levels within the accepted range(h1-h6)
            let src = "# heading 1";
            let html = md.parse(src).render();
            assert_eq!(html, "<h1>heading 1</h1>\n");

            let src = "### heading 3";
            let html = md.parse(src).render();
            assert_eq!(html, "<h3>heading 3</h3>\n");

            let src = "###### heading 6";
            let html = md.parse(src).render();
            assert_eq!(html, "<h6>heading 6</h6>\n");
        }

        let src =
            "# heading 1\n## heading 2\n### heading 3\n#### heading 4\n##### heading 5\n###### heading 6\n####### heading 7";
        let html = md.parse(src).render();

        assert_eq!(
            html,
            "<h1>heading 1</h1>\n<h2>heading 2</h2>\n<h3>heading 3</h3>\n<h4>heading 4</h4>\n<h5>heading 5</h5>\n<h6>heading 6</h6>\n<p>####### heading 7</p>\n"
            );
    }

    {
        // with options
        let options = MarkdonwItOptions::new(HashMap::from([(
            "heading_level_offset".to_string(),
            "3".to_string(),
        )]));
        options.add(&mut md);
        {
            // for heading levels out of the accepted range (h4 - shifted to h7)
            let src = "#### heading 4";
            let html = md.parse(src).render();
            assert_eq!(html, "<p>#### heading 4</p>\n");
        }

        {
            // for heading levels within the accepted range(h1-h3)
            let src = "# heading 1";
            let html = md.parse(src).render();
            assert_eq!(html, "<h4>heading 1</h4>\n");

            let src = "## heading 2";
            let html = md.parse(src).render();
            assert_eq!(html, "<h5>heading 2</h5>\n");

            let src = "### heading 3";
            let html = md.parse(src).render();
            assert_eq!(html, "<h6>heading 3</h6>\n");
        }

        let src =
            "# heading 1\n## heading 2\n### heading 3\n#### heading 4\n##### heading 5\n###### heading 6\n####### heading 7";
        let html = md.parse(src).render();

        assert_eq!(
            html,
            "<h4>heading 1</h4>\n<h5>heading 2</h5>\n<h6>heading 3</h6>\n<p>#### heading 4</p>\n<p>##### heading 5</p>\n<p>###### heading 6</p>\n<p>####### heading 7</p>\n"
            );
    }
}
