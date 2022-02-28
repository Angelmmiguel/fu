# fu
A du replacement with more features

## Installation

TODO

## Usage

```
fu --help
```

### Sort the entries by size

```
fu --sort ./
```

### Sort and show top N entries

```
fu --top 2 ./
```

### Use glob

```
fu ./*.md
```

## Development

To develop `fu` you need to install the Rust language on your environment:

* [Get started with Rust](https://www.rust-lang.org/learn/get-started)

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
```

## Why?

I developed `fu` for two main reasons:

- I love to use small projects to learn new languages, frameworks, patterns, etc. So, I use `fu` as an excuse to start learning Rust language.
- Try to offer an alternative and add missing features for well known CLIs I use everyday. I use `du`, although most of the times I combine it with other CLIs that helps me to filter and sort.

That's all! Feel free to request new features and let's discuss about how we can improve `fu` :)

## License

`fu` is released under the Apache License v2.0 (See [LICENSE](https://github.com/Angelmmiguel/fu/blob/main/LICENSE)). Copyright 2022 [Angel M Miguel](https://angel.kiwi).

