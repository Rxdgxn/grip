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
            // Grip applies .gitignore rules forcefully (TODO: toggle flag?)
            if path.ends_with(".gitignore") {
                let x = fs::read_to_string(&path).unwrap();
                let content = x.split('\n');
                let prev_path = &path[0..path.len() - 10]; // 10 = len(".gitignore")
                for mut p in content {
                    if p.starts_with('/') { // This may break the search, it's been tested only with examples like '/target'
                        p = &p[1..p.len()];
                    }
                    excluded.push(prev_path.to_string() + p);
                }
            }
            files.push(path);
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
            // Pattern delimiters (custom made since Powershell wants to mess with quotes)
            if args[i].starts_with('=') && args[i].ends_with('=') {
                pattern = &args[i][1..args[i].len()-1];
            }
            // Prefix for excluding certain paths (full paths from the root folder of the program must be provided however)
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
