## GPU Drivers

To work at its best, Bevy needs DirectX 12 (Windows) or Vulkan (Linux, Android,
Windows). macOS/iOS should just work, without any special driver setup, using
Metal.

OpenGL (GLES3) can be used as a fallback, but will likely have issues (some
bugs, unsupported features, worse performance).

Make sure you have compatible hardware and drivers installed on your system.
Your users will also need to satisfy this requirement.

If Bevy is not working, install the latest drivers for your OS, or check with
your Linux distribution whether Vulkan needs additional packages to be
installed.

Web games are supported and should work in any modern browser, using WebGL2.
Performance is limited and some Bevy features will not work. The new
experimental high-performance WebGPU API is also supported, but browser adoption
is still limited.
