use std::io;
mod mods;
use clap::{Parser, Arg};
use std::path::{Path, PathBuf};
use tera::{Tera, Context};
use mods::file_manager::{read_markdown_file, convert_markdown_to_html};
use mods::html_generator::{generate_html};

#[derive(Parser,Debug)]
pub struct Config{

    #[arg(short, long, value_name = "Path")]
    pub markdown: PathBuf,

    #[arg(short, long, value_name = "Path")]
    pub output: PathBuf,

    #[arg(short, long, value_name = "RUTA")]
    pub template: Option<PathBuf>,
}
fn main() {
    let config = Config::parse();

    println!("Configuración cargada:");
    println!("  Markdown de entrada: {}", config.markdown.display());
    println!("  Directorio de salida: {}", config.output.display());

    match read_markdown_file(config.markdown.as_path()) {
        Ok(content) => {
            let html_content = convert_markdown_to_html(&content);
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
