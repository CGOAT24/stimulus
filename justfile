#!/usr/bin/env just --justfile

lint:
    cd api && cargo clippy
    cd app/src-tauri && cargo clippy

fmt:
    cargo fmt --manifest-path api/Cargo.toml
    cargo fmt --manifest-path app/src-tauri/Cargo.toml

clean:
    cd api && cargo clean
    cd app/src-tauri && cargo clean

[group('build')]
[group('api')]
api_build:
    cd api && cargo build

[group('build')]
[group('app')]
app_build target="":
    cd app && bun run tauri {{target}} build

[group('build')]
build target="":
    just app_build {{target}}
    just api_build

[group('api')]
api_run:
    cd api && cargo loco start

[group('app')]
app_run target="":
    cd app && bun run tauri {{target}} dev

dev target="":
    just api_run & just app_run {{target}}