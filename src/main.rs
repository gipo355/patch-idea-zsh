mod ide;
mod patch;
mod shell;

use std::io;
use std::process;

fn main() -> io::Result<()> {
    let dir_path = shell::get_local_data_dir().unwrap_or_else(|| {
        eprintln!("\x1b[31mFailed to get the local data directory.\x1b[0m");
        process::exit(1);
    });

    let home_dir = shell::get_home_dir().unwrap_or_else(|| {
        eprintln!("\x1b[31mHOME environment variable not set.\x1b[0m");
        process::exit(1);
    });

    let shell = shell::choose_shell().unwrap_or_else(|| {
        eprintln!(
            "\x1b[31mInvalid shell choice. Please choose either 'bash', 'sh' or 'zsh'.\x1b[0m"
        );
        process::exit(1);
    });

    let shell_path = shell::get_shell_path(&shell).unwrap_or_else(|| {
        eprintln!(
            "\x1b[31mFailed to find the path for the shell: {}\x1b[0m",
            shell
        );
        process::exit(1);
    });

    println!("\x1b[32mUsing shell: {}\x1b[0m", shell_path);

    let new_exec_line = shell::create_exec_line(&shell_path, &home_dir);

    let selected_ides = ide::choose_ides();

    let files = patch::find_matching_files(&dir_path, &selected_ides).unwrap_or_else(|_| {
        eprintln!(
            "\x1b[31mFailed to read the directory: {:?}\x1b[0m",
            dir_path
        );
        process::exit(1);
    });

    if files.is_empty() {
        eprintln!("\x1b[31mNo matching JetBrains IDEA desktop files found.\x1b[0m");
        process::exit(1);
    }

    println!("\x1b[32mFound the following JetBrains IDEA desktop files:\x1b[0m");
    for (index, file) in files.iter().enumerate() {
        println!("{}: {:?}", index + 1, file);
    }

    let files_to_patch = patch::choose_files_to_patch(&files);

    if files_to_patch.is_empty() {
        eprintln!("\x1b[31mNo files selected for patching.\x1b[0m");
        process::exit(1);
    }

    patch::patch_files(&files_to_patch, &new_exec_line, &shell_path)?;

    Ok(())
}
