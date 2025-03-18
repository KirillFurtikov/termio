use termio::prelude::*;

fn main() {
    println!("Styling text with emoji using Terminal CSS:\n");
    
    // Basic emoji styling
    println!("{}", "🚀 Rocket launch!".color(Color::Blue));
    
    // Emoji with background
    println!("{}", "🎉 Celebration 🎊".color(Color::Yellow).bg(Color::Black));
    
    // Emoji with decorations
    println!("{}", "⚠️ Warning!".color(Color::Red).decoration(Decoration::Bold));
    
    // Emoji in borders
    println!("{}", "🍔 Delicious burger 🍟"
        .padding(1)
        .border(BorderStyle::Rounded)
        .border_color(Color::Green));
    
    // Emoji as bullet points in a list
    let items = vec![
        "🍎 Apple",
        "🍌 Banana",
        "🍇 Grapes",
        "🍓 Strawberry",
        "🥭 Mango",
    ];
    
    let list = items.join("\n");
    println!("{}", list
        .color(Color::Magenta)
        .padding(1)
        .border(BorderStyle::Dashed)
        .border_color(Color::Cyan));
    
    // Emoji status indicators
    println!("{}", "✅ Task completed"
        .color(Color::Green)
        .decoration(Decoration::Bold));
    
    println!("{}", "❌ Task failed"
        .color(Color::Red)
        .decoration(Decoration::Bold));
    
    println!("{}", "⏳ Task in progress"
        .color(Color::Yellow)
        .decoration(Decoration::Italic));
    
    // Complex emoji dashboard example
      println!("\n{}", "📊 System Dashboard 📈"
        .color(Color::Blue)
        .decoration(Decoration::Bold)
        .padding(1)
        .border(BorderStyle::Double)
        .border_color(Color::IntenseBlue));
    
    let dashboard = vec![
        "🔋 Battery: 85%",
        "💾 Disk: 120GB free",
        "🖥️ CPU: 12% usage",
        "🧠 Memory: 4GB available",
        "🌡️ Temperature: 45°C",
    ];
    
    let dashboard_text = dashboard.join("\n");
    println!("{}", dashboard_text
        .color(Color::White)
        .bg(Color::Black)
        .padding(1)
        .border(BorderStyle::Solid)
        .border_color(Color::Cyan));
} 