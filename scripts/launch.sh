#!/bin/bash

URL = "$1"

if command -v zen-browser &>/dev/null; then
  BROWSER="zen-browser"
elif command -v chromium &>/dev/null; then
  BROWSER="chromium"
elif command -v google-chrome &>/dev/null; then
  BROWSER="google-chrome"
else
  echo "No supported browser found"
  exit 1
fi

$BROWSER --app="$URL" &
