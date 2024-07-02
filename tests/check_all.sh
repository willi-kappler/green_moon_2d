#!/usr/bin/env bash

reset

export PYTHONPATH=$PYTHONPATH:"src/"

echo "ruff:"
ruff check src/green_moon_2d/
ruff check tests/

echo -e "\n\n-------------------------------------------\n\n"

echo "mypy:"
mypy --check-untyped-defs src/green_moon_2d/
mypy --check-untyped-defs tests/

echo -e "\n\n-------------------------------------------\n\n"

echo "flake8:"
flake8 src/green_moon_2d/
flake8 tests/

echo -e "\n\n-------------------------------------------\n\n"

echo "pyright:"
pyright src/green_moon_2d/
pyright tests/
