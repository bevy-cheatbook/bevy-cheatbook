{{#include ../include/header010.md}}

# New to Bevy? Guided Tutorial!

Welcome to Bevy! :) We are glad to have you in our community!

This page will guide you through this book, to help you gain comprehensive
knowledge of how to work with Bevy. The topics are structured in an order
that makes sense for learning: from basics to advanced.

It is just a suggestion to help you navigate. Feel free to jump around the book
and read whatever interests you. The main table-of-contents (the left sidebar)
was designed to be a reference for Bevy users of any skill level.

---

Make sure to also look at [the official Bevy examples][bevy::examples]. If
you need help, use [GitHub Discussions][bevy::ghdiscussions], or feel welcome
to join us to chat and ask for help in [Discord][bevy::discord].

If you run into issues, be sure to check the
[Common Pitfalls][chapter::pitfalls] chapter, to see if this book has something
to help you. Solutions to some of the most common issues that Bevy community
members have encountered are documented there.

## Basics

These are the absolute essentials of using Bevy. Every Bevy project, even a
simple one, would require you to be familiar with these concepts.

You could conceivably make something like a simple game-jam game or prototype,
using just this knowledge. Though, as your project grows, you will likely
quickly need to learn more.

 - [Bevy Setup Tips][chapter::setup]:
   Configuring your development tools and environment
   - [Getting Started][cb::getting-started]
 - [Bevy Programming Framework][chapter::programming]:
   How to write Bevy code, structure your data and logic
   - [Intro to ECS][cb::ecs-intro]
   - [Entities, Components, Bundles][cb::ec]
   - [Resources][cb::res]
   - [Systems][cb::system]
   - [App Builder][cb::app]
   - [Queries][cb::query]
   - [Commands][cb::commands]
   - [Events][cb::event]
 - [General Game Engine Features][chapter::features]:
   Basic features of Bevy, needed for making any game
   - [Coordinate System][cb::coords]
   - [Transforms][cb::transform]
   - [Time and Timers][cb::time]
   - [Parent/Child Hierarchies][cb::hierarchy]
 - [Bevy Asset Management][chapter::assets]:
   How to work with assets
   - [Handles][cb::handle]
   - [Load Assets with AssetServer][cb::assetserver]
 - [Input Handling][chapter::input]:
   Using various input devices
   - [Keyboard][input::keyboard]
   - [Mouse][input::mouse]
   - [Gamepad (Controller)][input::gamepad]
   - [Touchscreen][input::touch]
 - [Window Management][chapter::window]:
   Setting up the OS Window (or fullscreen) for your game
   - [Window Properties][cb::window]
   - [Change the Background Color][cb::clearcolor]

## Next Steps

You will likely need to learn most of these topics to make a non-trivial Bevy
project. After you are confident with the basics, you should learn these.

 - [Bevy Programming Framework][chapter::programming]
   - [System Order of Execution][cb::system-order]
   - [Run Conditions][cb::rc]
   - [System Sets][cb::systemset]
   - [Local Resources][cb::local]
   - [Schedules][cb::schedule]
   - [States][cb::state]
   - [Plugins][cb::plugin]
   - [Change Detection][cb::change-detection]
   - [Removal Detection][cb::removal-detection]
 - [General Game Engine Features][chapter::features]:
   - [Visibility][cb::visibility]
   - [Cameras][cb::camera]
   - [Logging / Console Messages][cb::log]
 - [Input Handling][chapter::input]:
   - [Convert cursor to world coordinates][cookbook::cursor2world]
 - [Bevy Asset Management][chapter::assets]:
   - [Access the Asset Data][cb::asset-data]
   - [React to Changes with Asset Events][cb::assetevent]
   - [Track asset loading progress][cb::asset-ready]
   - [Hot-Reloading Assets][cb::asset-hotreload]
 - [Bevy Setup Tips][chapter::setup]
   - [Bevy Dev Tools and Editors][cb::tools]
   - [Community Plugin Ecosystem][cb::3rdparty]

## Intermediate

These are more specialized topics. You may need some of them, depending on your
project.

 - [Bevy Programming Framework][chapter::programming]
   - [Direct World Access][cb::world]
   - [Exclusive Systems][cb::exclusive]
   - [Param Sets][cb::paramset]
   - [System Piping][cb::system-pipe]
 - [General Game Engine Features][chapter::features]:
   - [Fixed Timestep][cb::fixedtimestep]
 - [Input Handling][chapter::input]:
   - [Input Text][input::char]
   - [Drag-and-Drop files][input::dnd]
 - [Programming Patterns][chapter::patterns]
   - [Write tests for systems][cb::system-tests]
   - [Generic Systems][cb::system-generic]
   - [Manual Event Clearing][cb::event-manual]
 - [Window Management][chapter::window]:
   - [Grab/Capture the Mouse Cursor][cookbook::mouse-grab]
   - [Set the Window Icon][cookbook::window-icon]

## Advanced

These topics are for niche technical situations. You can learn them, if you want
to know more about how Bevy works internally, extend the engine with custom
functionality, or do other advanced things with Bevy.

 - [Bevy Programming Framework][chapter::programming]
   - [Non-Send][cb::nonsend]
 - [Programming Patterns][chapter::patterns]
   - [Component Storage][cb::component-storage]
 - [Bevy Setup Tips][chapter::setup]
   - [Customizing Bevy (cargo crates and features)][cb::features]
   - [Using bleeding-edge Bevy (main)][cb::bevy-main]

Tutorials:
 - [GPU: Vertex Pulling][cbtut::vertex-pulling]
