use termio::StyledText;
use termio::Termio;

fn main() {
    let mut parser = Termio::new();
    
    let tcss = r#"
        @element "solid-red" {
            border-style: solid;
            border-color: red;
            padding: 1;
        }

        @element "dashed-green" {
            border-style: dashed;
            border-color: green;
            padding: 1;
        }

        @element "rounded-blue" {
            border-style: rounded;
            border-color: blue;
            padding: 1;
        }

        @element "fancy-yellow" {
            border-style: solid;
            border-color: yellow;
            color: yellow;
            background: black;
            decoration: bold;
            padding: 1 2;
        }

        @element "rainbow" {
            border-style: rounded;
            border-color: i-magenta;
            color: cyan;
            background: black;
            decoration: bold italic;
            padding: 2;
            margin: 1;
        }

        @element "outer" {
            border-style: dashed;
            border-color: red;
            color: white;
            background: black;
            padding: 2;
            margin: 1;
        }

        @element "inner" {
            border-style: solid;
            border-color: cyan;
            color: i-green;
            background: rgb(44, 202, 0);
            padding: 1;
            margin: 0;
        }
    "#;

    parser.parse(tcss).unwrap();

    // Basic border styles
    println!("{}", "Simple solid red border".style("solid-red", &parser));
    println!("{}", "Dashed green border".style("dashed-green", &parser));
    println!("{}", "Rounded blue border".style("rounded-blue", &parser));

    println!("\nStyled content with borders:");
    println!("{}", "Fancy Yellow Box".style("fancy-yellow", &parser));
    println!("{}", "Rainbow Box with Margins".style("rainbow", &parser));

    println!("\nNested borders:");
    let outer_start = "Outer box containing:".style("outer", &parser);
    let inner_box = "This is an inner box\nwith multiple lines\nof styled content".style("inner", &parser);
    println!("{}\n{}", outer_start, inner_box);
} 