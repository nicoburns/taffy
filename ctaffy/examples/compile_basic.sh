#!/bin/sh

set -ex

(cd .. && cbindgen --crate ctaffy --output include/taffy.h)
cargo build --manifest-path ../Cargo.toml
gcc -O3 -DDEBUG -o basic basic.c -std=c99 -Wall -I../include -L../../target/debug -lctaffy
