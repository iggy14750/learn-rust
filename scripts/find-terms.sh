#!/bin/bash

# Search through Rust SOURCE CODE,
# finding all instances of the following terms:
# - unwrap
# - unsafe
# - expect
# printing out the filename and line number
# on which the term was found.
#
# These terms are a potential code smell.
# I am tempted to call them "banned",
# but I do not believe such a firm stance is helpful.
# They are to be futher reviewed when they are found.
# The following are valid situations to use these terms.
# - "unsafe" may be used in circumstances in which there
#   is NO OTHER OPTION. One such example is dealing with
#   memory-mapped peripherals in embedded systems.
# - "expect" may be used in limited contexts in which
#   this failure (Err or None) means that your program
#   cannot perform its funtion, such as a failure to
#   read in essential data. If this is encountered,
#   expect must provide a user-relevant error message.
# I personally cannot think of a valid use case for "unwrap".

TERMS=(\
    "unwrap" \
    "expect" \
    "unsafe" \
)

src_files=$(find . -type f | grep --invert-match 'target' | grep 'rs$')

for term in ${TERMS[@]}; do
    echo $src_files | xargs grep --line-number $term
done

