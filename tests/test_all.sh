#!/usr/bin/env bash

reset

export PYTHONPATH=$PYTHONPATH:"src/"

for t in tests/*.py; do
    echo $t
    python3 $t
    echo -e "###########################################################\n"
done
