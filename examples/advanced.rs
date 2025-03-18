use termio::*;

fn main() {
    let mut tcss = Termio::new();
    
    let tcss_content = r#"
        @element "card" {
            color: white;
            background: i-black;
            decoration: bold;
            border: rounded i-blue;
            padding: 1;
        }

        @element "title" {
            color: i-blue;
            decoration: bold underline;
        }

        @element "button" {
            color: black;
            background: i-green;
            decoration: bold;
            border: rounded i-green;
        }

        @element "error" {
            color: white;
            background: i-red;
            decoration: bold;
            border: solid i-red;
        }
    "#;

    tcss.parse(tcss_content).unwrap();

    // Create a card with a header and buttons
    println!("{}", "User Profile".style("title", &tcss));
    println!("{}", "Name: John Doe\nEmail: john@example.com".style("card", &tcss));
    println!("{}", "Edit Profile".style("button", &tcss));
    println!("{}", "Delete Account".style("button", &tcss));
    println!("{}", "Failed to connect to server".style("error", &tcss));
} 