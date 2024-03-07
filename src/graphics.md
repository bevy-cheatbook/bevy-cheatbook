{{#include ./include/header-none.md}}

# General Graphics Features

This chapter covers general graphics-related features in Bevy, that are
relevant to both 2D and 3D games.

Bevy's rendering is driven by / configured via [cameras][cb::camera]. Each
camera [entity][cb::entity] will cause Bevy to render your game world,
as configured via the various [components][cb::component] on the
[camera][cb::camera]. You can enable all kinds of different workflows, as
well as optional effects, by adding the relevant components to your camera
and configuring them.
