# Markdown to Site

A comprehensive Rust-based toolset for working with Markdown files - includes both a CLI converter and a live desktop editor with real-time preview.

> **Note**: This is a learning project created to explore Rust's ecosystem, including CLI argument parsing, file I/O, Markdown processing, template rendering, and desktop GUI development with Dioxus. It serves as a practical example of building functional tools while learning Rust fundamentals.

## Features

✨ **Two Usage Modes:**
1. **CLI Tool** - Convert Markdown files to static HTML with pre-defined themes
2. **Live Editor** - Desktop app with split-pane editor and real-time preview

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

The compiled binaries will be available at:
- `target/release/md_to_site` (CLI tool)
- `target/release/editor` (Live editor)

## Usage

### Mode 1: CLI Converter

Convert a Markdown file to HTML using the command-line tool:

```bash
cargo run -- --markdown example.md --output ./public
```

Or using the compiled binary:

```bash
./md_to_site --markdown example.md --output ./public
#### Command-Line Arguments

- `-m, --markdown <Path>`: Path to the input Markdown file (required)
- `-o, --output <Path>`: Directory where the HTML file will be generated (required)
- `-t, --template <Path>`: Optional path to a custom Tera template file
- `--theme <Theme>`: Choose a pre-defined theme (default: "default")

#### CLI ExamplesTheme>`: Choose a pre-defined theme (default: "default")

### Examples

**Using default theme:**
```bash
cargo run -- --markdown /path/to/document.md --output /path/to/output
```

**Using a specific theme:**
```bash
cargo run -- --markdown document.md --output ./public --theme dark
cargo run -- --markdown document.md --output ./public --theme minimal
cargo run -- --markdown document.md --output ./public --theme blog
```

**Using custom template (overrides theme):**
```bash
cargo run -- --markdown document.md --output ./public --template ./my-template.html
```

### CLI Tool Process

1. **Read**: Reads the Markdown file from the specified path
2. **Convert**: Parses Markdown to HTML using `pulldown-cmark`
3. **Generate**: Wraps the HTML content in a template and writes to the output directory

### Live Editor Process

1. **Input**: User types Markdown in the left pane
2. **Reactive Conversion**: Text changes trigger instant HTML conversion
3. **Theme Application**: Selected theme CSS is applied to the preview
4. **Isolated Rendering**: Preview renders in an iframe with complete HTML document

Or using the compiled binary:

```bash
./editor
```

#### Live Editor Features

- **Split-Pane Interface**: Editor on the left, live preview on the right
- **Real-Time Preview**: See HTML output instantly as you type
- **Theme Switching**: Change themes on-the-fly with dropdown selector
- **Isolated Preview**: Themes only affect the preview, not the editor interface
- **Dark Editor Theme**: Comfortable dark-themed editor with syntax-aware styling
- **Desktop App**: Native window using Dioxus framework

The editor window provides:
- Markdown editor with dark theme (left pane)
- Live HTML preview with selected theme (right pane)
- Theme selector dropdown to switch between available themes
- Full window split-pane layout

## How It Works

### CLI Tool Process

The tool follows a simple three-step process:

1. **Read**: Reads the Markdown file from the specified path
```
md_to_site/
├── Cargo.toml              # Project dependencies and metadata
├── README.md               # This file
├── example.md              # Example Markdown file
├── editor.css              # Live editor styling
├── public/                 # Output directory for generated HTML
│   ├── default.html        # Generated HTML file
│   └── style.css           # Theme CSS file (copied automatically)
├── templates/              # Pre-defined themes
│   ├── default/
│   │   ├── template.html
│   │   └── style.css
│   ├── minimal/
│   │   ├── template.html
│   │   └── style.css
│   ├── dark/
│   │   ├── template.html
│   │   └── style.css
│   └── blog/
│       ├── template.html
│       └── style.css
└── src/
    ├── lib.rs              # Library entry point
    ├── main.rs             # CLI tool entry point
    ├── bin/
    │   └── editor.rs       # Live editor application
    └── mods/
        ├── mod.rs          # Module declarations
        ├── file_manager.rs # File I/O and Markdown conversion
        ├── html_generator.rs # Template rendering
        └── theme_manager.rs  # Theme loading and management
``` └── mods/
        ├── mod.rs          # Module declarations
        ├── file_manager.rs # File I/O and Markdown conversion
        ├── html_generator.rs # Template rendering
        └── theme_manager.rs  # Theme loading and management
```

### Modules

#### `file_manager`
- `read_markdown_file()`: Reads Markdown content from a file
- `convert_markdown_to_html()`: Converts Markdown text to HTML

#### `html_generator`
- `generate_html()`: Renders HTML using Tera templates with title and content

#### `theme_manager`
- `get_theme()`: Loads a theme by name and returns template and CSS paths
- `list_available_themes()`: Lists all available themes in the templates directory

## Available Themes

The tool comes with 4 pre-defined themes:

### 1. **default** (default)
Clean, professional design with a white content card on a light gray background. Includes syntax highlighting for code blocks.

### 2. **minimal**
Simplistic design with serif typography and minimal styling. Perfect for distraction-free reading.

### 3. **dark**
Modern dark theme with gradient accents, neon colors, and a sophisticated dark background. Great for technical documentation.

### 4. **blog**
Full blog-style layout with header, navigation menu, and footer. Includes gradient header styling and professional typography.

Each theme includes:
- Responsive design with mobile support
- Proper typography and spacing
- Syntax-highlighted code blocks
## Dependencies

### Core Dependencies
- **clap** (4.5): Command-line argument parsing
- **pulldown-cmark** (0.11): CommonMark-compliant Markdown parser
- **tera** (1.20): Template engine for HTML generation
- **anyhow** (1.0): Error handling
- **log** (0.4) & **env_logger** (0.10): Logging capabilities

### Live Editor Dependencies
- **dioxus** (0.5): Desktop GUI framework with reactive state management
- **dioxus-desktop** (0.5): Desktop window and WebView support

## CLI Output

The CLI tool generates:
1. **HTML file** (`default.html`): Contains your converted Markdown with proper HTML5 structure
2. **CSS file** (`style.css`): Automatically copied from the selected theme
Command-line converter for batch processing and scripting

### 2. `editor` (Live Editor)
Desktop application with real-time preview and theme switchingnt
- A `<link>` tag to reference the CSS file

## Dependencies

## Use Cases

- 📚 Generate documentation sites from Markdown files
- 📝 Convert README files to HTML pages
- 🌐 Create simple static websites
- 📖 Build personal blogs or knowledge bases
- 🎓 Generate course materials or tutorials
- ✍️ Write and preview Markdown content in real-time
- 🎨 Test different themes before exporting

The tool generates:
1. **HTML file** (`default.html`): Contains your converted Markdown with proper HTML5 structure
2. **CSS file** (`style.css`): Automatically copied from the selected theme

Features of generated HTML:
- Proper HTML5 structure
- Semantic HTML from Markdown conversion
- Linked stylesheet for styling
- Support for code blocks with syntax highlighting classes
- Properly formatted headings, paragraphs, lists, blockquotes, and more

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
- Additional pre-defined themes
- Syntax highlighting integration (e.g., highlight.js)
- Live preview server
- Watch mode for automatic regeneration
- Custom metadata extraction from HTML comments in Markdown
- Table of contents generation
- Custom CSS file support via command-line argument
- Theme customization options

---

Built with ❤️ using Rust
