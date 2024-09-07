use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum JetBrainsIDE {
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
    pub fn as_str(&self) -> &'static str {
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

    pub fn all() -> Vec<JetBrainsIDE> {
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

pub fn choose_ides() -> Vec<JetBrainsIDE> {
    println!("Choose the JetBrains IDEs to patch (comma-separated numbers, default is all):");
    let all_ides = JetBrainsIDE::all();
    for (index, ide) in all_ides.iter().enumerate() {
        println!("{}: {}", index + 1, ide.as_str());
    }
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut ide_input = String::new();
    std::io::stdin().read_line(&mut ide_input).unwrap();
    let ide_input = ide_input.trim();

    if ide_input.is_empty() {
        all_ides
    } else {
        ide_input
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .filter_map(|i| all_ides.get(i - 1).cloned())
            .collect()
    }
}
