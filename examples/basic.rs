use termio::*;

fn main() {
    let mut tcss = Termio::new();
    
    let tcss_content = r#"
        @element "header" {
            color: blue;
            background: i-blue;
            decoration: bold;
        }

        @element "warning" {
            color: yellow;
            background: i-yellow;
            decoration: bold italic;
        }

        @element "info" {
            color: cyan;
            background: i-cyan;
            decoration: underline;
        }
    "#;

    tcss.parse(tcss_content).unwrap();

    println!("{}", "Welcome to Terminal CSS!".style("header", &tcss));
    println!("{}", "This is a warning message".style("warning", &tcss));
    println!("{}", "And this is an info message".style("info", &tcss));
} 