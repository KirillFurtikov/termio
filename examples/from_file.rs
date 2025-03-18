use std::fs::File;
use std::io::Read;
use std::path::Path;
use termio::prelude::*;

fn main() {
    // Create a new Termio instance
    let tcss = match Termio::from_file("examples/styles.tcss") {
        Ok(tcss) => tcss,
        Err(e) => {
            eprintln!("Error loading styles: {}", e);
            return;
        }
    };

    // Apply styles to text
    println!("Examples of using styles from file:");
    println!("{}", "Page Header".style("header", &tcss));
    println!("{}", "Important Message".style("warning", &tcss));
    println!("{}", "Information Message".style("info", &tcss));
    println!("{}", "Successful Action".style("success", &tcss));
    println!("{}", "Operation Error".style("error", &tcss));
} 