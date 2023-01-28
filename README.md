# Voltaire 📚

Voltaire is a really simple Rust cli grammar checker!
<br>
It should support various languages, and works thanks to [LanguageTool](https://languagetool.org) API

## Features

- Grammar checking
- Colored terminal output
- Replacement suggestions
- Error explanation

## Installation

### From source

Prerequisites
- [Git](https://git-scm.com/downloads)
- [Rust toolchain](https://www.rust-lang.org/tools/install)

Command line instructions
```bash
# Clone the repository
git clone https://github.com/emsquid/voltaire

# Build and install
cargo install --path voltaire

# Use freely like a good frenchie
voltaire "i'm relly goud at anglish" --verbose
```

## Command line usage

```
Who would be better than Voltaire to check (French) grammar

Usage: voltaire [OPTIONS] <TEXT>

Arguments:
  <TEXT>  Text to analyze

Options:
  -n, --number <NUMBER>  Maximum number of corrections [default: 3]
  -v, --verbose          show more informations
  -h, --help             Print help
  -V, --version          Print version
```

## Progress

- Documentation
    * [ ] Write a greater README
    * [ ] Make releases/packages (publish on crates.io eventually)
- APIs support
    * [ ] Find some other APIs that would work
    * [ ] Test the limits (requests wise) of the current one
