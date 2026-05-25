#!/bin/bash
# remove.sh
# Usage: ./remove.sh "YouTube"

NAME="$1"
APPS_FILE="$HOME/.config/webappman/apps.txt"

tmpfile=$(mktemp)
mkdir -p "$(dirname "$APPS_FILE")"
touch "$APPS_FILE"

awk -F'|' -v name="$NAME" '$1 != name { print }' "$APPS_FILE" > "$tmpfile"
mv "$tmpfile" "$APPS_FILE"

echo "  Removed: $NAME"
