// M0RX Formatter - morxfmt
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        process::exit(0);
    }

    match args[1].as_str() {
        "version" | "--version" => {
            println!("morxfmt v0.1.0");
        }
        "help" | "--help" => {
            print_help();
        }
        _ => {
            let path = &args[1];
            format_file(path);
        }
    }
}

fn format_file(path: &str) {
    if !std::path::Path::new(path).exists() {
        eprintln!("morxfmt Error: file '{}' not found", path);
        process::exit(1);
    }
    let source = fs::read_to_string(path).unwrap_or_default();
    let formatted = format_source(&source);
    fs::write(path, &formatted).ok();
    println!("morxfmt: Formatted '{}'", path);
}

fn format_source(source: &str) -> String {
    let mut result = Vec::new();
    let mut indent = 0usize;

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed.ends_with('}') && indent > 0 {
            indent -= 1;
        }

        if trimmed.is_empty() {
            result.push(String::new());
        } else {
            result.push(format!("{}{}", "    ".repeat(indent), trimmed));
        }

        if trimmed.ends_with('{') {
            indent += 1;
        }
    }
    result.join("\n")
}

fn print_help() {
    println!("morxfmt - M0RX Code Formatter v0.1.0");
    println!("Usage:");
    println!("  morxfmt <file.mrx>   Format a file");
    println!("  morxfmt version      Show version");
}
