use std::{path::Path, fs};

const FOLDERS: &'static [&'static str] = &[
    "build/",
    "build/libs/",
    "build/libs/headers",
    "build/libs/objs",
    "build/objs",
    "src/",
    "headers/",
];

pub struct Folders;

impl Folders {
    fn create_folder(parent: &Path, folder: &str) -> std::io::Result<()> {
        let mut path = parent.to_owned();
        path.push(folder);
        fs::create_dir(path)?;
        Ok(())
    }

    pub fn create_all_folders(root: &Path) -> std::io::Result<()> {
        for folder in FOLDERS {
            Folders::create_folder(root, folder)?;
        }
        Ok(())
    }
}