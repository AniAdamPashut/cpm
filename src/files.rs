use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

use std::ffi::OsStr;

const TOML_FILE: &'static str = "cpi.toml";

const MAKEFILE: &'static str = "makefile";

const README: &'static str = "readme.md";

const GITIGNORE: &'static str = ".gitignore";


const FILES: &'static [&'static str] = &[
    TOML_FILE,
    MAKEFILE,
    README,
    GITIGNORE,
];

pub struct Files;

impl Files {
    fn create_file(
        parent: &Path, 
        filename: &str,
        project_name: &str,
        lang: &str
    ) -> std::io::Result<()> {
        let mut path = parent.to_owned();
        path.push(filename);

        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

        match filename {
            TOML_FILE => {
                file.write(format!(
r#"[package]
name = "{}"
version = "0.1.0"
author = "you"

dependencies = []
"#, project_name
                ).as_bytes())?;
            },

            README => {
                file.write(format!(
					"# {}", project_name
                ).as_bytes())?;
            }

            MAKEFILE => {
                let compiler = match lang {
                    "c" => "gcc",
                    "cpp" => "g++ -std=c++20",
                    _ => unreachable!()
                };
                file.write(
                    format!(
r#".SUFFIXES:
CC = {}
SRC = ./src
LANG = {}
MAIN_EXEC = {}
BUILD = ./build
CFLAGS = -Iheaders -I$(BUILD)/libs/headers -fPIC
LIBS = $(wildcard $(BUILD)/libs/objs/*.so)
SUBDIRS = $(shell find $(SRC) -type d)
FILES = $(wildcard $(addsuffix /*.$(LANG),$(SUBDIRS)))
OBJS = $(patsubst $(SRC)/%.$(LANG), $(BUILD)/objs/%.o, $(FILES))
ALL_OBJS = $(LIBS) $(OBJS)

all: $(MAIN_EXEC)

debug:
	make CFLAGS='$(CFLAGS) -Wall -Wextra -g'

release:
	make CFLAGS='$(CFLAGS) -O3'

lib: lib_target

lib_target: $(MAIN_EXEC).so

$(MAIN_EXEC): $(ALL_OBJS)
	$(CC) $(ALL_OBJS) -o $@

$(BUILD)/objs/%.o: $(SRC)/%.$(LANG)
	$(CC) $(CFLAGS) -c $< -o $@

$(MAIN_EXEC).so: $(ALL_OBJS)
	$(CC) $(CFLAGS) $(ALL_OBJS) -shared -o $@

clean:
	rm -rf $(OBJS) $(MAIN_EXEC)

init:
	@mkdir -p $(BUILD)/libs/headers
	@mkdir -p $(BUILD)/libs/objs
	@mkdir -p $(BUILD)/objs

.PHONY: all clean lib release debug init"#, 
compiler, 
lang, 
project_name).as_bytes()
                )?;
            }

            GITIGNORE => {
                file.write(
                    format!(
r#"build/
./{}
"#, project_name).as_bytes()
                )?;
            }

            _ => unreachable!()
        }
        
        Ok(())
    }

    pub fn create_all_files(root: &Path, lang: &str) -> std::io::Result<()> {
        for file in FILES {
            let project_name = root
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("");
            Files::create_file(
                root, 
                file, 
                project_name,
                lang,
            )?;
        }
        Ok(())
    }
}