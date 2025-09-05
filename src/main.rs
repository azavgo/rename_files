use std::env::{args};
use rand::prelude::*;  

use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum RenameFilesError { 
    IOError(std::io::Error),  
}

impl From<std::io::Error> for RenameFilesError {
    fn from(error: std::io::Error) -> Self {
        RenameFilesError::IOError(error)
    }
}

pub fn random_string(n: u8) -> String {
    let random_characters_set = 
        ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', 
        '4', '5', '6', '7', '8', '9'];

    let mut rng = rand::rng();
    let mut rename_set: Vec<char> = vec!['-'];

    for _i in 0..n {
        rename_set.push(random_characters_set[rng.random_range(0..36)]); 
    }
    rename_set.into_iter().collect()
}

pub fn rename_files(target_dir: &str) -> Result<(), RenameFilesError> {
    let paths = fs::read_dir(target_dir)?;
    
    for entry in paths {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() 
                && let Some(file_stem) = path.file_stem() {
                let new_name = format!("{}{}.{}", file_stem.to_string_lossy(), random_string(8), extension.to_string_lossy());
                let new_path = Path::new(target_dir).join(new_name);

                fs::rename(&path, &new_path)?;
                println!("Renamed {:?} to {:?}", path.file_name(), new_path.file_name());
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), RenameFilesError> {
    match args().nth(1) {
        Some(target_dir) => rename_files(target_dir.as_str())?,
        None => println!("No argument passed"), 
    }
    Ok(())
}
