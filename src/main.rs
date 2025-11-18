use std::io;
mod mods;
use clap::{Parser, Arg};
use std::path::{Path, PathBuf};
use tera::{Tera, Context};
use mods::file_manager::{read_markdown_file, convert_markdown_to_html};
use mods::html_generator::{generate_html};
use mods::theme_manager::{get_theme, list_available_themes};

#[derive(Parser,Debug)]
pub struct Config{

    #[arg(short, long, value_name = "Path")]
    pub markdown: PathBuf,

    #[arg(short, long, value_name = "Path")]
    pub output: PathBuf,

    #[arg(short, long, value_name = "Path")]
    pub template: Option<PathBuf>,

    #[arg(long, value_name = "Theme", default_value = "default")]
    pub theme: String,
}
fn main() {
    let config = Config::parse();

    println!("Configuración cargada:");
    println!("  Markdown de entrada: {}", config.markdown.display());
    println!("  Directorio de salida: {}", config.output.display());
    println!("  Tema: {}", config.theme);

    match read_markdown_file(config.markdown.as_path()) {
        Ok(content) => {
            let html_content = convert_markdown_to_html(&content);
            
            // Use custom template or theme
            let template_path = if let Some(custom) = &config.template {
                Some(custom.clone())
            } else {
                match get_theme(&config.theme) {
                    Ok(theme) => {
                        // Copy CSS to output directory
                        let css_name = theme.css_path.file_name().unwrap();
                        let dest_css = config.output.join(css_name);
                        std::fs::create_dir_all(&config.output).ok();
                        std::fs::copy(&theme.css_path, &dest_css).ok();
                        Some(theme.template_path)
                    },
                    Err(e) => {
                        eprintln!("Error loading theme: {}", e);
                        println!("Available themes: {:?}", list_available_themes());
                        None
                    }
                }
            };
            
            let html_page = match &config.template {
                Some(path) => generate_html(html_content, String::from("Page Title"), &config.template).unwrap(),
                None => generate_html(html_content, String::from("Page Title"), &None).unwrap(),
            };
            
            // Crear el directorio de salida si no existe
            std::fs::create_dir_all(&config.output).expect("No se pudo crear el directorio de salida");
            
            // Escribir el archivo HTML
            let output_path = config.output.join("default.html");
            std::fs::write(&output_path, html_page).expect("No se pudo escribir el archivo HTML");
            
            println!("Archivo generado: {}", output_path.display());
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
