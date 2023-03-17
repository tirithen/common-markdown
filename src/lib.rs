use pulldown_cmark::{html, Parser};
use wasm_bindgen::prelude::wasm_bindgen;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse(markdown: &str) -> String {
    let mut html_result = String::new();

    let parser = Parser::new(markdown);
    html::push_html(&mut html_result, parser);

    html_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let markdown = "# Common Markdown\n\nThis is a paragraph";
        let expected_html = "<h1>Common Markdown</h1>\n<p>This is a paragraph</p>\n";

        let html = parse(markdown);

        assert_eq!(html, expected_html);
    }
}