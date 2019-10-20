# util.sh
# Version: 1.3.15
# https://github.com/Noah2610/util.sh

set -o pipefail

# Returns `0` or `1` depending on if the given string is available as a command.
function is_available {
  local cmd="$1"
  [ -z "$cmd" ] && err "No command to check for availability given to function \`is_available\`"
  command -v "$cmd" &> /dev/null
}

# Returns `0` or `1` depending on if the command is running (using `pgrep`).
function is_running {
  check "pgrep"
  local to_check="$1"
  [ -z "$to_check" ] && err "No command to check if running given to function \`is_running\`"
  pgrep -xc "$to_check" &> /dev/null
}

# Exit with error message (`err`) if the given string is not available as a command.
function check {
  local cmd="$1"
  [ -z "$cmd" ] && err "No command to check given to function \`check\`"
  is_available "$cmd" &> /dev/null || err "\`$( colored "$COLOR_CODE" "$cmd" )\` is not available."
}

# Exit with error message if the given file path does not exist.
function check_file {
  local FILE="$1"
  [ -f "$FILE" ] || err "File does not exist: $( colored "$COLOR_CODE" "$FILE" )"
}

# Exit with error message if the given directory path does not exist.
function check_dir {
  local DIR="$1"
  [ -d "$DIR" ] || err "Directory does not exist: $( colored "$COLOR_CODE" "$DIR" )"
}

# Print the given string to stdout and to the `$LOGFILE` (if one exists), with color.
function err {
  print_log "$( semantic_date )\nERROR: $1"
  (1>&2 echo -e "$( colored "$COLOR_ERR" "ERROR:" ) $1\nExiting.")
  exit 1
}

# Print the given string to the `$LOGFILE` (if one exists), and strip color from the text.
function print_log {
  local txt="$1"
  [ -z "$txt" ] && err "No message text given to function \`print_log\`"
  [ -n "$LOGFILE" ] && echo -e "$( strip_ansi_codes "$txt" )\n" >> "$LOGFILE"
}

# Strip ansi codes from the given string.
function strip_ansi_codes {
  local txt="$1"
  if is_available "sed"; then
    echo -e "$txt" | sed "s,\x1B\[[0-9;]*[a-zA-Z],,g"
  else
    echo -e "$txt"
  fi
}

# This function returns a string wrapped in ansi color codes.
# It takes two arguments:
#   color - ansi color code (ex. "0;33", "0;36;40")
#   txt   - the text to be colored
function colored {
  local color="$1"
  local txt="$2"
  { [ -z "$color" ] || [ -z "$txt" ]; } &&
    err "Function \`colored\` needs two arguments: the color and the text."
  echo "\033[${color}m${txt}\033[m"
}

# This function simply prints out the passed argument with a color.
function msg {
  print_log "$1"
  echo -e "$( colored "${COLOR_MSG}" "${1}" )"
}

# Same as `msg`, but also makes the text bold.
function msg_strong {
  print_log "$( semantic_date )\n${1}"
  echo -e "$( colored "${COLOR_MSG_STRONG}" "${1}" )"
}

# Print out a date string in a specifc format.
# If the command `boxed-string` is available, then it calls that with the date string.
# boxed-string: https://gist.github.com/Noah2610/2c4a92f6732419becade2f76bc943039
function semantic_date {
  check "date"
  local dfmt
  local dstr
  dfmt='+%F %T'
  dstr="$( date "$dfmt" )"
  if is_available "boxed-string"; then
    BOXED_PADDING_HORZ=1 \
    BOXED_PADDING_VERT=0 \
    boxed-string -- "$dstr"
  else
    echo "$dstr" | tee -a "$LOGFILE"
  fi
}

# Tries to run the given command and hides its output.
# If the command fails, then it prints the output with `err`.
# If the variable `$also_to_stderr` is set, then additionally to writing
# the commands output to the `$LOGFILE`, it also prints it to stderr.
function try_run {
  local cmd="$1"
  [ -z "$cmd" ] && err "No command given."
  local out
  local out_files=("$LOGFILE")
  [ -n "$also_to_stderr" ] && out_files+=("/dev/stderr")
  msg "${spacing}Running: \033[${COLOR_CODE}m${cmd}\033[m"
  if ! out="$( $cmd 2>&1 | tee -a "${out_files[@]}" )"; then
    err "Command failed:\n  \033[${COLOR_CODE}m${cmd}\033[m\nReturned:\n${out}"
  fi
}

# Returns `0` or `1` depending on if the final command should be run in a new terminal.
# For very specific use-case(s).
function should_run_in_terminal {
  [ -n "$RUN_NEW_TERMINAL" ] &&
    [ "$RUN_NEW_TERMINAL" != "0" ] &&
    [ -n "$TERMINAL" ] &&
    is_available "$TERMINAL"
}

# Run the given command in a new terminal.
function run_terminal {
  local cmd="$1"
  local cmd_bash="bash -c '$cmd || (echo -e \"----------\n[CONTINUE]\"; read)'"
  [ -n "$cmd" ] || err "No command given to function \`run_terminal\`."
  check "$TERMINAL"
  case "$TERMINAL" in
    "termite")
      termite -d "$ROOT" -e "$cmd_bash" & \
      disown
      ;;
    *)
      err "Function \`run_terminal\` is not configured for terminal '$TERMINAL'"
      ;;
  esac
}

check "basename"
check "dirname"
check "tee"

# Set `$ROOT` variable to the directory of this script,
# unless it was already set.
# If the name of the directory is 'bin', then set `$ROOT`
# to the parent directory of 'bin/'.
[ -z "$ROOT" ] &&
  ROOT="$( cd "$( dirname "$0" )" || exit 1; pwd )" &&
  [ "$( basename "$ROOT" )" = "bin" ] &&
  ROOT="$( dirname "$ROOT" )"

# Set the `$LOGFILE` variable unless it was already set.
[ -z "$LOGFILE" ] &&
  LOGFILE="$ROOT/.$( basename "$0" ).log"

# Create the directory path to `$LOGFILE` if it doesn't exist.
logfile_dir="$( dirname "$LOGFILE" )"
! [ -d "$logfile_dir" ] && mkdir -p "$logfile_dir"
unset logfile_dir

# Set the `$TERMINAL` variable unless it was already set.
[ -z "$TERMINAL" ] && TERMINAL="termite"

# Set some ansi color code variables.
COLOR_ERR="1;31"
COLOR_MSG="0;33"
COLOR_MSG_STRONG="1;33"
COLOR_CODE="0;36;40"
