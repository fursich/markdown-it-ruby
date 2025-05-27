use crate::driver::MarkdonwItOptions;
use markdown_it::parser::core::CoreRule;
use markdown_it::plugins::extra::tables::Table;
use markdown_it::{MarkdownIt, Node};

#[derive(Debug)]
struct TableDecorationRule;

impl CoreRule for TableDecorationRule {
    // a custom function that will be invoked once per document.
    fn run(root: &mut Node, md: &MarkdownIt) {
        let options = md.ext.get::<MarkdonwItOptions>();
        let table_class_name = match options {
            Some(options) => options.get_option_or_default("table_class_name", "table"),
            None => "table".to_string(),
        };
        root.walk_mut(|node, _| {
            if let Some(table) = node.cast::<Table>() {
                node.replace(Table {
                    alignments: table.alignments.clone(),
                });
                node.attrs = vec![("class", table_class_name.clone())];
            }
        });
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<TableDecorationRule>();
}

#[test]
fn test_table_decoration() {
    use std::collections::HashMap;

    let mut md = MarkdownIt::new();

    markdown_it::plugins::cmark::add(&mut md);
    markdown_it::plugins::extra::add(&mut md);
    add(&mut md);

    {
        // without options
        let src = "| 左寄せタイトル | センタリング | 右寄せタイトル |\n |:------------|:------------:|-----------:|\n | column | column | column |\n";
        let html = md.parse(src).render();

        assert_eq!(
            html,
            "<table class=\"table\">\n<thead>\n<tr>\n<th style=\"text-align:left\">左寄せタイトル</th>\n<th style=\"text-align:center\">センタリング</th>\n<th style=\"text-align:right\">右寄せタイトル</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style=\"text-align:left\">column</td>\n<td style=\"text-align:center\">column</td>\n<td style=\"text-align:right\">column</td>\n</tr>\n</tbody>\n</table>\n"
        )
    }

    {
        // with options
        let options = MarkdonwItOptions::new(HashMap::from([(
            "table_class_name".to_string(),
            "custom-table-class-name".to_string(),
        )]));
        options.add(&mut md);

        let src = "| 左寄せタイトル | センタリング | 右寄せタイトル |\n |:------------|:------------:|-----------:|\n | column | column | column |\n";
        let html = md.parse(src).render();

        assert_eq!(
            html,
            "<table class=\"custom-table-class-name\">\n<thead>\n<tr>\n<th style=\"text-align:left\">左寄せタイトル</th>\n<th style=\"text-align:center\">センタリング</th>\n<th style=\"text-align:right\">右寄せタイトル</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style=\"text-align:left\">column</td>\n<td style=\"text-align:center\">column</td>\n<td style=\"text-align:right\">column</td>\n</tr>\n</tbody>\n</table>\n"
        )
    }
}
