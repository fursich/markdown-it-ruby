use markdown_it::parser::core::CoreRule;
use markdown_it::plugins::cmark::inline::{autolink::Autolink, link::Link};
use markdown_it::plugins::extra::linkify::Linkified;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

use crate::driver::{InternalDomain, MarkdonwItOptions};

#[derive(Debug)]
pub struct LinkWithTarget {
    pub url: String,
    pub title: Option<String>,
    pub target: Option<String>,
    pub rel: Option<String>,
}

impl NodeValue for LinkWithTarget {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("href", self.url.clone()));

        if let Some(target) = self.target.as_ref() {
            attrs.push(("target", target.to_string()));
        }
        if let Some(rel) = self.rel.as_ref() {
            attrs.push(("rel", rel.to_string()));
        }

        if let Some(title) = &self.title {
            attrs.push(("title", title.clone()));
        }

        fmt.open("a", &attrs);
        fmt.contents(&node.children);
        fmt.close("a");
    }
}

impl LinkWithTarget {
    pub fn new(
        url: String,
        title: Option<String>,
        internal_domain_name: Option<InternalDomain>,
    ) -> Self {
        // for internal links only
        if let Some(internal_domain_name) = internal_domain_name {
            if internal_domain_name.matches(&url) {
                return LinkWithTarget {
                    url,
                    title,
                    target: None,
                    rel: None,
                };
            }
        }

        // open external links in a new tab
        LinkWithTarget {
            url,
            title,
            target: Some("_blank".to_string()),
            rel: Some("noopener noreferrer".to_string()),
        }
    }
}

struct LinkTargetRule;

impl CoreRule for LinkTargetRule {
    // a custom function that will be invoked once per document.
    fn run(root: &mut Node, md: &MarkdownIt) {
        let options = md.ext.get::<MarkdonwItOptions>();
        let internal_domain = match options {
            None => None,
            Some(options) => options.internal_domain(),
        };

        // walk through AST recursively
        root.walk_mut(|node, _| {
            if let Some(link) = node.cast::<Link>() {
                let link_with_target = LinkWithTarget::new(
                    link.url.clone(),
                    link.title.clone(),
                    internal_domain.clone(),
                );
                node.replace::<LinkWithTarget>(link_with_target);
            } else if let Some(autolink) = node.cast::<Autolink>() {
                let link_with_target =
                    LinkWithTarget::new(autolink.url.clone(), None, internal_domain.clone());
                node.replace::<LinkWithTarget>(link_with_target);
            } else if let Some(linkified) = node.cast::<Linkified>() {
                let link_with_target =
                    LinkWithTarget::new(linkified.url.clone(), None, internal_domain.clone());
                node.replace::<LinkWithTarget>(link_with_target);
            }
        });
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<LinkTargetRule>();
}

#[test]
fn test_link_with_target() {
    use std::collections::HashMap;

    let mut md = MarkdownIt::new();

    markdown_it::plugins::cmark::add(&mut md);
    markdown_it::plugins::extra::add(&mut md);
    add(&mut md);

    {
        // without options
        {
            // for non-autolink (custom links)
            {
                // with scheme part
                {
                    // with base_url that has scheme part
                    let src = "[some title](https://kyoto.dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"https://kyoto.dosue.jp/foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n"
                    );
                }
                {
                    // with base_url with http scheme
                    let src = "[some title](http://kyoto.dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"http://kyoto.dosue.jp/foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n"
                    );
                }
                {
                    // with subdomain of base_url
                    let src = "[some title](https://fuji.kyoto.dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"https://fuji.kyoto.dosue.jp/foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n"
                    );
                }
                {
                    // with 'parent' domain of base_url
                    // this should be treated as external link
                    let src = "[some title](https://dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"https://dosue.jp/foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n"
                        );
                }
                {
                    // with different domain
                    let src = "[some title](https://nara.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"https://nara.jp/foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n"
                        );
                }
            }
            {
                // without sheme part
                {
                    // domain part is the same as base_url
                    let src = "[some title](kyoto.dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"kyoto.dosue.jp/foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n"
                    );
                }
                {
                    // domain part is different from base_url
                    // NOTE: for simplicity, any url without scheme part is treated as "internal" link
                    let src = "[some title](dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(html, "<p><a href=\"dosue.jp/foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n");
                }
                {
                    // without domain part
                    let src = "[some title](foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(html, "<p><a href=\"foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n");
                }
                {
                    // with filename
                    let src = "[some title](foo.jpg)";
                    let html = md.parse(src).render();
                    assert_eq!(html, "<p><a href=\"foo.jpg\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n");
                }
            }
        }
        {
            // for autolink urls
            {
                // with base_url
                let src = "<https://kyoto.dosue.jp>";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://kyoto.dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">kyoto.dosue.jp</a></p>\n"
                );
            }
            {
                // with child domain of base_url
                let src = "<https://pontocho.kyoto.dosue.jp>";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://pontocho.kyoto.dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">pontocho.kyoto.dosue.jp</a></p>\n"
                    );
            }
            {
                // with parent domain of base_url
                let src = "<https://dosue.jp>";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">dosue.jp</a></p>\n"
                    );
            }
            {
                // with base_url, with http scheme
                let src = "<http://kyoto.dosue.jp>";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"http://kyoto.dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">kyoto.dosue.jp</a></p>\n"
                );
            }
            {
                // external links
                let src = "<https://nara.jp>";
                let html = md.parse(src).render();
                assert_eq!(html, "<p><a href=\"https://nara.jp\" target=\"_blank\" rel=\"noopener noreferrer\">nara.jp</a></p>\n");
            }
        }
        {
            // for linkified urls
            {
                // with base_url
                let src = "https://kyoto.dosue.jp";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://kyoto.dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">kyoto.dosue.jp</a></p>\n"
                );
            }
            {
                // with child domain of base_url
                let src = "https://pontocho.kyoto.dosue.jp";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://pontocho.kyoto.dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">pontocho.kyoto.dosue.jp</a></p>\n"
                    );
            }
            {
                // with parent domain of base_url
                let src = "https://dosue.jp";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">dosue.jp</a></p>\n"
                    );
            }
            {
                // with base_url, with http scheme
                let src = "http://kyoto.dosue.jp";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"http://kyoto.dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">kyoto.dosue.jp</a></p>\n"
                );
            }
            {
                // external links
                let src = "https://nara.jp";
                let html = md.parse(src).render();
                assert_eq!(html, "<p><a href=\"https://nara.jp\" target=\"_blank\" rel=\"noopener noreferrer\">nara.jp</a></p>\n");
            }
        }
    }

    {
        // with options
        let base_url = "https://kyoto.dosue.jp".to_string();
        let options = MarkdonwItOptions::new(HashMap::from([(
            "internal_domain_name".to_string(),
            base_url.to_string(),
        )]));
        options.add(&mut md);

        {
            // for non-autolink (custom links)
            {
                // with scheme part
                {
                    // with base_url that has scheme part
                    let src = "[some title](https://kyoto.dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"https://kyoto.dosue.jp/foo/bar\">some title</a></p>\n"
                    );
                }
                {
                    // with base_url with http scheme
                    let src = "[some title](http://kyoto.dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"http://kyoto.dosue.jp/foo/bar\">some title</a></p>\n"
                    );
                }
                {
                    // with subdomain of base_url
                    let src = "[some title](https://fuji.kyoto.dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"https://fuji.kyoto.dosue.jp/foo/bar\">some title</a></p>\n"
                    );
                }
                {
                    // with 'parent' domain of base_url
                    // this should be treated as external link
                    let src = "[some title](https://dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"https://dosue.jp/foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n"
                        );
                }
                {
                    // with different domain
                    let src = "[some title](https://nara.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"https://nara.jp/foo/bar\" target=\"_blank\" rel=\"noopener noreferrer\">some title</a></p>\n"
                        );
                }
            }
            {
                // without sheme part
                {
                    // domain part is the same as base_url
                    let src = "[some title](kyoto.dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(
                        html,
                        "<p><a href=\"kyoto.dosue.jp/foo/bar\">some title</a></p>\n"
                    );
                }
                {
                    // domain part is different from base_url
                    // NOTE: for simplicity, any url without scheme part is treated as "internal" link
                    let src = "[some title](dosue.jp/foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(html, "<p><a href=\"dosue.jp/foo/bar\">some title</a></p>\n");
                }
                {
                    // without domain part
                    let src = "[some title](foo/bar)";
                    let html = md.parse(src).render();
                    assert_eq!(html, "<p><a href=\"foo/bar\">some title</a></p>\n");
                }
                {
                    // with filename
                    let src = "[some title](foo.jpg)";
                    let html = md.parse(src).render();
                    assert_eq!(html, "<p><a href=\"foo.jpg\">some title</a></p>\n");
                }
            }
        }
        {
            // for autolink urls
            {
                // with base_url
                let src = "<https://kyoto.dosue.jp>";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://kyoto.dosue.jp\">kyoto.dosue.jp</a></p>\n"
                );
            }
            {
                // with child domain of base_url
                let src = "<https://pontocho.kyoto.dosue.jp>";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://pontocho.kyoto.dosue.jp\">pontocho.kyoto.dosue.jp</a></p>\n"
                    );
            }
            {
                // with parent domain of base_url
                let src = "<https://dosue.jp>";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">dosue.jp</a></p>\n"
                    );
            }
            {
                // with base_url, with http scheme
                let src = "<http://kyoto.dosue.jp>";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"http://kyoto.dosue.jp\">kyoto.dosue.jp</a></p>\n"
                );
            }
            {
                // external links
                let src = "<https://nara.jp>";
                let html = md.parse(src).render();
                assert_eq!(html, "<p><a href=\"https://nara.jp\" target=\"_blank\" rel=\"noopener noreferrer\">nara.jp</a></p>\n");
            }
        }
        {
            // for linkified urls
            {
                // with base_url
                let src = "https://kyoto.dosue.jp";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://kyoto.dosue.jp\">kyoto.dosue.jp</a></p>\n"
                );
            }
            {
                // with child domain of base_url
                let src = "https://pontocho.kyoto.dosue.jp";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://pontocho.kyoto.dosue.jp\">pontocho.kyoto.dosue.jp</a></p>\n"
                    );
            }
            {
                // with parent domain of base_url
                let src = "https://dosue.jp";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"https://dosue.jp\" target=\"_blank\" rel=\"noopener noreferrer\">dosue.jp</a></p>\n"
                    );
            }
            {
                // with base_url, with http scheme
                let src = "http://kyoto.dosue.jp";
                let html = md.parse(src).render();
                assert_eq!(
                    html,
                    "<p><a href=\"http://kyoto.dosue.jp\">kyoto.dosue.jp</a></p>\n"
                );
            }
            {
                // external links
                let src = "https://nara.jp";
                let html = md.parse(src).render();
                assert_eq!(html, "<p><a href=\"https://nara.jp\" target=\"_blank\" rel=\"noopener noreferrer\">nara.jp</a></p>\n");
            }
        }
    }
}
