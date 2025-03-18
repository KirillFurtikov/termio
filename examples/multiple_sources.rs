use std::fs::File;
use std::io::Read;
use std::path::Path;
use termio::{prelude::*, style};

fn main() {
    // Create a new Termio instance
    let mut tcss = Termio::new();

    // Load basic styles from string
    let base_styles = r#"
        @element "base" {
            color: white;
            background: black;
            decoration: bold;
            padding: 1;
        }

        @element "subtle" {
            color: i-black;
            background: black;
            decoration: italic;
        }
    "#;

    // Parse basic styles
    match tcss.parse(base_styles) {
        Ok(_) => println!("Basic styles successfully loaded from string!"),
        Err(e) => {
            eprintln!("Error parsing basic styles: {}", e);
            return;
        }
    }

    // Load additional styles from file
    let additional_styles_path = Path::new("examples/styles.tcss");
    let mut additional_styles = String::new();

    // Read the file
    let mut file = match File::open(&additional_styles_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open style file: {}", e);
            return;
        }
    };

    if let Err(e) = file.read_to_string(&mut additional_styles) {
        eprintln!("Failed to read style file: {}", e);
        return;
    }

    // Parse additional styles
    match tcss.parse(&additional_styles) {
        Ok(_) => println!("Additional styles successfully loaded from file!"),
        Err(e) => {
            eprintln!("Error parsing additional styles: {}", e);
            return;
        }
    }

    // Add another custom style via API
    let mut custom_style = Style::new();
    custom_style.fg = Some(Color::Magenta);
    custom_style.decoration = Some(vec![Decoration::Bold, Decoration::Underline]);
    custom_style.padding = Some(1);
    custom_style.padding_left = Some(6);
    custom_style.padding_right = Some(6);
    custom_style.border_style = Some(BorderStyle::Rounded);

    // Add style to the parser
    tcss.add_style("custom", custom_style);
    println!("Custom style successfully added via API!");

    let style_from_macros = style! {
        fg: Color::IntenseBlack,
        bg: Color::Black,
        decoration: vec![Decoration::Strikethrough],  
        padding: 1,
        border_style: BorderStyle::Dashed,
    };
    
    tcss.add_style("macros", style_from_macros);
    println!("Macros style successfully added via macros!");

    // Display all styles
    println!("Examples of using styles from different sources:");
    println!("{}", "Base style from string".style("base", &tcss));
    println!("{}", "Subtle style from string".style("subtle", &tcss));
    println!("{}", "Header from file".style("header", &tcss));
    println!("{}", "Warning from file".style("warning", &tcss));
    println!("{}", "Custom style via API".style("custom", &tcss));
    println!("{}", "Macros style".style("macros", &tcss));
} 