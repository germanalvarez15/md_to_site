use std::fs;
use std::path::Path;
use pulldown_cmark::{Parser, html};

pub fn read_markdown_file(file_path: &Path) -> std::io::Result<String> {
  let content = fs::read_to_string(file_path)?;
  Ok(content)
}

pub fn convert_markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
