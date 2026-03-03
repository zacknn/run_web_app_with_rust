#!/bin/bash

NAME="$1"
URL="$2"

# where to store apps
APPS_FILE="$HOME/.config/webappman/apps.txt"

# Create the folder if it doesn't exist yet
mkdir -p "$HOME/.config/webappman"

# Append name|url as one line to the file
echo "$NAME|$URL" >>"$APPS_FILE"

echo "Done added, $NAME"
