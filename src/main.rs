use image::DynamicImage;
use img_hash::HasherConfig;
use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;
use std::{env, fs, process};

fn main() {
    let all_arguments: Vec<String> = env::args().collect();

    let folder_to_check: String;

    if all_arguments.len() < 2 {
        folder_to_check = match env::current_dir() {
            Ok(t) => t.to_string_lossy().to_string(),
            Err(_) => {
                println!("Cannot find provided directory");
                process::exit(1);
            }
        }
    } else {
        folder_to_check = all_arguments[1].clone();
    }
    let folder_to_check = folder_to_check.trim_end_matches('/').to_string() + "/";

    if !Path::new(&folder_to_check).exists() {
        println!("'{}' don't exists", all_arguments[1]);
        process::exit(1);
    }
    if !Path::new(&folder_to_check).is_dir() {
        println!("'{}' isn't directory", all_arguments[1]);
        process::exit(1);
    }

    let supported_extensions = [
        ".jpg", ".jpeg", ".png", ".bmp", ".tiff", ".tif", ".pnm", ".tga", ".ff", ".gif", ".jif", ".jfi", ".ico",
        ".webp", ".avif",
    ];

    let mut folders_to_check: Vec<String> = Vec::new();
    let mut current_folder: String;

    let mut files_to_check: Vec<(String, u64)> = Vec::new();

    folders_to_check.push(folder_to_check);

    while !folders_to_check.is_empty() {
        current_folder = folders_to_check.pop().unwrap();

        let read_dir = match fs::read_dir(&current_folder) {
            Ok(t) => t,
            _ => continue,
        };
        for entry in read_dir {
            let entry_data = match entry {
                Ok(t) => t,
                Err(_) => continue, //Permissions denied
            };
            let metadata: Metadata = match entry_data.metadata() {
                Ok(t) => t,
                Err(_) => continue, //Permissions denied
            };
            if metadata.is_dir() {
                let folder_name: String = match entry_data.file_name().into_string() {
                    Ok(t) => t,
                    Err(_) => continue, // Permission Denied
                };

                folders_to_check.push(format!("{}{}/", current_folder, folder_name));
            } else if metadata.is_file() {
                let file_name: String = match entry_data.file_name().into_string() {
                    Ok(t) => t,
                    Err(_) => continue, // Permission Denied
                };
                if !supported_extensions
                    .iter()
                    .any(|e| file_name.to_lowercase().ends_with(e))
                {
                    continue;
                }
                files_to_check.push((format!("{}{}", current_folder, file_name), metadata.len()));
            }
        }
    }

    let mut time_sum: u64 = 0;
    let mut size_sum = 0;

    for (file, size) in &files_to_check {
        let start_time = SystemTime::now();
        println!("OOO - Trying to open {}", file);
        let _image = match image::open(file) {
            Ok(t) => t,
            Err(e) => {
                println!("LLL - Failed to open file {} - reason {}", file, e);
                continue;
            }
        };
        let end_time = SystemTime::now();
        println!(
            "TTT - Opening file '{}' ({} Kilobytes) took '{}' took miliseconds",
            file,
            size / 1024,
            end_time.duration_since(start_time).unwrap().as_millis()
        );

        // let hasher = HasherConfig::with_bytes_type::<[u8; 8]>().to_hasher();
        // let hash = hasher.hash_image(&image);
        // let mut buf = [0u8; 8];
        // buf.copy_from_slice(&hash.as_bytes());
        //
        // let end_time2 = SystemTime::now();
        // println!(
        //     "Hashing file '{}' ({} Kilobytes) took '{}' took miliseconds",
        //     file,
        //     size / 1024,
        //     end_time2.duration_since(end_time).unwrap().as_millis()
        // );
        //
        // time_sum += end_time2.duration_since(start_time).unwrap().as_millis() as u64;
        size_sum += size;
    }
    println!();
    println!(
        "Opening and hashing {} images which takes all {} MB, took {} seconds",
        files_to_check.len(),
        size_sum / 1024 / 1024,
        time_sum as f64 / 1024.0
    );
}
