#!/bin/sh

rustup target add wasm32-unknown-unknown

# Install cargo-leptos
cargo install --locked cargo-leptos

# Build the Leptos app
cargo leptos build