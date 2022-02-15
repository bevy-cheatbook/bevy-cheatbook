# List of Bevy Builtins

{{#include ./include/links.md}}

This page is a quick condensed listing of all the important things provided
by Bevy.

 - [SystemParams](#systemparams)
 - [Assets](#assets)
 - [Bundles](#bundles)
 - [Resources (Configuration)](#configuration-resources)
 - [Resources (Engine User)](#engine-resources)
 - [Resources (Input)](#input-handling-resources)
 - [Events (Input)](#input-events)
 - [Events (System/Control)](#system-and-control-events)
 - [Components](#components)
 - [GLTF Asset Labels](#gltf-asset-labels)
 - [Stages](#stages)

## SystemParams

These are all the special types that can be used as [system][cb::system] parameters.

{{#include ./include/builtins.md:systemparams}}

## Assets

These are the Asset types registered by Bevy by default.

{{#include ./include/builtins.md:assets}}

## Bundles

Bevy's built-in [bundle][cb::bundle] types, for spawning different common
kinds of entities.

{{#include ./include/builtins.md:bundles}}

## Resources

### Configuration Resources

These resources allow you to change the settings for how various parts of Bevy work.

Some of them affect the low-level initialization of the engine, so must be present from the
start to take effect. You need to insert these at the start of your [app builder][cb::app]:

{{#include ./include/builtins.md:resources-config-init}}

These may be inserted at the start, but should also be fine to change at runtime (from a
[system][cb::system]):

{{#include ./include/builtins.md:resources-config}}

### Engine Resources

These resources provide access to different features of the game engine at runtime.

Access them from your [systems][cb::system], if you need their state, or to control the respective
parts of Bevy.

{{#include ./include/builtins.md:resources-engine}}

### Input Handling Resources

These resources represent the current state of different input devices. Read them from your
[systems][cb::system] to [handle user input][cb::input].

{{#include ./include/builtins.md:resources-input}}

## Events

### Input Events

These [events][cb::event] fire on activity with input devices. Read them to [handle user input][cb::input].

{{#include ./include/builtins.md:events-input}}

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

