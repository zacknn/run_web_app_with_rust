#!/bin/bash

URL="$1"

if command -v chromium &>/dev/null; then
  BROWSER="chromium"
else
  echo "No supported browser found"
  exit 1
fi

$BROWSER --app="$URL" &
