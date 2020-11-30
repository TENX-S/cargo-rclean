Clean your rust projects, recursively

# Disclaimer
This program comes with no warranty. You must use this program at your own risk.

# Example
<img src="https://github.com/TENX-S/cargo-rclean/blob/main/screenshots/cargo-rclean.gif?raw=true">

# Installation
## Using Cargo

```shell script
$ cargo install cargo-rclean
```

Or you may want to keep up with upstream

```shell script
$ cargo install --git https://github.com/TENX-S/cargo-rclean.git --branch main
```

# Usage
```shell script
USAGE:
    cargo-rclean [FLAGS] <PATH>

FLAGS:
    -d, --doc        Whether or not to clean just the documentation directory
    -h, --help       Prints help information
    -r, --release    Whether or not to clean release artifacts
    -V, --version    Prints version information

ARGS:
    <PATH>    Cleans up all rust projects in the specified directory
```
