#!/bin/sh

prefix=/Users/franky_mac/Rust_Programming/halo2-graph/target/debug/build/jemalloc-sys-8a9f0716b025e8a9/out
exec_prefix=/Users/franky_mac/Rust_Programming/halo2-graph/target/debug/build/jemalloc-sys-8a9f0716b025e8a9/out
libdir=${exec_prefix}/lib

DYLD_INSERT_LIBRARIES=${libdir}/libjemalloc.2.dylib
export DYLD_INSERT_LIBRARIES
exec "$@"
