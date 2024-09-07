use crate::ide::JetBrainsIDE;
use chrono::Local;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub fn find_matching_files(
    dir_path: &PathBuf,
    selected_ides: &Vec<JetBrainsIDE>,
) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir_path)?;
    let files: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_string();
            selected_ides.iter().any(|ide| {
                file_name.starts_with(&format!("jetbrains-{}", ide.as_str()))
                    && file_name.ends_with(".desktop")
            })
        })
        .map(|entry| entry.path())
        .collect();
    Ok(files)
}

pub fn choose_files_to_patch(files: &Vec<PathBuf>) -> Vec<PathBuf> {
    println!(
        "Enter the numbers of the files you want to patch, separated by commas (default is all):"
    );
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() {
        files.clone()
    } else {
        let indices: Vec<usize> = input
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();

        indices
            .into_iter()
            .filter_map(|i| files.get(i - 1).cloned())
            .collect()
    }
}

pub fn patch_files(
    files_to_patch: &Vec<PathBuf>,
    new_exec_line: &str,
    shell_path: &str,
) -> io::Result<()> {
    for file_path in files_to_patch {
        let mut content = String::new();
        if File::open(&file_path)?
            .read_to_string(&mut content)
            .is_err()
        {
            eprintln!("\x1b[31mFailed to read the file: {:?}\x1b[0m", file_path);
            continue;
        }

        if content
            .lines()
            .any(|line| line.starts_with(&format!("Exec={}", shell_path)))
        {
            println!(
                "\x1b[33mx\x1b[0m File {:?} is already patched. Skipping.",
                file_path
            );
            continue;
        }

        let old_content = content.clone();
        let current_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let modified_content = content
            .lines()
            .map(|line| {
                if line.starts_with("Exec=") {
                    new_exec_line.to_string()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let modified_old_content = old_content
            .lines()
            .map(|line| {
                if line.starts_with('#') {
                    line.to_string()
                } else {
                    format!("# {}", line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let final_old_content =
            format!("\n# patched on {}\n{}", current_date, modified_old_content);
        let final_content = format!("{}\n\n{}", modified_content, final_old_content);

        if File::create(&file_path)?
            .write_all(final_content.as_bytes())
            .is_err()
        {
            eprintln!("Failed to write to the file: {:?}", file_path);
            continue;
        }

        println!("\x1b[32mv\x1b[0m Patched file: {:?}", file_path);
    }

    Ok(())
}
