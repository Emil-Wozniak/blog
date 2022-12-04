#!/usr/bin/env bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --verbose --git https://github.com/DioxusLabs/cli
dioxus serve --platform web
