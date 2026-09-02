#!/usr/bin/env bash

rm -rf builddir
meson setup builddir
meson compile -C builddir
meson test -C builddir --v

