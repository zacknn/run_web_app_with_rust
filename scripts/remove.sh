#!/bin/bash
# remove.sh
# Usage: ./remove.sh "YouTube"

NAME="$1"
APPS_FILE="$HOME/.config/webappman/apps.txt"

# sed -i = edit file in place
# /^$NAME|/d = delete lines that start with NAME|
sed -i "/^$NAME|/d" "$APPS_FILE"

echo "  Removed: $NAME"
