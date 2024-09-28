#!/bin/bash

# Get files
bash pull.sh

# Upload files
git add .
git commit -m "$1"
git push