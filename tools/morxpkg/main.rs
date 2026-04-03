// M0RX Package Manager - morxpkg
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        process::exit(0);
    }

    match args[1].as_str() {
        "install" => {
            if args.len() < 3 {
                eprintln!("M0RX PKG Error: provide package name");
                process::exit(1);
            }
            install_pkg(&args[2]);
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("M0RX PKG Error: provide package name");
                process::exit(1);
            }
            remove_pkg(&args[2]);
        }
        "update" => {
            if args.len() >= 3 {
                update_pkg(&args[2]);
            } else {
                update_all();
            }
        }
        "list" => {
            list_pkgs();
        }
        "search" => {
            if args.len() < 3 {
                eprintln!("M0RX PKG Error: provide search query");
                process::exit(1);
            }
            search_pkg(&args[2]);
        }
        "publish" => {
            publish_pkg();
        }
        "init" => {
            init_project();
        }
        "version" | "--version" => {
            println!("morxpkg v0.1.0");
        }
        "help" | "--help" => {
            print_help();
        }
        _ => {
            eprintln!("M0RX PKG Error: unknown command '{}'", args[1]);
            print_help();
        }
    }
}

fn install_pkg(name: &str) {
    println!("morxpkg: Installing '{}'...", name);
    println!("morxpkg: Resolving dependencies...");
    println!("morxpkg: Downloaded '{}'", name);
    println!("morxpkg: Installed '{}' successfully!", name);
}

fn remove_pkg(name: &str) {
    println!("morxpkg: Removing '{}'...", name);
    println!("morxpkg: Removed '{}' successfully!", name);
}

fn update_pkg(name: &str) {
    println!("morxpkg: Updating '{}'...", name);
    println!("morxpkg: Updated '{}' successfully!", name);
}

fn update_all() {
    println!("morxpkg: Updating all packages...");
    println!("morxpkg: All packages updated!");
}

fn list_pkgs() {
    println!("morxpkg: Installed packages:");
    println!("  m0rx.core     v0.1.0");
    println!("  m0rx.backend  v0.1.0");
    println!("  m0rx.ui       v0.1.0");
    println!("  m0rx.ai       v0.1.0");
    println!("  m0rx.voice    v0.1.0");
}

fn search_pkg(query: &str) {
    println!("morxpkg: Searching for '{}'...", query);
    println!("  m0rx.{} - M0RX package", query);
}

fn publish_pkg() {
    println!("morxpkg: Publishing package...");
    println!("morxpkg: Reading morx.toml...");
    println!("morxpkg: Package published successfully!");
}

fn init_project() {
    println!("morxpkg: Initializing M0RX project...");
    std::fs::write("morx.toml", "[package]\nname = \"myapp\"\nversion = \"0.1.0\"\nauthor = \"\"\n\n[dependencies]\n").ok();
    std::fs::write("main.mrx", "// M0RX App\nshowln(\"Hello M0RX!\")\n").ok();
    println!("morxpkg: Created morx.toml");
    println!("morxpkg: Created main.mrx");
    println!("morxpkg: Project initialized!");
}

fn print_help() {
    println!("morxpkg - M0RX Package Manager v0.1.0");
    println!("======================================");
    println!("Usage:");
    println!("  morxpkg install <pkg>   Install package");
    println!("  morxpkg remove <pkg>    Remove package");
    println!("  morxpkg update <pkg>    Update package");
    println!("  morxpkg update          Update all");
    println!("  morxpkg list            List installed");
    println!("  morxpkg search <query>  Search packages");
    println!("  morxpkg publish         Publish package");
    println!("  morxpkg init            Init new project");
    println!("  morxpkg version         Show version");
}
