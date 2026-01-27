use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("Usage: organizer <directory>")
    }

    let dir_path = &args[1];
    let path = Path::new(dir_path);

    if !path.exists() || !path.is_dir(){
        println!("{dir_path} is not a valid directory");
        return;
    }

    println!("Organizing directory: {dir_path}");

}
