#!/bin/bash

version=${1}
tmp_file=$(mktemp)
path="./Cargo.toml"

if [ -z "$version" ]; then
    echo "Usage: $0 <new_version>"
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
    echo "No version line found in $path."
    exit 1
fi

mv "$tmp_file" "$path"

git add .
git commit -m "ape_ast: $version"
git push -u origin main

cargo publish
