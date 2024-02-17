use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn delete_macos_tempfiles<P: AsRef<Path>>(start_dir: P, prefix: &str, size_bytes: u64) {
    for entry in WalkDir::new(start_dir) {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                println!("Error reading directory: {}", err);
                continue;
            }
        };
        
        let path = entry.path();
        if path.is_file() {
            match path.file_name() {
                Some(name) => {
                    let name = name.to_string_lossy();
                    if name.starts_with(prefix) {
                        match fs::metadata(path) {
                            Ok(metadata) => {
                                if metadata.len() == size_bytes {
                                    match fs::remove_file(path) {
                                        Ok(_) => println!("{:?} has been deleted", path),
                                        Err(err) => println!("Error deleting file {:?}: {}", path, err),
                                    }
                                }
                            },
                            Err(err) => println!("Error getting metadata for {:?}: {}", path, err),
                        }
                    }
                },
                None => continue,
            }
        }
    }
}

fn main() {
    delete_macos_tempfiles(".", "._", 4096);
}
