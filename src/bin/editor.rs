use dioxus::prelude::*;
use md_to_site::mods::file_manager::convert_markdown_to_html;
use md_to_site::mods::theme_manager::{list_available_themes, get_theme};
use std::fs;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut markdown = use_signal(|| String::from("# Welcome to Markdown Live Editor\n\nStart typing your markdown here...\n\n## Features\n\n- **Live preview** as you type\n- Supports all markdown syntax\n- Clean split-pane interface\n\n## Example Code\n\n```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```\n\n### Lists\n\n- Item 1\n- Item 2\n- Item 3\n\n### Blockquote\n\n> This is a blockquote example.\n> It can span multiple lines.\n"));
    
    let mut selected_theme = use_signal(|| "default".to_string());
    
    // Load theme CSS
    let theme_css = use_memo(move || {
        if let Ok(theme) = get_theme(&selected_theme()) {
            fs::read_to_string(&theme.css_path).unwrap_or_default()
        } else {
            String::new()
        }
    });
    
    // Wrap HTML content with complete HTML document including theme styles
    let html_preview = use_memo(move || {
        let html_content = convert_markdown_to_html(&markdown());
        
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        {}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
            theme_css(),
            html_content
        )
    });
    
    let available_themes = list_available_themes();
    
    rsx! {
        style { {include_str!("../../editor.css")} }
        div {
            class: "container",
            
            // Editor pane
            div {
                class: "editor-pane",
                div {
                    class: "editor-header",
                    h2 { "Markdown Editor" }
                    
                    // Theme selector
                    div {
                        class: "theme-selector",
                        label { "Theme: " }
                        select {
                            value: "{selected_theme}",
                            onchange: move |evt| selected_theme.set(evt.value().clone()),
                            for theme in available_themes.iter() {
                                option {
                                    value: "{theme}",
                                    selected: selected_theme() == *theme,
                                    "{theme}"
                                }
                            }
                        }
                    }
                }
                
                textarea {
                    class: "markdown-input",
                    value: "{markdown}",
                    oninput: move |evt| markdown.set(evt.value().clone()),
                    placeholder: "Write your markdown here..."
                }
            }
            
            // Preview pane
            div {
                class: "preview-pane",
                h2 { "Live Preview" }
                iframe {
                    class: "preview-frame",
                    srcdoc: "{html_preview}"
                }
            }
        }
    }
}
