#!/usr/bin/env bash

reset

export PYTHONPATH=$PYTHONPATH:"../src/"

FOLDERS=("text1a" "text1b")

for f in ${FOLDERS[@]}; do
    echo "---------- New folder: $f"
    ruff check $f
    mypy --check-untyped-defs $f
    flake8 $f
    pyright $f
    echo -e "\n\n-------------------------------------------\n\n"
done
