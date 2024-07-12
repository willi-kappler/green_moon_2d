#!/usr/bin/env bash

reset

export PYTHONPATH=$PYTHONPATH:"src/"

if [ "$1" == "" ]; then
    # Run all tests
    for t in tests/*.py; do
        echo "##### New test file: $t #####"
        python3 $t
        echo -e "\n\n"
    done
else
    # Run only one specific test
    python3 tests/$1
fi
