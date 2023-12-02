{{#include ../../include/header012.md}}

# Optimize for Size

When serving a WASM binary, the smaller it is, the faster the browser can
download it. Faster downloads means faster page load times and less data
bandwidth use, and that means happier users and happier server hosts. ;)

This page gives some suggestions for how to make your WASM files smaller for
deployment / release builds. You probably don't need small WASM files during
development, and many of these techniques can get in the way of your workflow!
They come at the cost of longer compile times and less debuggability.

Depending on the nature of your application, your mileage may vary, and
performing measurements of binary size and execution speed is recommended.

[Twiggy][project::twiggy] is a code size profiler for WASM binaries, which
you can use to make measurements.

For additional information and more techniques, refer to the Code Size
chapter in the [Rust WASM book][rustwasmbook::code-size].

Do you know of more WASM size-optimization techniques? Post about them in the
[GitHub Issue Tracker][project::cb] so that they can be added to this page!

## Compiling for size instead of speed

You can change the optimization profile of the compiler, to tell it to
prioritize small output size, rather than performance.

(although in some rare cases, optimizing for size can actually improve speed)

In `Cargo.toml`, add one of the following:

```toml
[profile.release]
opt-level = 'z'
```

```toml
[profile.release]
opt-level = 's'
```

These are two different profiles for size optimization. Usually, `z` produces
smaller files than `s`, but sometimes it can be the opposite. Measure to
confirm which one works better for you.

## Link-Time Optimization (LTO)

In `Cargo.toml`, add one of the following:

For some big improvements with moderate slowdown to compile times:

```toml
[profile.release]
lto = "thin"
```

For the biggest improvements at the cost of the slowest compile times:

```toml
[profile.release]
lto = true
codegen-units = 1
```

LTO tells the compiler to optimize all code together, considering all crates as
if they were one. It may be able to inline and prune functions much more
aggressively. This typically results in smaller size *and* better performance,
but do measure to confirm. Sometimes, the size can actually be larger.

## Use the `wasm-opt` tool

The [binaryen][project::binaryen] toolkit is a set of extra tools for working
with WASM. One of them is `wasm-opt`. It goes much further than what the
compiler can do, and can be used to further optimize for either speed or size:

```shell
# Optimize for size (z profile).
wasm-opt -Oz -o output.wasm input.wasm

# Optimize for size (s profile).
wasm-opt -Os -o output.wasm input.wasm

# Optimize for speed.
wasm-opt -O3 -o output.wasm input.wasm

# Optimize for both size and speed.
wasm-opt -O -ol 100 -s 100 -o output.wasm input.wasm
```

You should run this command on the final WASM file you deploy to your website,
after `wasm-bindgen` or other tools. If you run it before, `wasm-bindgen` can
get confused and panic.
