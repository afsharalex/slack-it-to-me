# Slack It To Me!

## Getting started
A simple `cargo build` will download and compile all dependencies
and the binary itself.

## Running
There are two options to run:
`cargo build` followed by `./target/debug/slack-it-to-me` or what I find more convenient:
`cargo run -- -c my-conf-file.conf`. Using the `--` allows the flags and subcommands entered thereafter to be passed to the binary rather than Cargo.