mod driver;
mod extensions;
use magnus::{define_module, function, prelude::*, Error};
use std::collections::HashMap;

// macro for regex
//
// usage:
//
// use crate::regex;
// let domain_regex = regex!(r"^[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)+/");
// if domain_regex.is_match(url) { ...
#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

use driver::MarkdownDriver;

fn convert(contents: String, options: HashMap<String, String>) -> String {
    let handler = MarkdownDriver::new(options);
    handler.parse(contents);
    handler.render()
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("MarkdownIt")?;
    module.define_singleton_method("__convert", function!(convert, 2))?;

    Ok(())
}
