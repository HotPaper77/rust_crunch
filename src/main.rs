use std::env;
use std::io::Error;
use std::fs::{self, DirEntry};
use std::io::{self, BufReader};
use std::path::Path;
use crate::fs::File;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <flag> [<value>]", args[0]);
        return;
    }

    let flag = &args[1];

    match flag.as_str() {
        "-h" | "--help" => {
            println!("Help: Use the flag '-dir' to share the directory to traverse");
        }
        "-v" | "--version" => {
            println!("Version 1.0");
        }
        "-dir" | "--directory" => {
            if args.len() >= 3 {
                let input_directory = &args[2];
                let directory = input_directory
                    .trim_matches(|c| c == '"' || c == '\'')
                    .trim();

                let path = Path::new(directory);

                let callback = |entry: &std::fs::DirEntry| -> Result<(),Error> {
                    let file = File::open(entry.path())?;
                    let reader = BufReader::new(file);
            
                    let count = reader.lines().count();
            
                    println!("{:?}", count);
                    Ok(())
                };

                let _ = visit_dirs(path, &callback);
            } else {
                println!("Error: Missing value for flag {}", flag);
            }
        }
        _ => {
            println!("Unknown flag: {}", flag);
        }
    }
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                if let Err(err) = cb(&entry) {
                    eprintln!("Error processing entry: {:?}", err);
                }
            }
        }
    } else {
        println!("{:?} is not a directory", dir);
    }
    Ok(())
}
