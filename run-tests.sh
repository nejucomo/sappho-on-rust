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

    if [ "$FAILURECOUNT" -eq 0 ]
    then
        echo '... No failures.'
    else
        echo "... $FAILURECOUNT failures."
    fi

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
    echo "=== $1 ==="
    if cargo "$1"
    then
        echo "--- $1: pass ---"
    else
        echo "--- $1: fail ---"
        FAILURECOUNT="$(expr "$FAILURECOUNT" + 1)"
    fi
    echo
}


main "$@"
