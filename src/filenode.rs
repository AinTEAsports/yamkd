use std::fs::File;

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
        if std::path::Path::new(&self.name).exists() { Ok(()) }
        else {
            if !self.is_dir {
                match File::create(&self.name) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("could not create file '{}'", self.name))
                }
            } else {
                match std::fs::create_dir_all(&self.name) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("could not creare folder '{}'", self.name))
                }
            }
        }
    }
}
