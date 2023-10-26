{{#include ../include/header012.md}}

# Linux Desktop

If you have any additional Linux-specific knowledge,
please help improve this page!

Create Issues or PRs on [GitHub][project::cb].

---

Desktop Linux is one of the best-supported platforms by Bevy.

There are some development dependencies you may need to setup, depending on your
distribution. [See instructions in official Bevy repo.][bevy::linux-dependencies]

[See here if you also want to build Windows EXEs from Linux][cross::linux-windows].

## GPU Drivers

Bevy apps need support for the Vulkan graphics API to run best. There is a
fallback on OpenGL ES 3 for systems where Vulkan is unsupported, but it might not
work and will have limited features and performance.

You (and your users) must ensure that you have compatible hardware and drivers
installed. On most modern distributions and computers, this should be no problem.

If Bevy apps refuse to run and print an error to the console about not being
able to find a compatible GPU, the problem is most likely with the Vulkan
components of your graphics driver not being installed correctly. You may
need to install some extra packages or reinstall your graphics drivers. Check
with your Linux distribution for what to do.

To confirm that Vulkan is working, you can try to run this command (found in
a package called `vulkan-tools` on most distributions):

```sh
vulkaninfo
```

## X11 and Wayland

As of the year 2023, the Linux desktop ecosystem is fragmented between
the legacy X11 stack and the modern Wayland stack. Many distributions are
switching to Wayland-based desktop environments by default.

Bevy supports both, but only X11 support is enabled by default. If you are
running a Wayland-based desktop, this means your Bevy app will run in the
XWayland compatibility layer.

To enable native Wayland support for Bevy, enable the `wayland` cargo feature:

```toml
[dependencies]
bevy = { version = "0.12", features = ["wayland"] }
```

Now your app will be built with support for both X11 and Wayland.

If you want to remove X11 support for whatever reason, you will have to disable
the default features and re-enable everything you need, without the `x11`
feature. [See here to learn how to configure Bevy features.][cb::features]

If both are enabled, you can override which display protocol to use at runtime,
using an environment variable:

```shell
export WINIT_UNIX_BACKEND=x11
```

(to run using X11/XWayland on a Wayland desktop)

or

```shell
export WINIT_UNIX_BACKEND=wayland
```

(to require the use of Wayland)
