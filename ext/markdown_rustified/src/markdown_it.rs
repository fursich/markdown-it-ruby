mod extensions;

fn main(md: &mut markdown_it::MarkdownIt) {
    markdown_it::plugins::html::add(md);
    // add commonmark syntax, you almost always want to do that
    markdown_it::plugins::cmark::add(md);
    markdown_it::plugins::extra::strikethrough::add(md);
    markdown_it::plugins::extra::tables::add(md);

    // add custom three rules described above
    emphasis::add(md);
    headings::add(md);
    link::add(md);
    table_of_contents::add(md);
}
