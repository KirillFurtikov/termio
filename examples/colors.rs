use termio::*;

fn main() {
    let mut tcss = Termio::new();
    
    let tcss_content = r#"
        @element "basic" {
            color: red;
            background: black;
        }

        @element "intense" {
            color: i-red;
            background: i-black;
        }

        @element "rgb" {
            color: rgb(255, 0, 0);
            background: rgb(0, 0, 0);
        }

        @element "code" {
            color: 196;
            background: 0;
            border: solid rgb(6, 180, 49);
        }
    "#;

    tcss.parse(tcss_content).unwrap();

    println!("Basic colors:");
    println!("{}", "Red text on black background".style("basic", &tcss));
    
    println!("\nIntense colors:");
    println!("{}", "Intense red text on intense black background".style("intense", &tcss));
    
    println!("\nRGB colors:");
    println!("{}", "RGB red text on RGB black background".style("rgb", &tcss));
    
    println!("\nColor codes:");
    println!("{}", "Color code 196 (red) on code 0 (black)".style("code", &tcss));
} 