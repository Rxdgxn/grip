/// Grip: grep but in Rust

/// USAGE: cargo run =pattern= ...(files or folders to search in) (+flags, when needed)
/// Example: 'cargo run =vec= . %.\src\lib.rs -n -i'. Here, '%' denotes a path that is excluded when searching

use grip::*;
use std::fs;

type Path = String;
type Paths = Vec<Path>;

fn parse_path(path: Path, md: fs::Metadata, excluded: &mut Paths) -> Paths {
    let mut files: Paths = Vec::new();

    if !excluded.contains(&path) {
        if md.is_file() {
            files.push(path.to_string());
        }
        else if md.is_dir() {
            let content = fs::read_dir(path).unwrap();
            for item in content {
                let de = item.unwrap().path();
                let p = de.to_str().unwrap().to_string();
                let new_md = fs::metadata(p.clone()).unwrap();
                files.append(&mut parse_path(p, new_md, excluded));
            }
        }
    }

    files
}

macro_rules! err {
    ($c: expr) => {
        println!("\x1b[0;31m{}\x1b[0m", $c);
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut flags: Vec<&str> = Vec::new();
    let mut pattern: &str = "";
    let mut files: Paths = Vec::new();
    let mut excluded: Paths = Vec::new();
    let mut paths: Paths = Vec::new();

    for i in 1 .. args.len() {
        if args[i].starts_with('-') {
            flags.push(&args[i]);
        }
        else {
            if args[i].starts_with('=') && args[i].ends_with('=') {
                pattern = &args[i][1..args[i].len()-1];
            }
            else if args[i].starts_with('%') {
                excluded.push(args[i][1..args[i].len()].to_string());
            }
            else {
                paths.push(args[i].to_string());
            }
        }
    }

    for path in paths {
        let md = fs::metadata(path.clone());
        match md {
            Ok(m) => {
                files.append(&mut parse_path(path, m, &mut excluded));
            }
            _ => {}
        };
    }

    if pattern.is_empty() {
        err!("Pattern was not specified or it's empty.");
        return;
    }

    let flags = Flags::new(&flags);
    let files: Vec<&str> = files.iter().map(|f| f as &str).collect();
    let results = grip(pattern, &flags, &files);
    if results.is_empty() {
        err!("No matches found. Try respelling the pattern or check if the input files exist.");
        return;
    }

    for m in results {
        println!("{m}");
    }
}
