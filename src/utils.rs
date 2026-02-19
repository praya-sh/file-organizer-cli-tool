use std::path::PathBuf;

use crate::config::Config;

pub fn category_for_extension(ext: &str, config: &Config) -> String {
    for (category, extensions) in &config.categories {
        if extensions.iter().any(|e| e == ext) {
            return category.clone();
        }
    }
    "Others".to_string()
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
