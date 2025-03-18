# Termio

A Rust library for styling terminal output with CSS-like syntax. Termio allows you to create beautiful and consistent terminal output using familiar CSS-like syntax.

## Features

- CSS-like syntax for styling terminal text
- Support for colors (basic, intense, RGB, and color codes)
- Text decorations (bold, italic, underline, etc.)
- Padding and margins with support for multiple values
- Border styles and colors
- Easy-to-use API with method chaining
- Zero dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
termio = "0.1.0"
```

## Quick Start

```rust
fn main() {
    let mut termio = Termio::new();
    
    // Parse CSS-like syntax from a string
    let css = r#"
        @element "header" {
            color: blue;
            background: black;
            decoration: bold;
            padding: 1;
        }
    "#;

    termio.parse(css).unwrap();

    // Style text using the parsed styles
    println!("{}", "This is a header".style("header", &termio));
}
```

## Macro Support

Termio provides convenient macro TCSS (Terminal CSS):

### Using the `tcss!` macro

```rust
let parser = tcss! {
    "header" => {
        fg: Color::Cyan,
        bg: Color::Black,
        decoration: vec![Decoration::Bold],
        padding: 2,
        border_color: Color::Cyan,
        border_style: BorderStyle::Rounded
    },
    "warning" => {
        fg: Color::Yellow,
        bg: Color::Red,
        decoration: vec![Decoration::Bold, Decoration::Italic],
        border_style: BorderStyle::Dashed,
        border_color: Color::Yellow,
        padding: 1
    }
};

println!("{}", "Welcome!".style("header", &parser));
println!("{}", "Warning!".style("warning", &parser));
```

### Using the `style!` macro

For single style definitions:

```rust
let style = style! {
    fg: Color::Green,
    bg: Color::Black,
    decoration: vec![Decoration::Bold],
    padding: 1,
    margin: 1,
    border_color: Color::Yellow,
    border_style: BorderStyle::Rounded
};
```
## Examples

### Colors
```rust
@element "colored" {
    color: i-red;        // Intense red
    background: black;   // Black background
    color: rgb(255, 0, 0);  // RGB red
    color: 196;         // Color code 196 (red)
}
```

### Decorations
```rust
@element "decorated" {
    decoration: bold italic underline;
}
```

### Padding and Margins
```rust
@element "spaced" {
    padding: 1;
    margin: 1;
    padding-top: 2;
    margin-bottom: 2;
}
```

### Borders
```rust
@element "bordered" {
    border: rounded yellow;
    border: dashed i-red;
    border: solid green;
}
```

## Available Examples

The crate includes several examples demonstrating different features:

- `basic` - Basic styling examples
- `advanced` - Advanced UI components like cards and buttons
- `colors` - Different color formats and combinations
- `decorations` - Text decoration options
- `fluent` - Fluent interface for direct styling without TCSS
- `borders` - Different border styles and customizations
- `emoji` - Using emoji with styled text
- ...and other


Run examples with:
```bash
cargo run --example basic
```

## Emoji Support

Termio fully supports emoji characters in styled text. You can use emoji within any styled element:

```rust
use terminal_css::prelude::*;

// Basic emoji with color
println!("{}", "🚀 Rocket launch!".color(Color::Blue));

// Emoji with borders and padding
println!("{}", "🎉 Celebration 🎊"
    .padding(1)
    .border(BorderStyle::Rounded)
    .border_color(Color::Green));

// Emoji list with styling
let items = vec![
    "🍎 Apple",
    "🍌 Banana", 
    "🍇 Grapes"
].join("\n");

println!("{}", items
    .color(Color::Magenta)
    .padding(1)
    .border(BorderStyle::Dashed));
```

Run the emoji example with:
```bash
cargo run --example emoji
```

## Color Support

Termio supports multiple color formats:
- Basic colors: `red`, `green`, `blue`, etc.
- Intense colors: `i-red`, `i-green`, `i-blue`, etc.
- RGB colors: `rgb(255, 0, 0)`
- Color codes: `196` (for 256-color terminals)

## Text Decorations

Available text decorations:
- `bold`
- `italic`
- `underline`
- `overline`
- `blink`
- `reverse`
- `hidden`
- And more...

## Padding and Margin Support

The library supports multiple value formats for padding and margin:

1. Single value (applies to all sides):
```tcss
padding: 1;
margin: 1;
```

2. Two values (first for vertical sides, second for horizontal):
```tcss
padding: 1 2;
margin: 1 2;
```

3. Four values (in order: top, right, bottom, left):
```tcss
padding: 1 2 3 4;
margin: 1 2 3 4;
```

You can also set individual sides:
```tcss
padding-top: 1;
padding-bottom: 2;
padding-left: 3;
padding-right: 4;
margin-top: 1;
margin-bottom: 2;
margin-left: 3;
margin-right: 4;
```

## Styling Methods

Termio provides two ways to style text:

### 1. Using predefined styles

```rust
// Define styles using CSS-like syntax
let mut tcss = Termio::new();
tcss.parse(r#"
    @element "warning" {
        color: yellow;
        background: black;
        decoration: bold;
        padding: 1;
        border: solid red;
    }
"#).unwrap();

// Apply the style to text
let styled_text = "Warning message".style("warning", &tcss);
println!("{}", styled_text);
```

### 2. Using the fluent interface

```rust
// Style text directly using method chaining
let styled_text = "Warning message"
    .color(Color::Yellow)
    .bg(Color::Black)
    .decoration(Decoration::Bold)
    .padding(1)
    .border(BorderStyle::Solid)
    .border_color(Color::Red);

println!("{}", styled_text);
```

Available styling methods:

- `color(Color)` - Set the text color
- `bg(Color)` - Set the background color
- `decoration(Decoration)` - Add a text decoration
- `padding(u8)` - Set padding on all sides
- `padding_trbl(u8, u8, u8, u8)` - Set padding for top, right, bottom, left
- `margin(u8)` - Set margin on all sides
- `border(BorderStyle)` - Set the border style
- `border_color(Color)` - Set the border color

## License

This project is licensed under the MIT License.