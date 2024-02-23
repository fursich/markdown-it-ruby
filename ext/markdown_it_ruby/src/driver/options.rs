use markdown_it::parser::extset::MarkdownItExt;
use markdown_it::MarkdownIt;
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
pub struct MarkdonwItOptions {
    options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct InternalDomain {
    base_url: Url,
}

impl MarkdownItExt for MarkdonwItOptions {}
impl Clone for MarkdonwItOptions {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}
impl Default for MarkdonwItOptions {
    fn default() -> Self {
        Self {
            options: HashMap::new(),
        }
    }
}

impl MarkdonwItOptions {
    pub fn new(options: HashMap<String, String>) -> Self {
        Self { options }
    }

    pub fn add(self, md: &mut MarkdownIt) {
        md.ext.insert::<Self>(self);
    }

    pub fn is_enabled(&self, key: &str, default: bool) -> bool {
        match self.get_option_or_default(key, "_NOT_APPLICABLE").as_str() {
            "true" => true,
            "false" => false,
            _ => default,
        }
    }

    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.options.get(key)
    }

    pub fn get_option_or_default(&self, key: &str, default: &str) -> String {
        self.options
            .get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    pub fn internal_domain(&self) -> Option<InternalDomain> {
        let domain_name = self.get_option("internal_domain_name");
        match domain_name {
            None => None,
            Some(domain_name) => InternalDomain::new(domain_name.clone()),
        }
    }
}

impl InternalDomain {
    fn new(domain_name: String) -> Option<Self> {
        if let Ok(base_url) = Url::parse(domain_name.as_str()) {
            if base_url.domain().is_some() {
                return Some(InternalDomain { base_url });
            }
        }

        let domain_with_scheme = format!("https://{}", domain_name);
        if let Ok(base_url) = Url::parse(&domain_with_scheme) {
            if base_url.domain().is_some() {
                return Some(InternalDomain { base_url });
            }
        }

        None
    }

    pub fn matches(&self, url: &str) -> bool {
        // let url_parser = Url::options().base_url(Some(&self.base_url));
        let internal_domain_name = self.base_url.domain().unwrap();

        // url with scheme
        if Self::check_scheme(url) {
            return self.check_internal_domain(url, internal_domain_name, false);
        }

        // 一部の「schemeなし、パス付きURL表記」をヒューリスティックに判定できるが
        // 仕様が複雑になるためいったんやらない
        // （google.com/foo は外部サイトとして判定しうるが、google.com は foo.pdf
        // と判別がつかないため内部リンク扱いとなり、ルールがわかりにくくなるため）
        // // possiblly a domain name
        // // heuristics: if the url contains a dot, it's a relative path with domain
        // // we do not regard them as internal links
        // // e.g. google.com/foo/bar
        // let domain_regex = regex!(r"^[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)+/");
        // if domain_regex.is_match(url) {
        //     println!(
        //         "check #2 - url: {}, internal_domain_name: {}",
        //         url, internal_domain_name
        //     );
        //     return false;
        // }

        // should be a relative path
        // NOTE: google.com（外部サイト）, google.pdf（ファイル名）は区別がつかないため全て内部リンク扱いとする
        // 外部サイトの場合はスキームをつけること
        self.check_internal_domain(&url, internal_domain_name, true)
    }

    fn check_scheme(url: &str) -> bool {
        if let Ok(url) = Url::parse(url) {
            if !url.scheme().is_empty() {
                return true;
            }
        }
        false
    }

    fn check_internal_domain(
        &self,
        url: &str,
        internal_domain_name: &str,
        is_base_url_required: bool,
    ) -> bool {
        let url = if is_base_url_required {
            Url::options().base_url(Some(&self.base_url)).parse(url)
        } else {
            Url::parse(url)
        };
        if let Ok(url) = url {
            if let Some(domain) = url.domain() {
                // for `example.com`, `foo.example.com` is also considered as an internal link
                if domain == internal_domain_name
                    || domain.ends_with(format!(".{}", internal_domain_name).as_str())
                {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test_get_option() {
    let raw_options = HashMap::from([
        ("some_random_option".to_string(), "true".to_string()),
        ("another_option".to_string(), "some-string".to_string()),
    ]);
    let options = MarkdonwItOptions::new(raw_options);

    assert_eq!(
        options.get_option("some_random_option"),
        Some(&"true".to_string())
    );
    assert_eq!(
        options.get_option("another_option"),
        Some(&"some-string".to_string())
    );
}

#[test]
fn test_get_option_or_default() {
    let raw_options = HashMap::from([("some_random_option".to_string(), "true".to_string())]);
    let options = MarkdonwItOptions::new(raw_options);

    assert_eq!(
        options.get_option_or_default("some_random_option", "false"),
        "true".to_string()
    );
    assert_eq!(
        options.get_option_or_default("another_option", "default value"),
        "default value".to_string()
    );
}

#[test]
fn test_is_enabled() {
    {
        // with values set to "true"
        let raw_options = HashMap::from([("some_option".to_string(), "true".to_string())]);
        let options = MarkdonwItOptions::new(raw_options);
        assert_eq!(options.is_enabled("some_option", false), true);
        assert_eq!(options.is_enabled("some_option", true), true);
    }

    {
        // with values set to "false"
        let raw_options = HashMap::from([("some_option".to_string(), "false".to_string())]);
        let options = MarkdonwItOptions::new(raw_options);
        assert_eq!(options.is_enabled("some_option", false), false);
        assert_eq!(options.is_enabled("some_option", true), false);
    }

    {
        // with values set to other than "true" or "false"
        let raw_options =
            HashMap::from([("some_option".to_string(), "some_random-value".to_string())]);
        let options = MarkdonwItOptions::new(raw_options);
        assert_eq!(options.is_enabled("some_option", false), false);
        assert_eq!(options.is_enabled("some_option", true), true);
    }

    {
        // with no value set
        let options = MarkdonwItOptions::new(HashMap::new());
        assert_eq!(options.is_enabled("some_option", false), false);
        assert_eq!(options.is_enabled("some_option", true), true);
    }
}

#[test]
fn test_internal_domain_matches() {
    {
        // with base_url that has scheme part
        let base_url = "https://shizuoka.jp".to_string();
        let internal_domain = InternalDomain::new(base_url).unwrap();

        {
            // when the url shares the same domain
            assert_eq!(internal_domain.matches("https://shizuoka.jp"), true);
            assert_eq!(internal_domain.matches("https://fuji.shizuoka.jp"), true);
            assert_eq!(internal_domain.matches("http://fuji.shizuoka.jp"), true);
            assert_eq!(
                internal_domain.matches("https://www.city.fuji.shizuoka.jp"),
                true
            );
        }

        {
            // when the url does not share the same domain
            assert_eq!(internal_domain.matches("https://fuji.shizu-oka.jp"), false);
        }

        {
            // when the url does not have a scheme part
            assert_eq!(internal_domain.matches("fuji.shizuoka.jp"), true);
            assert_eq!(internal_domain.matches("www.city.fuji.shizuoka.jp"), true);
            assert_eq!(
                internal_domain.matches("totally-different-domain.com"),
                true
            );
        }

        {
            //with relative path
            assert_eq!(internal_domain.matches("/foo/bar"), true);
            assert_eq!(internal_domain.matches("/foo/bar/baz.jpg"), true);
        }
    }

    {
        // with base_url that does NOT have scheme part
        let base_url = "shizuoka.jp".to_string();
        let internal_domain = InternalDomain::new(base_url).unwrap();

        {
            // when the url shares the same domain
            assert_eq!(internal_domain.matches("https://shizuoka.jp"), true);
            assert_eq!(internal_domain.matches("https://fuji.shizuoka.jp"), true);
            assert_eq!(internal_domain.matches("http://fuji.shizuoka.jp"), true);
            assert_eq!(
                internal_domain.matches("https://www.city.fuji.shizuoka.jp"),
                true
            );
        }

        {
            // when the url does not share the same domain
            assert_eq!(internal_domain.matches("https://fuji.shizu-oka.jp"), false);
        }

        {
            // when the url does not have a scheme part
            assert_eq!(internal_domain.matches("fuji.shizuoka.jp"), true);
            assert_eq!(internal_domain.matches("www.city.fuji.shizuoka.jp"), true);
            assert_eq!(
                internal_domain.matches("totally-different-domain.com"),
                true
            );
        }

        {
            //with relative path
            assert_eq!(internal_domain.matches("/foo/bar"), true);
            assert_eq!(internal_domain.matches("/foo/bar/baz.jpg"), true);
        }
    }
}
