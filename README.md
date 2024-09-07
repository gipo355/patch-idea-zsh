# patch-idea-zsh

⚠ LINUX ONLY ⚠

Small CLI utility to patch JetBrains IDEA desktop files to use a shell to launch the IDE and inherit the environment variables (paths to runtimes, etc.)

It will find all the desktop files in the local data directory and patch them to use the shell you choose (sh/bash/zsh) and the path to the shell executable.

The shell executable is chosen by the user, and the path to the shell executable is determined by the operating system.

## Features

- Find all the desktop files in the local data directory
- Patch the desktop files to use the shell you choose (sh/bash/zsh) and the path to the shell executable
- Choose the shell you want to use (sh/bash/zsh)
- Choose the JetBrains IDEs you want to patch (comma-separated numbers, default is all)
- Choose the files you want to patch (default is all)
- Patch the files you choose
- Show the patching results

## Usage

```bash
patch-idea-zsh
```

## Installation

### Install from source

```bash
git clone https://github.com/gipo355/patch-idea-zsh.git
cd patch-idea-zsh
cargo install --path .
```

### Install with [binstall](https://github.com/ryankurte/cargo-binstall)

```bash
cargo install binstall
binstall patch-idea-zsh
```

### Install from crates.io

```bash
cargo install patch-idea-zsh
```

## License

MIT
