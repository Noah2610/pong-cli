# shellcheck source=./util.sh source=./share.sh
_dir="$( dirname "$0" )"
[ -f "${_dir}/util.sh" ] || bash "${_dir}/download-util.sh" || exit 1
source "${_dir}/util.sh"
unset _dir

# https://stackoverflow.com/a/17841619/10927893
function join_by { local IFS="$1"; shift; echo "$*"; }

function cargo_cmd {
    local cargo_subcmd="$1"
    [ -z "$cargo_subcmd" ] && err "No cargo subcommand passed to function \`cargo_cmd\`"

    check "cargo"
    local args=("$@")
    unset 'args[0]'
    local features_str
    local features=("$RUN_FEATURES")
    features_str="$( join_by ',' "${features[@]}" )"
    [ -n "$features_str" ] && features_str="--features $features_str"
    local cmd="cargo +$RUST_VERSION $cargo_subcmd $features_str ${args[*]} && read"
    local run_msg
    run_msg="$( colored "$COLOR_MSG_STRONG" "RUNNING:" ) $( colored "$COLOR_CODE" "$cmd" )"
    echo -e "$run_msg"
    if should_run_in_terminal; then
        run_terminal "$cmd"
    else
        $cmd
    fi
}

RUST_VERSION="stable"
_logdir="${ROOT}/logs"
[ -d "$_logdir" ] || mkdir -p "$_logdir"
LOGFILE="${_logdir}/$( basename "$0" ).log"
unset _logdir
