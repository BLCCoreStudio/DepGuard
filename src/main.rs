use std::{env, process};

fn help() {
    println!("DepGuard 0.1.0-dev\n\nUSAGE:\n  depguard status\n\nRegistry-backed verification is not implemented yet. This command scaffold intentionally does not claim a package is safe or unsafe.");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        help();
        return;
    }
    if args[0] == "--version" || args[0] == "-V" {
        println!("depguard 0.1.0-dev");
        return;
    }
    if args.len() == 1 && args[0] == "status" {
        println!("DepGuard is in early development; registry verification is not available yet.");
        return;
    }
    eprintln!("depguard: unsupported command in the current development scaffold");
    process::exit(2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn development_scaffold_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "depguard");
    }
}
