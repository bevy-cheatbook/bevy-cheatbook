## GPU Drivers

On Linux, Bevy currently requires Vulkan for graphics.

On Windows, either Vulkan or DirectX 12 can be used.

Make sure you have compatible hardware and drivers installed on your system.
Your users will also need to satisfy this requirement.

If Bevy is not working, install the latest drivers, or check with your Linux
distribution whether Vulkan needs additional packages to be installed.

OpenGL should work as a fallback, for systems that do not support other APIs,
but might not be in such a good state as other APIs. DirectX 11 support for
legacy systems is planned, but not available yet.

macOS/iOS should work without any special driver setup, using Metal.

Web games are supported and should work in any modern browser, using WebGL2.
