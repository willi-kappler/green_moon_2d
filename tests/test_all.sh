#!/usr/bin/env bash

# In order to run only one test use:
# tests/test_all.sh test_animation.py

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
    echo "##### New test file: $1 #####"
    python3 tests/$1
fi
