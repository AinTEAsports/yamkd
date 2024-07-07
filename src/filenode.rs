use std::fs::File;
use crate::parser::OUTER_SEPARATOR;
use crate::utils::get_lastindex;

#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub is_dir: bool,
}

impl FileNode {
    pub fn new(name: String, is_dir: bool) -> Self {
        FileNode { name, is_dir }
    }

    pub fn create(&self) -> Result<(), String> {
        let abscurrpath = std::env::current_dir().unwrap().display().to_string();

        if std::path::Path::new(&self.name).exists() { Ok(()) }
        else {
            if self.name.contains(OUTER_SEPARATOR) && self.name.chars().nth(self.name.len()-1).unwrap() != OUTER_SEPARATOR {
                let lasti = get_lastindex(&self.name, OUTER_SEPARATOR).unwrap();

                let (folder, _) = &self.name.split_at(lasti);

                return match FileNode::new(folder.to_string(), true).create() {
                    Ok(_) => self.create_file_nocheck(),
                    Err(e) => Err(e)
                }
            }

            if !self.is_dir && self.name.chars().nth(self.name.len()-1).unwrap() != OUTER_SEPARATOR {
                match File::create(&self.name) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("could not create file {}'{}'", abscurrpath, self.name))
                }
            } else {
                match std::fs::create_dir_all(&self.name) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("could not create folder '{}' in supposingly existing folder '{}'", self.name, abscurrpath))
                }
            }
        }
    }

    pub fn create_file_nocheck(&self) -> Result<(), String> {
        match File::create(&self.name) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("could not create file '{}'", self.name))
        }
    }
}
