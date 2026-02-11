use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn category_for_extension(ext: &str) -> &str {
    match ext {
        "png" | "jpg" | "jpeg" | "gif" => "Images",
        "pdf" | "doc" | "docx" | "txt" => "Docs",
        "mp4" | "mkv" | "avi" => "Videos",
        "mp3" | "wav" => "Audio",
        "zip" | "rar" | "7z" => "Archives",
        _ => "Others",
    }
}

fn unique_path(mut path: PathBuf) -> PathBuf{
    if !path.exists(){
        return path
    }

    let stem = path.file_stem().unwrap().to_string_lossy();
    let ext = path.extension().map(|e| e.to_string_lossy());
    let parent = path.parent().unwrap();

    for i in 1..{
        let mut new_name = format!("{}({})", stem, i);
        if let Some(ext) = &ext {
            new_name.push('.');
            new_name.push_str(ext);

        }

        let candidate = parent.join(&new_name);
        if !candidate.exists(){
            return candidate
        }
    }

    unreachable!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1).map(String::as_str).unwrap_or(".");
    let dry_run = args.iter().any(|arg| arg == "--dry-run");

    let entries = fs::read_dir(dir).expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() {
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();

            let category = category_for_extension(&ext);
            let target_dir = Path::new(dir).join(category);

            if !target_dir.exists() {
                fs::create_dir(&target_dir).expect("Failed to create directory");
            }

            let file_name = path.file_name().unwrap();
            let mut new_path = target_dir.join(file_name);
            new_path = unique_path(new_path);

            if dry_run {
                println!("[DRY RUN] {:?} -> {:?}", path, new_path);
                continue;
            }

            fs::rename(&path, &new_path).expect("Failed to move file");
            println!("Moved {:?} -> {:?}", path, new_path);
        }
    }
}
