use std::path::Path;
use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} <directory> <max-depth> -d/-f", args[0]);
        return;
    }

    let root = &args[1];

    let opt = &args[3];

    if !Path::new(root).exists() {
        println!("Error: Directory does not exist.");
        return;
    }

    let max_depth: usize = args[2].parse().unwrap_or(usize::MAX);

    let mut dir_sizes: Vec<(String, u64)> = Vec::new();

    let walker = WalkDir::new(root).max_depth(max_depth).into_iter();

    match opt.as_str() {
        "-d" => {
            for entry in walker.filter_map(|e| e.ok()) {
                if entry.file_type().is_dir() {
                    let dir_size = calculate_directory_size(&entry.path());
                    dir_sizes.push((entry.path().display().to_string(), dir_size));
                }
            }
        }
        "-f" => {
            // Iterate over all entries in the directory
            for entry in walker.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    // If it's a file, get its size
                    let file_size = fs::metadata(path).unwrap().len();
                    dir_sizes.push((path.display().to_string(), file_size));
                }
            }
        }
        _ => {
            println!("Error: options dees not exist only -d/-f.");
            return;
        }
    }

    // Sort directories by size
    dir_sizes.sort_by(|a, b| b.1.cmp(&a.1));

    // Print sorted directories
    for (dir_path, size) in dir_sizes {
        println!("{} | {}", human_readable_size(size), dir_path);
    }
}

fn calculate_directory_size(path: &Path) -> u64 {
    let mut total_size: u64 = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            total_size += entry.metadata().unwrap().len();
        }
    }

    total_size
}

fn human_readable_size(size: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    if size < (KB as u64) {
        format!("{} B", size)
    } else if size < (MB as u64) {
        format!("{:.2} KB", size as f64 / KB)
    } else if size < (GB as u64) {
        format!("{:.2} MB", size as f64 / MB)
    } else if size < (TB as u64) {
        format!("{:.2} GB", size as f64 / GB)
    } else {
        format!("{:.2} TB", size as f64 / TB)
    }
}
