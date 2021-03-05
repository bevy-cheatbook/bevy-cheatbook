# Optimizing for Binary Size

When serving a WASM binary, the smaller it is, the faster the
browser can download it. Faster downloads means faster page load times,
and that means happier users.

However, binary size is not the only metric which you should care about, and
some optimizations that reduce binary size come at the cost of execution
speed or lack of debug information. Depending on the nature of your
application, your mileage may vary, and performing measurements of binary size
and execution speed is recommended.

[Twiggy](https://github.com/rustwasm/twiggy) is a code size profiler for WASM
binaries which you can use.

For additional information and more techniques, refer to the Code Size chapter in the [Rust WASM book](https://rustwasm.github.io/docs/book/reference/code-size.html).

## Compiling with Link Time Optimizations (LTO)

In `Cargo.toml`, add the following:

```toml
[profile.release]
lto = true
```

This tells LLVM to inline and prune functions much more aggressively, resulting
in a smaller binary size _and_ an execution speed increase. The downside here
is that compilation will take longer.

## Compiling for size instead of speed

In `Cargo.toml`, add the following:

```toml
[profile.release]
opt-level = 's'
```

Or, an even more aggressive variant:

```toml
[profile.release]
opt-level = 'z'
```

Surprisingly, `opt-level = "s"` can sometimes result in smaller binaries than `opt-level = "z"`. As mentioned before, always make sure to take measurements.

## Use the `wasm-opt` tool

The [binaryen](https://github.com/WebAssembly/binaryen) toolkit is a set of
WASM-specific compiler tools, amongst which `wasm-opt`. It goes much further
than what the LLVM compiler can do and can be used to further optimize for
either speed or size:

```shell
# Optimize for size.
wasm-opt -Os -o output.wasm input.wasm

# Optimize aggressively for size.
wasm-opt -Oz -o output.wasm input.wasm

# Optimize for speed.
wasm-opt -O -o output.wasm input.wasm

# Optimize aggressively for speed.
wasm-opt -O3 -o output.wasm input.wasm
```

Note: `wasm-pack` performs this partially by default.

## Use the `wee-alloc` memory allocator

By default, Rust uses a port of `dlmalloc` for WebAssembly, which is about 10
kilobytes in size. If you don't need particularly fast memory allocator, you
can use [wee-alloc](https://github.com/rustwasm/wee_alloc) instead, which is less than a single kilobyte in size.

In `Cargo.toml`, add the following:

```toml
[dependencies]
wee_alloc = "0.4"
```

And now add the following to `main.rs`:

```rust
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```
