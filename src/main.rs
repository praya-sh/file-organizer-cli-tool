use std::fs;

fn main() {
    let entries = fs::read_dir(".").expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file(){
            let name = path.file_name().unwrap().to_string_lossy();
            let ext = path.extension().unwrap_or_default().to_string_lossy();

            println!("{name} -> {ext}");

            
        }
    }
}
