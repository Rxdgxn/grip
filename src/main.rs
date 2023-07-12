/// Grip: grep but in Rust
use grip::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut flags: Vec<&str> = Vec::new();
    let mut pattern: &str = "";
    let mut files: Vec<&str> = Vec::new();

    for i in 1 .. args.len() {
        if args[i].starts_with('-') {
            flags.push(&args[i]);
        }
        else {
            if args[i].starts_with('='){
                pattern = &args[i][1..args[i].len()];
            }
            else {
                // TODO: check for directory
                files.push(&args[i]);
            }
        }
    }

    if pattern.is_empty() {
        println!("[ERROR]: Pattern was not specified or it's empty");
        return;
    }

    let flags = Flags::new(&flags);

    let results = grep(pattern, &flags, &files);
    match results {
        Ok(xs) => {
            for x in xs { println!("{x}"); }
        }
        Err(e) => println!("[ERROR]: {e}")
    }
}
