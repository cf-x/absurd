#!/bin/bash

crate=${1}
version=${2}
tmp_file=$(mktemp)
path="./interpreter/ape_$crate/Cargo.toml"

if [ -z "$crate" ]; then
    echo "usage: $0 <cargo_name> <new_version>"
    exit 1
fi

if [ -z "$version" ]; then
    echo "usage: $0 <cargo_name> <new_version>"
    exit 1
fi

if [ ! -f "$path" ]; then
    echo "Failed to find $path."
    exit 1
fi

while IFS= read -r line; do
    if [[ $line == version\ *=* ]]; then
        echo "version = \"$version\"" >> "$tmp_file"
    else
        echo "$line" >> "$tmp_file"
    fi
done < "$path"

if [ ! -s "$tmp_file" ]; then
    rm "$tmp_file"
    echo "no version line found in $path."
    exit 1
fi

mv "$tmp_file" "$path"

git add ./interpreter/ape_$crate
git commit -m "ape_$crate: $version"
git push -u origin main


cd ./interpreter/ape_$crate
cargo publish
