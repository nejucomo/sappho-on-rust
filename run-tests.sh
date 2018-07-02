#!/bin/bash
set -efuo pipefail

SELF="$(readlink -f "$0")"
GITHOOK='.git/hooks/commit-msg'
FAILURECOUNT=0
ALL=0



function main
{
    cd "$(dirname "$SELF")"

    init-githook

    if [ "$#" -gt 0 ] && [ "$1" = '--all' ]
    then
        shift
        ALL=1
    fi

    # Allow opt-out commits:
    if [ "$#" -gt 0 ] && grep -q '^\[SKIP-TESTS\]' "$1"
    then
        exit 0
    fi

    run-phase build
    run-phase test
    run-phase doc

    exit-report
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
        [ "$ALL" -eq 1 ] || exit-report
    fi
    echo
}


function exit-report
{
    if [ "$FAILURECOUNT" -eq 0 ]
    then
        echo '... No failures.'
    else
        echo "... $FAILURECOUNT failures."
    fi

    exit "$FAILURECOUNT"
}


main "$@"
