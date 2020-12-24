#!/bin/sh

cargo fmt
cargo run --release -- $*
