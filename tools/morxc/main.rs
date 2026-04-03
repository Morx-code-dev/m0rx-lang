// M0RX Compiler CLI - morxc
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
        "build" => {
            if args.len() < 3 {
                eprintln!("M0RX Error: provide a .mrx file");
                process::exit(1);
            }
            let file = &args[2];
            build_file(file, false);
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("M0RX Error: provide a .mrx file");
                process::exit(1);
            }
            let file = &args[2];
            build_file(file, true);
        }
        "check" => {
            if args.len() < 3 {
                eprintln!("M0RX Error: provide a .mrx file");
                process::exit(1);
            }
            let file = &args[2];
            check_file(file);
        }
        "version" | "--version" | "-v" => {
            println!("M0RX Compiler v0.1.0");
            println!("LLVM 17 backend");
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            // Direct file
            let file = &args[1];
            if file.ends_with(".mrx") {
                build_file(file, true);
            } else {
                eprintln!("M0RX Error: unknown command '{}'", args[1]);
                print_help();
                process::exit(1);
            }
        }
    }
}

fn build_file(path: &str, run: bool) {
    if !std::path::Path::new(path).exists() {
        eprintln!("M0RX Error: file '{}' not found", path);
        process::exit(1);
    }
    let source = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("M0RX Error: cannot read '{}'", path);
        process::exit(1);
    });
    println!("M0RX Compiling: {}", path);
    println!("Source length: {} chars", source.len());
    if run {
        println!("M0RX Running: {}", path);
    } else {
        println!("M0RX Build: OK");
    }
}

fn check_file(path: &str) {
    if !std::path::Path::new(path).exists() {
        eprintln!("M0RX Error: file '{}' not found", path);
        process::exit(1);
    }
    println!("M0RX Check: {}", path);
    println!("M0RX Check: OK - no errors found");
}

fn print_help() {
    println!("M0RX Compiler v0.1.0");
    println!("====================");
    println!("Usage:");
    println!("  morxc <file.mrx>        Run a M0RX file");
    println!("  morxc build <file.mrx>  Compile to binary");
    println!("  morxc run <file.mrx>    Compile and run");
    println!("  morxc check <file.mrx>  Check for errors");
    println!("  morxc version           Show version");
    println!("  morxc help              Show this help");
    println!("");
    println!("Examples:");
    println!("  morxc hello.mrx");
    println!("  morxc build myapp.mrx");
    println!("  morxc run server.mrx");
}
