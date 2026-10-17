default: list

list:
    @just --list

download:
    cargo run --release

compile:
    typst compile ygo-db-support.typ ygo-db-support.png
