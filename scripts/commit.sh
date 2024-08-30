#!/bin/bash

version=${1}

git add .
git commit -m "update: v$version"
git push -u origin main
echo "version updated to $version"