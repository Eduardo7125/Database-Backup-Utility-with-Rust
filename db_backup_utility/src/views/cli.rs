use std::{fs::{read_dir, ReadDir}, io::Error};

#[allow(dead_code)]
pub fn print_success(message: &str) {
    println!("\x1b[32m[SUCCESS]\x1b[0m {}", message); // Verde
}

pub fn print_error(message: &str) {
    eprintln!("\x1b[31m[ERROR]\x1b[0m {}", message); // Rojo
}

pub async fn list_backups() -> Result<(), Box<dyn std::error::Error>> {

    let backup_dir: &str = "backups";
    let paths: Result<ReadDir, Error> = read_dir(backup_dir);

    println!("Available backups:");
    match paths {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(path) => println!("{}", path.path().display()),
                    Err(e) => print_error(&format!("Error reading entry: {}", e)),
                }
            }
        }
        Err(e) => print_error(&format!("Error reading directory: {}", e)),
    }

    Ok(())
}