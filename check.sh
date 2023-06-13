#!/bin/sh

check() {
    pushd "${1}" || return 1
    rm -rf Cargo.lock target
    cargo clean
    cargo update
    cargo c --lib --examples
    local c=$?
    popd
    return $c
}

check src/code && check src/code010 && check src/code011
