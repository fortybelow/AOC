#!/bin/bash

day=$1

if [ -z $day ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

echo '''
[[bin]]
name = "day'''$day'''"
path = "src/day'''$day'''/day'''$day'''.rs"
args = ["src/day'''$day'''/day'''$day'''.txt"]
''' >> Cargo.toml

mkdir -p src/day$day
touch src/day$day/day$day.rs
touch src/day$day/day$day.txt
