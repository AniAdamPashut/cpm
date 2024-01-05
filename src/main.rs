use std::{path::PathBuf, fs::{self, OpenOptions}};
use std::io::Write;

use clap::Parser;
use files::Files;
use folders::Folders;

mod folders;
mod files;

const MAIN: &'static str = "src/main";

const MAIN_C_CONTENT: &'static str = 
r#"#include <stdio.h>

int main(int argc, char **argv) {
    printf("Hello, World!\n");
    return 0;
}
"#;

const MAIN_CPP_CONTENT: &'static str =
r#"#include <iostream>

int main(int argc, char **argv) {
    std::cout << "Hello, World\n";
    return 0;
}
"#;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    project_name: String,

    #[clap(short, long, default_value_t=String::from("c"))]
    lang: String,

    #[clap(long)]
    empty: bool
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Empty flag is {:?}", args.empty);
    match args.lang.as_str() {
        "c" => {
            println!("Creating a C Project");
        }
        "cpp" => {
            println!("Creating a C++ Project");
        }
        _ => {
            println!("Invalid language: {}", args.lang.as_str());
            return Ok(());
        }
    }

    let project_name = args.project_name;
    let mut root = PathBuf::from("./");
    root.push(project_name);

    if root.is_dir() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir(&root)?;
    
    Folders::create_all_folders(&root)?;
    println!("Folders are done");
    Files::create_all_files(&root, &args.lang)?;
    println!("Created all files");

    if !args.empty {
        let filename = PathBuf::from(format!("{}/{}.{}", root.to_str().unwrap(), MAIN, &args.lang));
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)?;

        match args.lang.as_str() {
            "c" => {
                file.write(MAIN_C_CONTENT.as_bytes())?;
            }

            "cpp" => {
                file.write(MAIN_CPP_CONTENT.as_bytes())?;
            }

            _ => {
                println!("Invalid Language");
            }
        }
    }
    Ok(())
}
