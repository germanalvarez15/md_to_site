use std::path::{Path, PathBuf};
use std::fs;

pub struct Theme {
  pub template_path: PathBuf,
  pub css_path: PathBuf,
}

pub fn get_theme(theme_name: &str) -> Result<Theme, String> {
  let templates_dir = PathBuf::from("templates");
  let theme_dir = templates_dir.join(theme_name);

  if !theme_dir.exists() {
    return Err(format!("Theme '{}' does not exist.", theme_name));
  }

  let template_path = theme_dir.join("template.html");
  let css_path = theme_dir.join("style.css");

  if !template_path.exists() || !css_path.exists() {
    return Err(format!("Theme '{}' is missing required files.", theme_name));
  }

  Ok(Theme {
    template_path,
    css_path,
  })
}

pub fn list_available_themes() -> Vec<String> {
  let templates_dir = PathBuf::from("templates");
  let mut themes = Vec::new();

  if let Ok(entries) = std::fs::read_dir(templates_dir) {
    for entry in entries {
      if let Ok(entry) = entry {
        if entry.path().is_dir() {
          if let Some(theme_name) = entry.file_name().to_str() {
            themes.push(theme_name.to_string());
          }
        }
      }
    }
  }

  themes
}