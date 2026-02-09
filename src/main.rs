use std::fs;
use std::path::Path;

fn category_for_extentions(ext: &str) ->&str{
    match ext {
        "png" | "jpg" | "jpeg" | "gif" => "Images",
        "pdf" | "doc" | "docx" | "txt" => "Docs",
        "mp4" | "mkv" | "avi" => "Videos",
        "mp3" | "wav" => "Audio",
        "zip" | "rar" | "7z" => "Archives",
        _ => "Others",
    }
}

fn main() {
    let entries = fs::read_dir(".").expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file(){
            let name = path.file_name().unwrap().to_string_lossy();
            let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();

            let category = category_for_extentions(&ext);
            let dir_path = Path::new(category);

            if !dir_path.exists(){
                fs::create_dir(dir_path).expect("failed to create directory");
                println!("Created folder: {category}")
            }

            println!("{name} -> {ext}, {category}");      
        }
    }
}
