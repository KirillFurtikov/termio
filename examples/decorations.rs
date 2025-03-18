use termio::*;

fn main() {
    let mut parser = Termio::new();
    
    let tcss = r#"
        @element "bold" {
            decoration: bold;
            color: green;
        }
        @element "italic" {
            decoration: italic;
            color: blue;
        }
        @element "underline" {
            decoration: underline;
            color: cyan;
        }
        @element "multiple" {
            decoration: bold italic underline;
            color: magenta;
        }
        @element "fancy" {
            decoration: bold italic underline overline;
            color: yellow;
        }
        @element "reverse" {
            decoration: reverse;
            color: red;
        }
        @element "conceal" {
            decoration: conceal;
            color: white;
        }
        @element "strikethrough" {
            decoration: strikethrough;
            color: i-red;
        }
        @element "framed" {
            decoration: framed;
            color: i-green;
        }
        @element "encircled" {
            decoration: encircled;
            color: i-blue;
        }
        @element "all" {
            decoration: bold italic underline overline blink rapid-blink reverse strikethrough framed encircled;
            color: i-magenta;
            background: black;
        }
    "#;

    parser.parse(tcss).unwrap();

    println!("Basic decorations:");
    println!("{}", "Bold Text".style("bold", &parser));
    println!("{}", "Italic Text".style("italic", &parser));
    println!("{}", "Underlined Text".style("underline", &parser));
    
    println!("\nMultiple decorations:");
    println!("{}", "Multiple Decorations".style("multiple", &parser));
    println!("{}", "Fancy Text".style("fancy", &parser));
    
    println!("\nSpecial decorations:");
    println!("{}", "Reversed Text".style("reverse", &parser));
    println!("{}", "Concealed Text".style("conceal", &parser));
    println!("{}", "Strikethrough Text".style("strikethrough", &parser));
    println!("{}", "Framed Text".style("framed", &parser));
    println!("{}", "Encircled Text".style("encircled", &parser));
    println!("\nAll decorations combined:");
    println!("{}", "All Decorations".style("all", &parser));
} 