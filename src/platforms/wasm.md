# Browser (WebAssembly)

(big thanks to **@Zaszi** for providing most of the information and writing [the
draft for this chapter](https://github.com/bevy-cheatbook/bevy-cheatbook/pull/23))

---

## Introduction

You can make web browser games using Bevy. This chapter will help you with the
things you need to know to do it. This page gives an overview of Bevy's Web support.

Your Bevy app will be compiled for WebAssembly (WASM), which allows it to be
embedded in a web page and run inside the browser.

Performance will be limited, as WebAssembly is slower than native code and does
not support multithreading.

Most Bevy features work fine. A few things need 3rd-party replacements. Some
additional configuration is required.

Not all 3rd-party plugins are compatible. If you need extra unofficial plugins,
you will have to check if they are compatible with WASM.

## Project Setup

You can configure your project so that it can be compiled for both web and desktop.

For WASM to work, it needs to be built in a special way, with accompanying
JavaScript code for interfacing with the browser. This can be handled and
auto-generated using some extra tools alongside `cargo`.

This chapter demonstrates two alternative ways for how to set up your project:
[using `wasm-pack`](./wasm/wasm-pack.md) or [using Cargo Make](./wasm/cargo-make.md).
You can choose whichever way you like better (or come up with your own).

You will need a website with some HTML and JavaScript to load and run your game.
This could be just a minimal shim. It is shown as part of the setup guides.

To deploy, you will need a server to host your website for other people to
access. You could use GitHub's hosting service: [GitHub Pages](./wasm/gh-pages.md).

When users load your site to play your game, their browser will need to download
the files. [Optimizing for size](./wasm/size-opt.md) is important, so that your
game can load fast and not waste data bandwidth.

## Rendering

Bevy's official rendering is based on WebGPU. This is a new, modern graphics API,
similar to Metal and Vulkan, but designed for browsers and to be cross-platform.
Ironically, WebGPU is not yet supported in web browsers, only for native apps.

This means that the official `bevy_wgpu` crate needs to be replaced.
[`bevy_webgl2`](https://github.com/mrk-its/bevy_webgl2) is an unofficial drop-in
replacement that works in current browsers, using WebGL2.

## Audio

Bevy's built-in audio does not work in the web browser. However, it is very
limited in functionality anyway; it is not very useful for desktop games either.

Instead, you could use the unofficial
[`bevy_kira_audio`](https://github.com/NiklasEi/bevy_kira_audio) plugin, which
is an integration with the [Kira](https://github.com/tesselode/kira) sound
engine. It is a more feature-rich replacement for Bevy's audio. It works both on
the web and the desktop platforms.

## Input Devices

Gamepads/joysticks/controllers are not supported in Web browsers.

## Additional Caveats

Some minor extra configuration is needed to be able to:
 - [See Rust panic messages](./wasm/panic-console.md)
 - [Generate random numbers](./wasm/rng.md)
