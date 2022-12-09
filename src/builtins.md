# List of Bevy Builtins

{{#include ./include/links.md}}

This page is a quick condensed listing of all the important things provided
by Bevy.

 - [SystemParams](#systemparams)
 - [Assets](#assets)
 - [File Formats](#file-formats)
 - [`wgpu` Backends](#wgpu-backends)
 - [Bundles](#bundles)
 - [Resources (Configuration)](#configuration-resources)
 - [Resources (Engine User)](#engine-resources)
   - [Main World](#engine-resources)
   - [Render World](#render-world-resources)
   - [Low-Level `wgpu` access](#low-level-wgpu-resources)
 - [Resources (Input)](#input-handling-resources)
 - [Events (Input)](#input-events)
 - [Events (Engine)](#engine-events)
 - [Events (System/Control)](#system-and-control-events)
 - [Components](#components)
 - [GLTF Asset Labels](#gltf-asset-labels)
 - [Stages](#stages)

## SystemParams

These are all the special types that can be used as [system][cb::system] parameters.

{{#include ./include/builtins.md:systemparams}}

## Assets

[(more info about working with assets)][cb::asset]

These are the Asset types registered by Bevy by default.

{{#include ./include/builtins.md:assets}}

## File Formats

These are the asset file formats (asset loaders) supported by Bevy. Support
for each one can be enabled/disabled using [cargo features][cb::features]. Some
are enabled by default, many are not.

{{#include ./include/builtins.md:file-formats}}

There are [unofficial plugins][cb::3rdparty::file-formats] available for adding
support for even more file formats.

## `wgpu` Backends

{{#include ./include/builtins.md:wgpu-backends}}

## Bundles

Bevy's built-in [bundle][cb::bundle] types, for spawning different common
kinds of entities.

{{#include ./include/builtins.md:bundles}}

## Resources

[(more info about working with resources)][cb::res]

### Configuration Resources

These resources allow you to change the settings for how various parts of Bevy work.

These may be inserted at the start, but should also be fine to change at runtime (from a
[system][cb::system]):

{{#include ./include/builtins.md:resources-config}}

Settings that are not modifiable at runtime are not represented using resources. Instead,
they are configured via the respective [plugins](#plugins).

In Bevy 0.9, there is an exception to this rule:

{{#include ./include/builtins.md:resources-config-init}}

These settings must be inserted as a resource to the [app][cb::app], at the top,
before adding `DefaultPlugins`.

This API inconsistency will be addressed in future versions of Bevy. These
settings will be configurable using the plugin, instead of a resource, just like
other settings that are only used during engine startup.

### Engine Resources

These resources provide access to different features of the game engine at runtime.

Access them from your [systems][cb::system], if you need their state, or to control the respective
parts of Bevy. These resources are in the [Main World][cb::render-architecture]. [See here for the
resources in the Render World](#render-world).

{{#include ./include/builtins.md:resources-main}}

#### Render World Resources

These resources are present in the [Render World][cb::render-architecture]. They can be accessed
from rendering systems (that run during [render stages][cb::render::stage]).

{{#include ./include/builtins.md:resources-render}}

There are many other resources in the Render World, which are not mentioned
here, either because they are internal to Bevy's rendering algorithms, or
because they are just extracted copies of the equivalent resources in the Main
World.

#### Low-Level `wgpu` Resources

Using these resources, you can have direct access to the `wgpu` APIs for controlling the GPU.
These are available in both the Main World and the Render World.

{{#include ./include/builtins.md:resources-wgpu}}

### Input Handling Resources

These resources represent the current state of different input devices. Read them from your
[systems][cb::system] to [handle user input][chapter::input].

{{#include ./include/builtins.md:resources-input}}

## Events

[(more info about working with events)][cb::event]

### Input Events

These [events][cb::event] fire on activity with input devices. Read them to [handle user input][cb::input].

{{#include ./include/builtins.md:events-input}}

### Engine Events

[Events][cb::event] related to various internal things happening during the
normal runtime of a Bevy app.

{{#include ./include/builtins.md:events-engine}}

### System and Control Events

Events from the OS / windowing system, or to control Bevy.

{{#include ./include/builtins.md:events-system}}

## Components

The complete list of individual component types is too specific to be useful to list here.

See: [(List in API Docs)][bevy::impl::Component]

Curated/opinionated list of the most important built-in component types:

{{#include ./include/builtins.md:components}}

## GLTF Asset Labels

[Asset path labels to refer to GLTF sub-assets.][cb::gltf-asset-path]

{{#include ./include/builtins.md:gltf-asset-labels}}

## Stages

{{#include ./include/builtins.md:stages}}

The [Render Stages][cb::render::stage] are each intended for a specific purpose:

{{#include ./include/builtins.md:render-stages}}
