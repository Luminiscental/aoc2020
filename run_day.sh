#!/bin/sh

if [ $# -ne 1 ]; then
    echo "expected one argument"
    echo "usage: $0 day_number (e.g. $0 1)"
else
    sed -i "s/day[0-9]\\+/day$1/g" src/main.rs
    cargo run --release
fi
