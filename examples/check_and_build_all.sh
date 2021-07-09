#!/bin/bash

cd bullets1
cargo check
cargo build --release
cd ..

cd menu1
cargo check
cargo build --release
cd ..

cd menu2
cargo check
cargo build --release
cd ..

cd particle1
cargo check
cargo build --release
cd ..

cd scenes1
cargo check
cargo build --release
cd ..

