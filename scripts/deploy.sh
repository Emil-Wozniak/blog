#!/usr/bin/env bash
cd ..
cargo build --release
netlify deploy --build