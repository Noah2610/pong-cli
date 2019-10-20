#!/bin/bash
# Small script that attempts to download my util.sh helper script.
# Version: 1.0.2
URL="https://raw.githubusercontent.com/Noah2610/util.sh/master/util.sh"
function err { (echo -e "ERROR: $1\nExiting" 1>&2); exit 1; }
function check { command -v "$1" &> /dev/null || err "'$1' is not available"; }
check "dirname"; check "curl"
path="$( dirname "$0" )/util.sh"
echo -e "Attempting to download \`util.sh\` script from\n  ${URL}\nto\n  ${path}"
[ -f "$path" ] && err "File exists at '${path}'"
out="$( { ( curl "$URL" ) 1> "$path"; } 2>&1 )" || { rm "$path"; err "$out"; }
