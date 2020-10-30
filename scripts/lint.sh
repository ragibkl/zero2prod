#!/usr/bin/env bash

cargo check
cargo fmt --all
cargo clippy -- -D warnings
