#!/usr/bin/env bash
set -eu
cd $1
cargo clippy --all-features --tests -- -D clippy::all -D warnings --no-deps
cargo fmt -- --check
