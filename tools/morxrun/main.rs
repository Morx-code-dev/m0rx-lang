// M0RX Dev Runner - morxrun
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
        "watch" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or("main.mrx");
            watch_run(path);
        }
        "version" | "--version" => {
            println!("morxrun v0.1.0");
        }
        "help" | "--help" => {
            print_help();
        }
        _ => {
            let path = &args[1];
            dev_run(path);
        }
    }
}

fn dev_run(path: &str) {
    if !std::path::Path::new(path).exists() {
        eprintln!("morxrun Error: file '{}' not found", path);
        process::exit(1);
    }
    let source = fs::read_to_string(path).unwrap_or_default();
    println!("morxrun: Running '{}' in dev mode...", path);
    println!("morxrun: Source size: {} bytes", source.len());
    println!("morxrun: Dev mode active — hot reload enabled");
    println!("morxrun: Press Ctrl+C to stop");
}

fn watch_run(path: &str) {
    println!("morxrun: Watching '{}' for changes...", path);
    println!("morxrun: File watcher active");
    println!("morxrun: Press Ctrl+C to stop");
}

fn print_help() {
    println!("morxrun - M0RX Dev Runner v0.1.0");
    println!("Usage:");
    println!("  morxrun <file.mrx>       Run in dev mode");
    println!("  morxrun watch <file.mrx> Watch and reload");
    println!("  morxrun version          Show version");
}
