#!/bin/bash
# Get files
git stash
git pull
git stash apply

# Upload files
git add .
git commit -m "$1"
git push