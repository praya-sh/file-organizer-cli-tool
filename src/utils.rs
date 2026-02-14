use std::path::PathBuf;

pub fn category_for_extension(ext: &str) -> &str {
    match ext {
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" => "Images",
        "pdf" | "doc" | "docx" | "txt" | "rtf" | "otd" => "Docs",
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" => "Videos",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => "Audio",
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => "Archives",
        _ => "Others",
    }
}

pub fn unique_path(path: PathBuf) -> PathBuf {
    if !path.exists() {
        return path;
    }

    let stem = path.file_stem().unwrap().to_string_lossy();
    let ext = path.extension().map(|e| e.to_string_lossy());
    let parent = path.parent().unwrap();

    for i in 1.. {
        let mut new_name = format!("{}({})", stem, i);
        if let Some(ext) = &ext {
            new_name.push('.');
            new_name.push_str(ext);
        }

        let candidate = parent.join(&new_name);
        if !candidate.exists() {
            return candidate;
        }
    }

    unreachable!()
}
