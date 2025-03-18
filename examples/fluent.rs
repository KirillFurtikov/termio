use termio::prelude::*;

fn main() {
    println!("Direct styling with fluent interface:\n");
    
    // Basic color styling
    println!("{}", "Red text".color(Color::Red));
    
    // Background color
    println!("{}", "Blue text on yellow background".color(Color::Blue).bg(Color::Yellow));
    
    // Decorations
    println!("{}", "Bold and italic text".decoration(Decoration::Bold).decoration(Decoration::Italic));
    
    // Padding and border
    println!("{}", "Text with padding and border"
        .padding(1)
        .border(BorderStyle::Solid)
        .border_color(Color::Green));
    
    // Complex chained styling
    println!("{}", "Complex styling example"
        .color(Color::Code(48))
        .bg(Color::IntenseGreen)
        .decoration(Decoration::Reverse)
        .decoration(Decoration::DoubleUnderline)
        .padding_trbl(1, 2, 1, 2)
        .margin(1)
        .border(BorderStyle::Double)
        .border_color(Color::Yellow));
    
    // Different border styles
    println!("{}", "Solid border".border(BorderStyle::Solid).padding(1));
    println!("{}", "Dashed border".border(BorderStyle::Dashed).padding(1));
    println!("{}", "Rounded border".border(BorderStyle::Rounded).padding(1));
    
    // Multiple decorations
    let decorated = "Multiple decorations: bold, italic, underline"
        .decoration(Decoration::Bold)
        .decoration(Decoration::Italic)
        .decoration(Decoration::Underline);
    println!("{}", decorated);
} 