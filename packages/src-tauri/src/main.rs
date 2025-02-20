#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
  fs,
  path::Path,
  process::exit,
};

use std::cmp::Ordering;

#[derive(Debug)]
struct DirectoryInfo {
  path: String,
  size: u64,
  file_count: u64,
}

impl DirectoryInfo {
  fn new(path: String, size: u64, file_count: u64) -> Self {
    DirectoryInfo {
      path,
      size,
      file_count,
    }
  }
}

fn calculate_directory_info(path: &Path) -> DirectoryInfo {
  let mut size: u64 = 0;
  let mut file_count: u64 = 0;

  if let Ok(entries) = fs::read_dir(path) {
    for entry in entries {
      if let Ok(entry) = entry {
        let metadata = entry.metadata().unwrap();
        if metadata.is_file() {
          size += metadata.len();
          file_count += 1;
        } else if metadata.is_dir() {
          let dir_info = calculate_directory_info(&entry.path());
          size += dir_info.size;
          file_count += dir_info.file_count;
        }
      }
    }
  }
  DirectoryInfo::new(path.display().to_string(), size, file_count)
}

fn format_size(size: u64) -> String {
  const KB: u64 = 1024;
  const MB: u64 = KB * 1024;
  const GB: u64 = MB * 1024;

  if size >= GB {
    format!("{:.2} GB", size as f64 / GB as f64)
  } else if size >= MB {
    format!("{:.2} MB", size as f64 / MB as f64)
  } else if size >= KB {
    format!("{:.2} KB", size as f64 / KB as f64)
  } else {
    format!("{:.2} bytes", size as f64)
  }
}

use colored::*;

fn print_directory_tree(dir_info: &DirectoryInfo, indent: usize, total_size: u64, root: &DirectoryInfo) {
    let indent_str = " ".repeat(indent * 4);
    let percentage;
    if root.path==dir_info.path {
      percentage=100.0;
    }
    else{
      percentage = (dir_info.size as f64 / total_size as f64) * 100.0;
    }

    let color = if percentage > 50.0 {
        Color::Red
    } else if percentage > 25.0 {
        Color::Yellow
    } else {
        Color::Green
    };

    println!(
        "{}{} {} Files ({}, {}%)",
        indent_str,
        Path::new(&dir_info.path).file_name().unwrap().to_string_lossy().color(color),
        format!("{}", dir_info.file_count).color(color),
        format_size(dir_info.size).color(color),
        format!("{:0.2}", percentage).color(color)
    );

    if let Ok(entries) = fs::read_dir(&dir_info.path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let metadata = entry.metadata().unwrap();
                if metadata.is_dir() {
                    let dir_info = calculate_directory_info(&entry.path());
                    print_directory_tree(&dir_info, indent + 1, total_size, &root);
                }
            }
        }
    }
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      match app.get_cli_matches() {
        Ok(matches) => {
          if let Some(path_arg) = matches.args.get("path") {
            if let Some(path_value) = path_arg.value.as_str() {
              let path = Path::new(path_value);
              let dir_info = calculate_directory_info(path);

              print_directory_tree(&dir_info, 0, dir_info.size, &dir_info);
            }
          } else {
            println!("Path argument not found");
          }
        }
        Err(e) => {
          eprintln!("Failed to get CLI matches: {}", e);
          exit(1);
        }
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
