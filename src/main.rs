use chrono::Local;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::io::{stdin, stdout, Write as IoWrite};
use std::path::PathBuf;
use std::process;

#[derive(Debug, Clone)]
enum JetBrainsIDE {
    IntelliJ,
    PyCharm,
    WebStorm,
    PhpStorm,
    CLion,
    Rider,
    DataGrip,
    RubyMine,
    AppCode,
}

impl JetBrainsIDE {
    fn as_str(&self) -> &'static str {
        match self {
            JetBrainsIDE::IntelliJ => "idea",
            JetBrainsIDE::PyCharm => "pycharm",
            JetBrainsIDE::WebStorm => "webstorm",
            JetBrainsIDE::PhpStorm => "phpstorm",
            JetBrainsIDE::CLion => "clion",
            JetBrainsIDE::Rider => "rider",
            JetBrainsIDE::DataGrip => "datagrip",
            JetBrainsIDE::RubyMine => "rubymine",
            JetBrainsIDE::AppCode => "appcode",
        }
    }

    #[allow(dead_code)]
    fn from_str(input: &str) -> Option<JetBrainsIDE> {
        match input {
            "idea" => Some(JetBrainsIDE::IntelliJ),
            "pycharm" => Some(JetBrainsIDE::PyCharm),
            "webstorm" => Some(JetBrainsIDE::WebStorm),
            "phpstorm" => Some(JetBrainsIDE::PhpStorm),
            "clion" => Some(JetBrainsIDE::CLion),
            "rider" => Some(JetBrainsIDE::Rider),
            "datagrip" => Some(JetBrainsIDE::DataGrip),
            "rubymine" => Some(JetBrainsIDE::RubyMine),
            "appcode" => Some(JetBrainsIDE::AppCode),
            _ => None,
        }
    }

    fn all() -> Vec<JetBrainsIDE> {
        vec![
            JetBrainsIDE::IntelliJ,
            JetBrainsIDE::PyCharm,
            JetBrainsIDE::WebStorm,
            JetBrainsIDE::PhpStorm,
            JetBrainsIDE::CLion,
            JetBrainsIDE::Rider,
            JetBrainsIDE::DataGrip,
            JetBrainsIDE::RubyMine,
            JetBrainsIDE::AppCode,
        ]
    }
}

fn main() -> io::Result<()> {
    // Get the .local/share/ directory path
    let dir_path = match dirs::data_local_dir() {
        Some(path) => path.join("applications"),
        None => {
            eprintln!("\x1b[31mFailed to get the local data directory.\x1b[0m");
            process::exit(1);
        }
    };

    // Get the home directory path
    let home_dir = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("\x1b[31mHOME environment variable not set.\x1b[0m");
        process::exit(1);
    });

    // Ask the user to choose the shell
    println!("Choose the shell to use (sh/bash/zsh, default is zsh):");
    print!("> ");
    stdout().flush().unwrap();

    let mut shell_input = String::new();
    stdin().read_line(&mut shell_input).unwrap();
    let shell_input = shell_input.trim().to_lowercase();

    let shell = match shell_input.as_str() {
        "bash" => "bash",
        "sh" => "sh",
        "zsh" | "" => "zsh",
        _ => {
            eprintln!(
                "\x1b[31mInvalid shell choice. Please choose either 'bash', 'sh' or 'zsh'.\x1b[0m"
            );
            process::exit(1);
        }
    };

    // Get the path of the chosen shell
    let shell_path = match which::which(shell) {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => {
            eprintln!(
                "\x1b[31mFailed to find the path for the shell: {}\x1b[0m",
                shell
            );
            process::exit(1);
        }
    };

    println!("\x1b[32mUsing shell: {}\x1b[0m", shell_path);

    // Create the new Exec line with the home directory path to use
    let new_exec_line = format!(
        r#"Exec={} -i -c "{}/.local/share/JetBrains/Toolbox/apps/intellij-idea-ultimate/bin/idea" %u"#,
        shell_path, home_dir
    );

    // Ask the user to choose the IDEs
    println!("Choose the JetBrains IDEs to patch (comma-separated numbers, default is all):");
    let all_ides = JetBrainsIDE::all();
    for (index, ide) in all_ides.iter().enumerate() {
        println!("{}: {}", index + 1, ide.as_str());
    }
    print!("> ");
    stdout().flush().unwrap();

    let mut ide_input = String::new();
    stdin().read_line(&mut ide_input).unwrap();
    let ide_input = ide_input.trim();

    let selected_ides: Vec<JetBrainsIDE> = if ide_input.is_empty() {
        all_ides
    } else {
        ide_input
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .filter_map(|i| all_ides.get(i - 1).cloned())
            .collect()
    };

    // Find all matching files
    let files: Vec<PathBuf> = match fs::read_dir(&dir_path) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let file_name = entry.file_name().to_string_lossy().to_string();
                selected_ides.iter().any(|ide| {
                    file_name.starts_with(&format!("jetbrains-{}", ide.as_str()))
                        && file_name.ends_with(".desktop")
                })
            })
            .map(|entry| entry.path())
            .collect(),
        Err(_) => {
            eprintln!(
                "\x1b[31mFailed to read the directory: {:?}\x1b[0m",
                dir_path
            );
            process::exit(1);
        }
    };

    if files.is_empty() {
        eprintln!("\x1b[31mNo matching JetBrains IDEA desktop files found.\x1b[0m");
        process::exit(1);
    }

    // List all found files
    println!("\x1b[32mFound the following JetBrains IDEA desktop files:\x1b[0m");
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
        eprintln!("\x1b[31mNo files selected for patching.\x1b[0m");
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
            eprintln!("\x1b[31mFailed to read the file: {:?}\x1b[0m", file_path);
            process::exit(1);
        }

        // Check if the file is already patched
        if content
            .lines()
            .any(|line| line.starts_with(&format!("Exec={}", shell_path)))
        {
            println!(
                // red 31m
                // "\x1b[31mx\x1b[0m File {:?} is already patched. Skipping.",
                // yellow 33m
                "\x1b[33mx\x1b[0m File {:?} is already patched. Skipping.",
                file_path
            );
            continue;
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

        // println!("Modified content:\n{}", final_content);

        // Write the modified content back to the file
        if File::create(&file_path)?
            .write_all(final_content.as_bytes())
            .is_err()
        {
            eprintln!("Failed to write to the file: {:?}", file_path);
            process::exit(1);
        }

        // green 32m
        println!("\x1b[32mv\x1b[0m Patched file: {:?}", file_path);
    }

    Ok(())
}
