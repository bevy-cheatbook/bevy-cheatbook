#!/bin/bash

check() {
    pushd "${1}" || return 1
    rm -rf Cargo.lock target
    # cargo clean
    cargo update
    cargo c --lib --examples
    local c=$?
    rm -rf Cargo.lock target
    popd
    return $c
}

check src/code015
