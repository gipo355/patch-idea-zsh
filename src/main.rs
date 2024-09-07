use chrono::Local;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};

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
    // get the .local/share/ directory path
    let dir_path = dirs::data_local_dir().unwrap().join("applications");

    // get the home directory path
    let home_dir = env::var("HOME").expect("HOME environment variable not set");

    // create the new Exec line with the home directory path to use
    let new_exec_line = format!(
        r#"Exec=/usr/bin/zsh -i -c "{}/.local/share/JetBrains/Toolbox/apps/intellij-idea-ultimate/bin/idea" %u"#,
        home_dir
    );

    // Find all matching files
    let files: Vec<_> = fs::read_dir(dir_path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                // apply the filter
                .starts_with("jetbrains-idea-")
                && entry.file_name().to_string_lossy().ends_with(".desktop")
        })
        // collect the files into a vector
        .collect();

    // loop through each file and patch it
    for file in files {
        let file_path = file.path();

        // Read the file content
        let mut content = String::new();
        File::open(&file_path)?.read_to_string(&mut content)?;

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

        // Write the modified content back to the file
        let mut file = File::create(&file_path)?;
        file.write_all(final_content.as_bytes())?;

        println!("Patched file: {:?}", file_path);
    }

    Ok(())
}
