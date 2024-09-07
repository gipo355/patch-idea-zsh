use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn get_local_data_dir() -> Option<PathBuf> {
    dirs::data_local_dir().map(|path| path.join("applications"))
}

pub fn get_home_dir() -> Option<String> {
    env::var("HOME").ok()
}

pub fn choose_shell() -> Option<String> {
    println!("Choose the shell to use (sh/bash/zsh, default is zsh):");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut shell_input = String::new();
    io::stdin().read_line(&mut shell_input).unwrap();
    let shell_input = shell_input.trim().to_lowercase();

    match shell_input.as_str() {
        "bash" => Some("bash".to_string()),
        "sh" => Some("sh".to_string()),
        "zsh" | "" => Some("zsh".to_string()),
        _ => None,
    }
}

pub fn get_shell_path(shell: &str) -> Option<String> {
    which::which(shell)
        .ok()
        .map(|path| path.to_string_lossy().to_string())
}

pub fn create_exec_line(shell_path: &str, home_dir: &str) -> String {
    format!(
        r#"Exec={} -i -c "{}/.local/share/JetBrains/Toolbox/apps/intellij-idea-ultimate/bin/idea" %u"#,
        shell_path, home_dir
    )
}
