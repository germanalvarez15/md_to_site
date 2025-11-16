use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Tera, Context};

pub fn generate_html(
  markdown_content_html: String,
  page_title: String,
  template_path: &Option<PathBuf>
) -> Result<String> {
  let mut context = Context::new();
  context.insert("content", &markdown_content_html);
  context.insert("title", &page_title);
  let mut tera: Tera;

  if let Some(path) = template_path {
    tera = Tera::new(path.to_str().unwrap())?;
    let rendered = tera.render("template.html", &context)?;
    Ok(rendered)
  } else {
    let default_template = r#"
      <!DOCTYPE html>
      <html lang="en">
      <head>
          <meta charset="UTF-8">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
          <title>{{ title }}</title>
      </head>
      <body>
          {{ content | safe }}
      </body>
      </html>
    "#;

    tera = Tera::default();
    tera.add_raw_template("default.html", default_template)?;
    let rendered = tera.render("default.html", &context)?;
    Ok(rendered)
  }
}