use chrono::Local;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::io::{stdin, stdout, Write as IoWrite};
use std::path::PathBuf;
use std::process;

/**
* path:
* ~/.local/share/applications/jetbrains-idea-4de0683c-bebb-4365-885d-0112e2b356b5.desktop
*
* content:
* [Desktop Entry]
* Name=IntelliJ IDEA Ultimate 2024.2.1
* Exec=/usr/bin/zsh -i -c "/home/wolf/.local/share/JetBrains/Toolbox/apps/intellij-idea-ultimate/bin/idea" %u
* Version=1.0
* Type=Application
* Categories=Development;IDE;
* Terminal=false
* Icon=/home/wolf/.local/share/JetBrains/Toolbox/apps/intellij-idea-ultimate/bin/idea.svg
* Comment=The Leading Java and Kotlin IDE
* StartupWMClass=jetbrains-idea
* StartupNotify=true
*/
fn main() -> io::Result<()> {
    // Get the .local/share/ directory path
    let dir_path = match dirs::data_local_dir() {
        Some(path) => path.join("applications"),
        None => {
            eprintln!("Failed to get the local data directory.");
            process::exit(1);
        }
    };

    // Get the home directory path
    let home_dir = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("HOME environment variable not set.");
        process::exit(1);
    });

    // create the new Exec line with the home directory path to use
    let new_exec_line = format!(
        r#"Exec=/usr/bin/zsh -i -c "{}/.local/share/JetBrains/Toolbox/apps/intellij-idea-ultimate/bin/idea" %u"#,
        home_dir
    );

    // Find all matching files
    let files: Vec<PathBuf> = match fs::read_dir(&dir_path) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with("jetbrains-idea-")
                    && entry.file_name().to_string_lossy().ends_with(".desktop")
            })
            .map(|entry| entry.path())
            .collect(),
        Err(_) => {
            eprintln!("Failed to read the directory: {:?}", dir_path);
            process::exit(1);
        }
    };

    if files.is_empty() {
        eprintln!("No matching JetBrains IDEA desktop files found.");
        process::exit(1);
    }

    // List all found files
    println!("Found the following JetBrains IDEA desktop files:");
    for (index, file) in files.iter().enumerate() {
        println!("{}: {:?}", index + 1, file);
    }

    // Ask for confirmation
    println!(
        "Enter the numbers of the files you want to patch, separated by commas (default is all):"
    );
    print!("> ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let files_to_patch: Vec<PathBuf> = if input.is_empty() {
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
    };

    if files_to_patch.is_empty() {
        eprintln!("No files selected for patching.");
        process::exit(1);
    }

    // loop through each file and patch it
    for file_path in files_to_patch {
        // Read the file content
        let mut content = String::new();
        if File::open(&file_path)?
            .read_to_string(&mut content)
            .is_err()
        {
            eprintln!("Failed to read the file: {:?}", file_path);
            process::exit(1);
        }

        // Store the old content (for appending to the end of the file)
        let old_content = content.clone();

        // Get the current date and time
        let current_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Modify the content (add the new Exec line)
        let modified_content = content
            .lines()
            .map(|line| {
                if line.starts_with("Exec=") {
                    new_exec_line.clone()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Modify the old content
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

        // Append the modified date to the old content
        let final_old_content =
            format!("\n# patched on {}\n{}", current_date, modified_old_content);

        // Append the old content and the current date to the modified content
        let final_content = format!("{}\n\n{}", modified_content, final_old_content);

        println!("Modified content:\n{}", final_content);

        // Write the modified content back to the file
        // if File::create(&file_path)?
        //     .write_all(final_content.as_bytes())
        //     .is_err()
        // {
        //     eprintln!("Failed to write to the file: {:?}", file_path);
        //     process::exit(1);
        // }

        println!("Patched file: {:?}", file_path);
    }

    Ok(())
}
