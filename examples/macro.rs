use termio::{tcss, BorderStyle, Color, Decoration, StyledText};

fn main() {
    let parser = tcss! {
        "footer" => {
            fg: Color::Rgb(0, 255, 0),
            bg: Color::IntenseYellow,
            decoration: vec![Decoration::Bold, Decoration::Underline],
            border_color: Color::IntenseBlue,
            border_style: BorderStyle::Dashed,
            padding: 1,
            margin: 1
        },
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
        },
        "info" => {
            fg: Color::White,
            bg: Color::Blue,
            decoration: vec![Decoration::Bold],
            border_style: BorderStyle::Solid,
            border_color: Color::Cyan,
            padding: 1,
            margin: 1
        }
    };

    println!("Styled elements using tcss! macro:\n");
    println!("{}", "Welcome to Terminal CSS!".style("header", &parser));
    println!("{}", "Warning: This is important!".style("warning", &parser));
    println!("{}", "Info: Using macros is fun!".style("info", &parser));
    println!("{}", "Footer text".style("footer", &parser));
}
