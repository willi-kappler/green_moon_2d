#!/usr/bin/env bash

CLEAN_BUILD=0
RUN_TEST=0
SHOW_LOG=0
export NIX_ENFORCE_NO_NATIVE=0

for flag in "$@"
do
    case $flag in
        --clean)
            CLEAN_BUILD=1
        ;;
        --test)
            RUN_TEST=1
        ;;
        --log)
            SHOW_LOG=1
        ;;
        *)
            echo "Usage:"
            echo "Clean build: $0 --clean"
            echo "Run all test cases: $0 --test"
            echo "Show log after tests: $0 --test --log"
            exit 1
        ;;
    esac
done


if [[ $CLEAN_BUILD -eq 1 ]]; then
    echo "Clean build!"
    rm -f build_compile.log
    rm -f build_setup.log
    rm -rf builddir
    meson setup builddir > build_setup.log
fi

if [ ! -d "builddir" ]; then
  meson setup builddir > build_setup.log
fi

meson compile -C builddir > build_compile.log

if [[ $RUN_TEST -eq 1 ]]; then
    echo "Run test cases!"
    meson test -C builddir --v

    if [[ $SHOW_LOG -eq 1 ]]; then
        less builddir/meson-logs/testlog.txt
    fi
fi
