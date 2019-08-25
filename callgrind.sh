#!/bin/bash
rm callgrind.out.*
CARGO_INCREMENTAL=0 cargo build --release
valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes target/release/memory_allocator
kcachegrind
