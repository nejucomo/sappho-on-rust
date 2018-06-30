#!/bin/bash
set -efuo pipefail

SELF="$(readlink -f "$0")"
GITHOOK='.git/hooks/commit-msg'
FAILURECOUNT=0


function main
{
    cd "$(dirname "$SELF")"

    init-githook

    # Allow opt-out commits:
    if [ "$#" -gt 0 ] && grep -q '^\[SKIP-TESTS\]' "$1"
    then
        exit 0
    fi

    run-phase build
    run-phase test
    run-phase doc

    exit "$FAILURECOUNT"
}


function init-githook
{
    if ! [ -e "$GITHOOK" ]
    then
        echo 'Symlinking githook:'
        ln -sv "../../$(basename "$0")" "$GITHOOK"
    fi
}


function run-phase
{
    echo "=== cargo $1 ==="
    cargo "$1" || FAILURECOUNT="$(expr "$FAILURECOUNT" + 1)"
}


main "$@"
