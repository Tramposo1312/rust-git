# Rust Git

A simple git-like tool made in Rust.

## Features (Planned)

- Basic repo initialisation
- Adding files to staging area
- Checking repo status
- Commiting changes

## Installation

```bash
cargo install --path .

## Usage
rustgit init
rustgit add file1.txt file2.txt
rustgit status
rustgit commit --message "init commit"