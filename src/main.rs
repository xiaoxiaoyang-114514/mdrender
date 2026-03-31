use std::env;
use std::fs;
use std::io;

use pulldown_cmark::{html, Parser};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")){
        println!("mdrender - Convert Markdown file to HTML");
        println!("\nUsage: mdrender [OPTIONS] [COMMAND]");
        println!("\nOptions:\n   -h, --help   Print help information");
        println!("   -o   Output the HTML src into a file\n    example: mdrender -o a.html a.md");
        return Ok(());
    }
    if args.len() < 2 {
        eprintln!("Error: Missing Markdown file path");
        eprintln!("\nUsage: mdrender [OPTIONS] [COMMAND]");
        std::process::exit(-1);
    }
    if args.contains(&String::from("-o")){
        let file_path = &args[3];
        let output = &args[2];
        let md_content = fs::read_to_string(file_path)?;
        let parser = Parser::new(&md_content);
        let mut html = String::new();
        html::push_html(&mut html, parser);
        fs::write(&output, html);
        return Ok(());
    }
    let file_path = &args[1];
    let md_content = fs::read_to_string(file_path)?;
    let parser = Parser::new(&md_content);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    println!("{html}");
    return Ok(());
}
