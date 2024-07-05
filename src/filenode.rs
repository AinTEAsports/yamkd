use crate::parser::{ gets, OUTER_SEPARATOR };

use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub is_dir: bool,
}

impl FileNode {
    pub fn new(name: String, is_dir: bool) -> Self {
        // println!("[FILENODE] is_dir={}", is_dir);
        //
        // let len = name.len();
        // let mut processed = name.clone();
        //
        // if gets(name, len-1) == Some(OUTER_SEPARATOR) { processed.truncate(len-1); }
        //
        // FileNode {
        //     name: processed,
        //     is_dir,
        // }

        FileNode { name, is_dir }
    }

    pub fn create(&self) -> Result<(), String> {
        if std::path::Path::new(&self.name).exists() { Ok(()) }
        else {
            if !self.is_dir {
                match File::create(&self.name) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("could not create file '{}'", self.name))
                }
            } else {
                match std::fs::create_dir(&self.name) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("could not creare folder '{}'", self.name))
                }
            }
        }
    }
}
