mod driver;
mod extensions;
mod initial_data;
use magnus::{define_module, function, prelude::*, Error};

use driver::MarkdownHandler;

fn parse(content: String) -> String {
    let handler = MarkdownHandler::new();
    handler.parse(content).unwrap();
    handler.render()
}

fn parse_default() -> String {
    let handler = MarkdownHandler::new();
    let content = initial_data::load();
    handler.parse(content).unwrap();
    handler.render()
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("MarkdownRustified")?;
    module.define_singleton_method("parse", function!(parse, 1))?;
    module.define_singleton_method("parse_default", function!(parse_default, 0))?;

    Ok(())
}
