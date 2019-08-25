#!/bin/bash
CARGO_INCREMENTAL=0 cargo build --release
valgrind --tool=memcheck --leak-check=full --show-reachable=yes --show-possibly-lost=yes --log-file=memcheck.out.1 target/release/memory_allocator
cat memcheck.out.1
