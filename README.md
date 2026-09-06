# rcat

A small Rust CLI that recreates some of the basic functionality of the Linux `cat` command.

## Installation

### Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/) and Cargo installed.

### Clone the repository

```bash
git clone https://github.com/DafexDV/rcat.git
cd rcat
```

### Run

You can run the program directly with Cargo:

```bash
cargo run -- <OPTIONS> <FILES>
```

For example:

```bash
cargo run -- Cargo.toml Cargo.lock
```

## Usage

```bash
rcat [OPTIONS] <FILES>
```

### Options

| Option | Description                         |
| ------ | ----------------------------------- |
| `-n`   | Number all output lines             |
| `-E`   | Display `$` at the end of each line |

### Examples

Display the contents of a file:

```bash
rcat Cargo.toml
```

Display multiple files:

```bash
rcat Cargo.toml Cargo.lock
```

Number all output lines:

```bash
rcat -n Cargo.toml
```

Display the end of each line:

```bash
rcat -E Cargo.toml
```

Combine options:

```bash
rcat -nE Cargo.toml
```

## Build

To create an optimized release binary:

```bash
cargo build --release
```

The binary will be available at:

```text
target/release/rcat
```

## License

This project is licensed under the MIT License.
