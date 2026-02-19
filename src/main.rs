mod config;
mod log;
mod utils;

use config::Config;
use log::{OperationLog, log_file_name};
use std::env;
use std::fs;
use std::path::Path;
use utils::{category_for_extension, unique_path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let dry_run = args.iter().any(|arg| arg == "--dry-run");
    let force = args.iter().any(|arg| arg == "--force");
    let revert = args.iter().any(|arg| arg == "--revert");
    let init_config = args.iter().any(|arg| arg == "--init-config");

    if init_config {
        match Config::save_default() {
            Ok(path) => println!("Config file created at: {:?}", path),
            Err(e) => eprintln!("Error: {}", e),
        }
        return;
    }

    let dir = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .map(String::as_str)
        .unwrap_or(".");

    let config = Config::load();

    if revert {
        // Revert mode: reverse the operations from the log
        let log = OperationLog::load(dir);

        if log.operations.is_empty() {
            println!("No operations to revert. Log is empty.");
            return;
        }

        println!("Found {} operations to revert", log.operations.len());
        let mut reverted_count = 0;

        // Reverse the operations in reverse order (LIFO)
        for op in log.operations.iter().rev() {
            let from_path = Path::new(&op.from);
            let to_path = Path::new(&op.to);

            if dry_run {
                println!("[DRY RUN] Revert: {:?} <- {:?}", from_path, to_path);
                continue;
            }

            if !to_path.exists() {
                println!("Warning: {:?} does not exist, skipping revert", to_path);
                continue;
            }

            if from_path.exists() {
                println!("Warning: {:?} already exists, skipping revert", from_path);
                continue;
            }

            if let Some(parent) = from_path.parent() {
                if !parent.exists() {
                    println!(
                        "Warning: Parent directory {:?} does not exist, skipping",
                        parent
                    );
                    continue;
                }
            }

            match fs::rename(to_path, from_path) {
                Ok(_) => {
                    println!("Reverted: {:?} <- {:?}", from_path, to_path);
                    reverted_count += 1;
                }
                Err(e) => {
                    println!("Error reverting {:?}: {}", to_path, e);
                }
            }
        }

        if dry_run {
            println!(
                "\n[DRY RUN] Would revert {} operations",
                log.operations.len()
            );
        } else {
            println!("\nDone. {} operations reverted", reverted_count);

            if reverted_count > 0 {
                let new_log = OperationLog::new();
                new_log.save(dir);
                println!("Log cleared.");
            }
        }
    } else {
        // Organize mode: categorize files and log operations
        let mut log = OperationLog::load(dir);
        let entries = fs::read_dir(dir).expect("Failed to read directory");

        let mut moved_count = 0;

        for entry in entries {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();

            if path.file_name().and_then(|n| n.to_str()) == Some(log_file_name()) {
                continue;
            }

            if path.is_file() {
                let ext = path
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();

                let category = category_for_extension(&ext, &config);
                let target_dir = Path::new(dir).join(&category);

                let file_name = path.file_name().unwrap();
                let mut new_path = target_dir.join(file_name);

                if !force {
                    new_path = unique_path(new_path);
                }

                if dry_run {
                    println!("[DRY RUN] {:?} -> {:?}", path, new_path);
                    continue;
                }

                if !target_dir.exists() {
                    fs::create_dir(&target_dir).expect("Failed to create directory");
                }

                log.add_operation(
                    path.to_string_lossy().to_string(),
                    new_path.to_string_lossy().to_string(),
                );

                fs::rename(&path, &new_path).expect("Failed to move file");
                println!("Moved {:?} -> {:?}", path, new_path);
                moved_count += 1;
            }
        }

        if !dry_run && moved_count > 0 {
            log.save(dir);
        }

        println!("\nDone. {} files organized", moved_count);
    }
}