#!/bin/bash

check() {
    pushd "${1}" || return 1
    rm -rf Cargo.lock target
    # cargo clean
    cargo update
    cargo c --lib --examples
    local c=$?
    popd
    return $c
}

check src/code012 && check src/code011 && check src/code
