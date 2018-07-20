#!/bin/bash
set -efuo pipefail

SELF="$(readlink -f "$0")"
GITHOOK='.git/hooks/commit-msg'
MSGPATH='/dev/null'
FAILURECOUNT=0
ALL=0



function main
{
    cd "$(dirname "$SELF")"

    init-githook
    parse-arguments "$@"
    check-msg-directives "$MSGPATH"

    touch ./src/main.rs

    run-phase test
    run-phase doc
    run-phase build

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


function parse-arguments
{
    if [ "$#" -gt 0 ] && [ "$1" = '--all' ]
    then
        shift
        ALL=1
    fi

    if [ "$#" -gt 0 ]
    then
        MSGPATH="$1"
        shift
    fi

    if [ "$#" -gt 0 ]
    then
        echo 'Too many arguments.'
        exit -1
    fi

}


function check-msg-directives
{
    for word in $(sed 's/^[^\[].*$//; s/^\[SKIP //; s/\].*$//' "$MSGPATH")
    do
        case $word
        in
            (build|test|doc)
                continue
                ;;

            (*)
                echo "Unknown SKIP directive \"$word\" in $MSGPATH"
                exit -2;
                ;;
        esac
    done
}


function run-phase
{
    local PHASE="$1"

    # Explicit opt-out:
    if grep -q "^\[SKIP [a-z ]*${PHASE}[a-z ]*\]" "$MSGPATH"
    then
        echo -e "[SKIP ${PHASE}]\n"
        return 0
    fi

    echo "=== ${PHASE} phase ==="
    if cargo "${PHASE}"
    then
        echo "--- ${PHASE} phase: pass ---"
    else
        echo "--- ${PHASE} phase: fail ---"
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
