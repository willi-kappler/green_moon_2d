#!/usr/bin/env bash

reset

echo "ruff:"
ruff check src/green_moon_2d/

echo -e "\n\n-------------------------------------------\n\n"

echo "mypy:"
mypy src/green_moon_2d/

echo -e "\n\n-------------------------------------------\n\n"

echo "flake8:"
flake8 src/green_moon_2d/

# end
