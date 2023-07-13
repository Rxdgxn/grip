use anyhow::Error;

#[derive(Debug)]
pub struct Flags {
    numbers: bool,
    names: bool,
    insensitive: bool,
    invert: bool,
    entire: bool
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            numbers: flags.contains(&"-n"),
            names: flags.contains(&"-l"),
            insensitive: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
            entire: flags.contains(&"-x"),
        }
    }
}

macro_rules! push {
    ($x: expr, $flags: expr, $filename: expr, $prefix: expr, $clone: expr, $i: expr) => {
        $x.push(match $flags.names {
            true => $filename.to_string(),
            false => "\x1b[0;45m".to_string() + &$prefix + "\x1b[0m" + &$clone
        });
        if $flags.names && $i {
            break;
        }
    };
}

pub fn grip(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut wanted: Vec<String> = Vec::new();
    let mut unwanted: Vec<String> = Vec::new();

    for filename in files {
        let file_res = std::fs::read_to_string(filename);
        match file_res {
            Ok(_) => {}
            Err(_) => continue // return Err(Error::msg(format!("File {filename} doesn't exist or format is not supported")))
        }
        let file = file_res.unwrap();

        let split = file.split('\n'); // Note: empty lines are not counted
        let mut lc = 1;

        for line in split {
            let clone = line.clone();

            if clone.trim().is_empty() {
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
                    push!(wanted, flags, filename, prefix, clone, !flags.invert);
                }
                else {
                    push!(unwanted, flags, filename, prefix, clone, flags.invert);
                }
                continue;
            }

            if line.contains(&pat) {
                push!(wanted, flags, filename, prefix, clone, !flags.invert);
            }
            else {
                push!(unwanted, flags, filename, prefix, clone, flags.invert);
            }
        }
    }

    if flags.invert {
        return Ok(unwanted);
    }
    Ok(wanted)
}