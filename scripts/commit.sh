#!/bin/bash

version=${1}

echo "commiting changes..."
git add .
git commit -m "$version"
git push -u origin main
echo "version updated to $version"