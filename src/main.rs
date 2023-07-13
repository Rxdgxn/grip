/// Grip: grep but in Rust
use grip::*;
use std::fs;

fn parse_path(path: &str, md: fs::Metadata) -> Option<Vec<String>> {
    let mut files: Vec<String> = Vec::new();

    if md.is_file() {
        files.push(path.to_string());
        return Some(files);
    }
    else if md.is_dir() {
        let content = fs::read_dir(path).unwrap();
        for item in content {
            let de = item.unwrap().path();
            let p = de.to_str().unwrap();
            let new_md = fs::metadata(p).unwrap();
            files.append(&mut parse_path(p, new_md).unwrap());
        }
        return Some(files);
    }

    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut flags: Vec<&str> = Vec::new();
    let mut pattern: &str = "";
    let mut files: Vec<String> = Vec::new();

    for i in 1 .. args.len() {
        if args[i].starts_with('-') {
            flags.push(&args[i]);
        }
        else {
            if args[i].starts_with('=') && args[i].ends_with('=') {
                pattern = &args[i][1..args[i].len()-1];
            }
            else {
                let md = fs::metadata(&args[i]).unwrap();
                files.append(&mut parse_path(&args[i], md).unwrap());
            }
        }
    }

    if pattern.is_empty() {
        println!("[ERROR]: Pattern was not specified or it's empty");
        return;
    }

    let flags = Flags::new(&flags);
    
    let files: Vec<&str> = files.iter().map(|f| f as &str).collect();

    let results = grip(pattern, &flags, &files);
    match results {
        Ok(xs) => {
            for x in xs { println!("{x}"); }
        }
        Err(e) => println!("[ERROR]: {e}")
    }
}
