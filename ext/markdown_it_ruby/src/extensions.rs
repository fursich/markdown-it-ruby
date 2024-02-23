pub(super) mod heading_level_modification;
pub(super) mod link_with_target;
pub(super) mod table_decoration;

use crate::driver::MarkdonwItOptions;
use markdown_it::MarkdownIt;

pub(super) fn add(md: &mut MarkdownIt, option: &MarkdonwItOptions) {
    if option.is_enabled("heading_level_offset", true) {
        heading_level_modification::add(md);
    }
    if option.is_enabled("internal_domain_name", true) {
        link_with_target::add(md);
    }
    if option.is_enabled("table_class_name", true) {
        table_decoration::add(md);
    }
}
