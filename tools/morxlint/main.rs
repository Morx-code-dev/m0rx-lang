// M0RX Linter - morxlint
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
            println!("morxlint v0.1.0");
        }
        "help" | "--help" => {
            print_help();
        }
        _ => {
            let path = &args[1];
            lint_file(path);
        }
    }
}

fn lint_file(path: &str) {
    if !std::path::Path::new(path).exists() {
        eprintln!("morxlint Error: file '{}' not found", path);
        process::exit(1);
    }
    let source = fs::read_to_string(path).unwrap_or_default();
    let warnings = lint_source(&source);
    if warnings.is_empty() {
        println!("morxlint: '{}' — No issues found ✓", path);
    } else {
        for w in &warnings {
            println!("morxlint: {}", w);
        }
        println!("morxlint: {} issue(s) found", warnings.len());
    }
}

fn lint_source(source: &str) -> Vec<String> {
    let mut warnings = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let ln = i + 1;
        let trimmed = line.trim();
        // Line too long
        if line.len() > 100 {
            warnings.push(format!(
                "Line {}: line too long ({} chars)", ln, line.len()
            ));
        }
        // Trailing whitespace
        if line != line.trim_end() {
            warnings.push(format!(
                "Line {}: trailing whitespace", ln
            ));
        }
        // TODO comments
        if trimmed.contains("TODO") || trimmed.contains("FIXME") {
            warnings.push(format!(
                "Line {}: unresolved TODO/FIXME", ln
            ));
        }
        // Empty function body
        if trimmed == "fn" {
            warnings.push(format!(
                "Line {}: empty function name", ln
            ));
        }
    }
    warnings
}

fn print_help() {
    println!("morxlint - M0RX Linter v0.1.0");
    println!("Usage:");
    println!("  morxlint <file.mrx>   Lint a file");
    println!("  morxlint version      Show version");
}
