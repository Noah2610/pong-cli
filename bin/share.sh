# shellcheck source=./util.sh source=./share.sh
_dir="$( dirname "$0" )"
[ -f "${_dir}/util.sh" ] || bash "${_dir}/download-util.sh" || exit 1
source "${_dir}/util.sh"
unset _dir

function cargo_cmd {
    local cargo_subcmd="$1"
    [ -z "$cargo_subcmd" ] && err "No cargo subcommand passed to function \`cargo_cmd\`"

    check "cargo"
    local args=("$@")
    unset 'args[0]'
    local features=("$RUN_FEATURES")
    local features_str
    features_str="${features[*]}"
    [ -n "$features_str" ] && features_str="--features $features_str"
    local cmd=(
        "cargo"
        "+$RUST_VERSION"
        "--color always"
        "$cargo_subcmd"
        "$features_str"
        "${args[@]}"
    )
    local run_msg
    run_msg="$( colored "$COLOR_MSG_STRONG" "RUNNING:" ) $( colored "$COLOR_CODE" "${cmd[*]}" )"
    echo -e "$run_msg"
    if should_run_in_terminal; then
        run_terminal "${cmd[*]}"
    else
        eval "${cmd[*]}"
    fi
}

[ -z "$RUST_VERSION" ] && RUST_VERSION="nightly-2019-08-13"
_logdir="${ROOT}/logs"
[ -d "$_logdir" ] || mkdir -p "$_logdir"
LOGFILE="${_logdir}/$( basename "$0" ).log"
unset _logdir
