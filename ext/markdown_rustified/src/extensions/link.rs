//! Links
//!
//! `![link](<to> "stuff")`
//!
//! <https://spec.commonmark.org/0.30/#links>
use markdown_it::generics::inline::full_link;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};
use url::Url;

#[derive(Debug)]
pub struct LinkWithTarget {
    pub url: String,
    pub title: Option<String>,
}

impl NodeValue for LinkWithTarget {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("href", self.url.clone()));

        if !is_internal_domain(&self.url) {
            attrs.push(("target", "_blank".to_string()));
            attrs.push(("rel", "noopener noreferrer".to_string()));
        }

        if let Some(title) = &self.title {
            attrs.push(("title", title.clone()));
        }

        fmt.open("a", &attrs);
        fmt.contents(&node.children);
        fmt.close("a");
    }
}

fn is_internal_domain(url: &str) -> bool {
    // TODO: env varから取得する
    const INTERNAL_DOMAIN_NAME: &str = "google.com";

    if check_internal_domain(url, INTERNAL_DOMAIN_NAME) {
        return true;
    }

    // scheme, domainがない場合は、https://INTERNAL_DOMAIN_NAME/を付与してパースを試みる
    // 例：パス指定（/path/to/somewhere）
    let url_with_domain = format!("https://{}{}", INTERNAL_DOMAIN_NAME, url);
    check_internal_domain(url_with_domain.as_str(), INTERNAL_DOMAIN_NAME)
}

fn check_internal_domain(url: &str, internal_domain_name: &str) -> bool {
    // schemeがない場合
    if url.starts_with(format!("{}/", internal_domain_name).as_str()) {
        return true;
    }

    // schemeがある場合はパースを試みる
    if let Ok(url) = Url::parse(&url) {
        if let Some(domain) = url.domain() {
            if domain == internal_domain_name {
                return true;
            }
            if domain.starts_with("www.") && &domain[4..] == internal_domain_name {
                return true;
            }
        }
    }
    false
}

pub fn add(md: &mut MarkdownIt) {
    full_link::add::<false>(md, |href, title| {
        let url = href.unwrap_or_default();
        Node::new(LinkWithTarget { url, title })
    });
}
