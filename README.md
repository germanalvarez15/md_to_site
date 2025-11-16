# Markdown to Site

A lightweight command-line tool written in Rust that converts Markdown files into static HTML pages. Perfect for quickly generating simple websites from Markdown documentation.

> **Note**: This is a learning project created to explore Rust's ecosystem, including CLI argument parsing, file I/O, Markdown processing, and template rendering. It serves as a practical example of building a functional tool while learning Rust fundamentals.

## Installation

### Prerequisites

- Rust 1.70+ (edition 2024)
- Cargo

### Build from Source

```bash
git clone <repository-url>
cd md_to_site
cargo build --release
```

The compiled binary will be available at `target/release/md_to_site`.

## Usage

### Basic Usage

Convert a Markdown file to HTML using the default template:

```bash
cargo run -- --markdown example.md --output ./public
```

Or using the compiled binary:

```bash
./md_to_site --markdown example.md --output ./public
```

### Command-Line Arguments

- `-m, --markdown <Path>`: Path to the input Markdown file (required)
- `-o, --output <Path>`: Directory where the HTML file will be generated (required)
- `-t, --template <RUTA>`: Optional path to a custom Tera template directory

### Examples

**Using default template:**
```bash
cargo run -- --markdown /path/to/document.md --output /path/to/output
```

**Using custom template:**
```bash
cargo run -- --markdown document.md --output ./public --template ./templates/
```

## How It Works

The tool follows a simple three-step process:

1. **Read**: Reads the Markdown file from the specified path
2. **Convert**: Parses Markdown to HTML using `pulldown-cmark`
3. **Generate**: Wraps the HTML content in a template and writes to the output directory

### Project Structure

```
md_to_site/
├── Cargo.toml              # Project dependencies and metadata
├── README.md               # This file
├── example.md              # Example Markdown file
├── public/                 # Output directory for generated HTML
│   └── default.html        # Generated HTML file
└── src/
    ├── main.rs             # Entry point and CLI configuration
    └── mods/
        ├── mod.rs          # Module declarations
        ├── file_manager.rs # File I/O and Markdown conversion
        └── html_generator.rs # Template rendering
```

### Modules

#### `file_manager`
- `read_markdown_file()`: Reads Markdown content from a file
- `convert_markdown_to_html()`: Converts Markdown text to HTML

#### `html_generator`
- `generate_html()`: Renders HTML using Tera templates with title and content

### Default Template

The tool includes a built-in HTML5 template with:
- Responsive viewport meta tag
- UTF-8 character encoding
- Dynamic title injection
- Content placeholder for rendered Markdown

## Dependencies

- **clap** (4.0): Command-line argument parsing
- **pulldown-cmark** (0.9): CommonMark-compliant Markdown parser
- **tera** (1.19): Template engine for HTML generation
- **anyhow** (1.0): Error handling
- **log** (0.4) & **env_logger** (0.10): Logging capabilities

## Output

The generated HTML file (`default.html`) includes:
- Proper HTML5 structure
- Your Markdown content converted to semantic HTML
- Support for code blocks with syntax highlighting classes
- Properly formatted headings, paragraphs, lists, and more

## Use Cases

- 📚 Generate documentation sites from Markdown files
- 📝 Convert README files to HTML pages
- 🌐 Create simple static websites
- 📖 Build personal blogs or knowledge bases
- 🎓 Generate course materials or tutorials

## Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest new features
- Submit pull requests
- Improve documentation

## Future Enhancements

Potential features for future versions:
- Multiple file processing (batch conversion)
- CSS styling options
- Syntax highlighting integration
- Live preview server
- Watch mode for automatic regeneration
- Custom metadata extraction from frontmatter
- Table of contents generation

---

Built with ❤️ using Rust
