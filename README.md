# fu

`fu` is a `du` alike CLI. It comes with a set a new features and a better output. For example, it allows you to sort and limit the number of entries and provides a colorized output to highlight heaviest entries.

## Installation

### Package managers

This work is still in progress. `fu` will be available in the different package managers :)

### Manual

#### MacOS

Run the following commands in a terminal:

```
curl -L \
  https://github.com/Angelmmiguel/fu/releases/latest/download/fu-x86_64-apple-darwin.tar.gz \
    -o /tmp/fu-x86_64-apple-darwin.tar.gz && \
  tar -xvf /tmp/fu-x86_64-apple-darwin.tar.gz -C /tmp && \
  mv /tmp/fu-x86_64-apple-darwin/fu /usr/local/bin && \
  rm -r /tmp/fu-x86_64-apple-darwin.tar.gz /tmp/fu-x86_64-apple-darwin
```

> NOTE: MacOS may block `fu` CLI due to unknown signature. You can allow it by accessing the _Security and Privacy_ system preference panel and clicking on the _Allow anyway_ button.

This will install the `fu` CLI in the `/usr/local/bin` folder.

#### Linux

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

#### Windows

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
