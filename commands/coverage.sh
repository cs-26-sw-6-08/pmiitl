#!/bin/bash
cargo llvm-cov --ignore-filename-regex 'src/grammar/.*' --open
