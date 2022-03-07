<p align="center">
  <img width="400" alt="Fu example screenshot showing the size of files in the project" src="https://user-images.githubusercontent.com/4056725/156439322-8e8f56b7-a4ae-4489-8429-323489f6a8a2.png">
  <br>
  <a href="https://github.com/Angelmmiguel/fu/actions?query=workflow%3ARust">
    <img src="https://github.com/Angelmmiguel/fu/workflows/Rust/badge.svg" alt="Build Status">
  </a>
  <img src="https://img.shields.io/badge/License-Apache--2.0-blue" alt="license">
</p>

# fu

`fu` is a `du` alike CLI. It comes with a set of new features and a better output. For example, it allows you to sort and limit the number of entries and provides a colorized output to highlight heaviest entries.

## Installation

### MacOS

To install it using view, run the following commands:

```
brew tap Angelmmiguel/tap
brew install Angelmmiguel/tap/fu
```

<details>
  <summary>Install <code>fu</code> without brew</summary>

Run the following commands to:

1. Download the file from the GitHub releases page
1. Uncompress it
1. Remove the quarantine attribute from MacOS
1. Move the CLI binary to /usr/local/bin
1. Remove any remaining file

```
curl -L \
  https://github.com/Angelmmiguel/fu/releases/latest/download/fu-x86_64-apple-darwin.tar.gz \
    -o /tmp/fu-x86_64-apple-darwin.tar.gz && \
  tar -xvf /tmp/fu-x86_64-apple-darwin.tar.gz -C /tmp && \
  xattr -d com.apple.quarantine /tmp/fu-x86_64-apple-darwin/fu && \
  mv /tmp/fu-x86_64-apple-darwin/fu /usr/local/bin && \
  rm -r /tmp/fu-x86_64-apple-darwin.tar.gz /tmp/fu-x86_64-apple-darwin
```

The `xattr` call is required because downloaded binaries are marked as "quarantine" by MacOS. In addition to that, the system may block `fu` CLI due to unknown signature. You can allow it by accessing the _Security and Privacy_ system preference panel and clicking on the _Allow anyway_ button.

This will install the `fu` CLI in the `/usr/local/bin` folder.

</details>

### Linux

Run the following commands in a terminal:

```
curl -L \
  https://github.com/Angelmmiguel/fu/releases/latest/download/fu-x86_64-unknown-linux-gnu.tar.gz \
    -o /tmp/fu-x86_64-unknown-linux-gnu.tar.gz && \
  tar -xvf /tmp/fu-x86_64-unknown-linux-gnu.tar.gz -C /tmp && \
  mv /tmp/fu-x86_64-unknown-linux-gnu/fu /usr/local/bin && \
  rm -r /tmp/fu-x86_64-unknown-linux-gnu.tar.gz /tmp/fu-x86_64-unknown-linux-gnu
```

This will install the `fu` CLI in the `/usr/local/bin` folder.

### Windows

For Windows, please follow the next steps:

- Download the latest release from [the releases page](https://github.com/Angelmmiguel/fu/releases/latest/download/fu-x86_64-pc-windows-gnu.tar.gz)
- Uncompress it
- Place the `fu.exe` binary in a folder that it's referenced in your `PATH`

## Usage

```
$ fu --help
fu
Search for a pattern in a file and display the lines that contain it

USAGE:
    fu [OPTIONS] <PATH>

ARGS:
    <PATH>    The path to the file to read

OPTIONS:
    -h, --help         Print help information
        --no-colors    Disable the colors in the output
        --no-header    Hide the headers from the output
    -s, --sort         Sort the output based on the size
    -t, --top <TOP>    Sort and limit the output to the N heaviest entries
```

### Sort the entries by size

```
$ fu --sort ./
DISK    BYTES   PATH
258M    238M    target
492K    71K     .git
12K     11K     LICENSE
8K      6K      Cargo.lock
12K     4K      src
4K      1K      README.md
4K      361B    Cargo.toml
4K      263B    .github
4K      131B    .gitignore
```

### Just the sum of the folder

```
$ fu .
DISK    BYTES   PATH
259M    239M    .
```

### Sort and show top N entries

```
$ fu --top 2 ./
DISK    BYTES   PATH
258M    238M    target
492K    71K     .git
```

### Use glob

```
$ fu ./*.md
DISK    BYTES   PATH
4K      1K      ./README.md
```

### Hide headers

```
$ fu --top 2 --no-headers ./
258M    238M    target
492K    71K     .git
```

## Development

To develop `fu` you need to install the Rust language on your environment:

- [Get started with Rust](https://www.rust-lang.org/learn/get-started)

Then, you need to clone the project and start exploring the CLI:

```
# Clone the repo
git clone https://github.com/Angelmmiguel/fu.git && cd fu

# Run the CLI (development) against the current folder
cargo run -- ./

# Run tests
cargo tests

# Build the CLI
cargo build

# Build using the release profile
cargo build --release
```

## Why?

I developed `fu` for two main reasons:

- I love to use small projects to learn new languages, frameworks, patterns, etc. So, I use `fu` as an excuse to start learning Rust language.
- Try to offer an alternative and add missing features for well known CLIs I use everyday. I use `du`, although most of the times I combine it with other CLIs that helps me to filter and sort.

That's all! Feel free to request new features and let's discuss about how we can improve `fu` :)

## License

`fu` is released under the Apache License v2.0 (See [LICENSE](https://github.com/Angelmmiguel/fu/blob/main/LICENSE)).

Copyright 2022 [Angel M Miguel](https://angel.kiwi).
