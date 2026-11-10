default: list

list:
    @just --list

download:
    cargo run --release --locked -- download

compile:
    cargo run --release --locked -- compile
