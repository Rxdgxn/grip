#[derive(Debug)]
pub struct Flags {
    numbers: bool,
    names: bool,
    insensitive: bool,
    invert: bool,
    entire: bool,
    highlight: bool
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            numbers: flags.contains(&"-n"),
            names: flags.contains(&"-l"),
            insensitive: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
            entire: flags.contains(&"-x"),
            highlight: flags.contains(&"-hl")
        }
    }
}

macro_rules! push {
    ($x: expr, $flags: expr, $filename: expr, $prefix: expr, $line_clone: expr, $i: expr) => {
        $x.push(match $flags.names {
            true => $filename.to_string(),
            false => "\x1b[0;45m".to_string() + &$prefix + "\x1b[0m" + &$line_clone
        });
        if $flags.names && $i {
            break;
        }
    };
}

pub fn grip(pattern: &str, flags: &Flags, files: &[&str]) -> Vec<String> {
    let mut wanted: Vec<String> = Vec::new();
    let mut unwanted: Vec<String> = Vec::new();

    for filename in files {
        let file_res = std::fs::read_to_string(filename);
        match file_res {
            Ok(_) => {}
            Err(_) => continue
        }
        let file = file_res.unwrap();

        let split = file.split('\n');
        let mut lc = 1;

        for line in split {
            let mut line_clone = String::from(line.clone().trim());

            if line_clone.is_empty() {
                lc += 1;
                continue;
            }

            let pat = match flags.insensitive {
                true => pattern.to_lowercase(),
                false => pattern.to_string()
            };
            let line = match flags.insensitive {
                true => line.to_lowercase(),
                false => line.to_string()
            };
            
            let num = &(lc.to_string() + ":");
            let prefix = filename.to_string() + ":" + match flags.numbers {
                true => num,
                false => ""
            };

            lc += 1;

            if flags.entire {
                if pat == line.trim() {
                    push!(wanted, flags, filename, prefix, line_clone, !flags.invert);
                }
                else {
                    push!(unwanted, flags, filename, prefix, line_clone, flags.invert);
                }
                continue;
            }

            let start = line.find(&pat);
            if let Some(idx) = start {
                if flags.highlight {
                    let diff = line.len() - line.trim_start().len();
                    line_clone.insert_str(idx - diff, "\x1b[0;42m");
                    line_clone.insert_str(idx + pat.len() - diff + "\x1b[0;42m".len(), "\x1b[0m");
                }
                push!(wanted, flags, filename, prefix, line_clone, !flags.invert);
            }
            else {
                push!(unwanted, flags, filename, prefix, line_clone, flags.invert);
            }
        }
    }

    if flags.invert {
        return unwanted;
    }
    wanted
}