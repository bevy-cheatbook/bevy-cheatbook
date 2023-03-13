Warning: this page has not been updated for Bevy 0.10 yet!

# New to Bevy? Guided Tutorial!

{{#include ./include/links.md}}

Welcome to Bevy! :) We are glad to have you in our community!

Make sure to also look at [the official Bevy examples][bevy::examples]. If
you need help, use [GitHub Discussions][bevy::ghdiscussions], or feel welcome
to join us to chat and ask for help in [Discord][bevy::discord].

---

This page is intended for new learners. It will guide you through this book in
an order that makes sense for learning: from the basics, towards more advanced
topics. This is unlike the main table-of-contents (the left sidebar), which
was designed to be a reference for Bevy users of any skill level.

This tutorial page does not list/link every page in the book. It is a guide
to help you gain comprehensive general knowledge. The book also has many
pages dedicated to solutions for specific problems; those are not listed here.

Feel free to jump around the book and read whatever interests you.

You will be making something cool with Bevy in no time! ;)

---

If you run into issues, be sure to check the
[Common Pitfalls][chapter::pitfalls] chapter, to see if this book has something
to help you. Solutions to some of the most common issues that Bevy community
members have encountered are documented there.

## Basics

These are the absolute essentials of using Bevy – the minimum concepts to
get you started. Every Bevy project, even a simple one, would require you
to be familiar with these concepts.

You could conceivably make something like a simple game-jam game or prototype,
using just this knowledge. Though, as your project grows, you will likely
quickly need to learn more.

 - [Bevy Setup Tips][chapter::setup]:
   Configuring your development tools and environment
   - [Getting Started][cb::getting-started]
 - [Bevy Programming Framework][chapter::programming]:
   How to write Bevy code, structure your data and logic
   - [Intro to ECS][cb::ecs-intro]
   - [Entities and Components][cb::ec]
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
 - [Window Management][chapter::window]:
   Setting up the OS Window (or fullscreen) for your game
   - [Change the Background Color][cb::clearcolor]

## Next Steps

You will likely need to learn about at least some of these topics to make
a non-trivial Bevy project. After you are confident with the basics, you
can familiarize yourself with these, to become a proficient Bevy user.

 - [Bevy Setup Tips][chapter::setup]
   - [Bevy Dev Tools and Editors][cb::tools]
   - [Community Plugin Ecosystem][cb::3rdparty]
 - [Bevy Programming Framework][chapter::programming]
   - [System Order of Execution][cb::system-order]
   - [System Sets][cb::systemset]
   - [Local Resources][cb::local]
   - [Plugins][cb::plugin]
   - [Labels][cb::label]
   - [States][cb::state]
   - [Stages][cb::stage]
   - [Change Detection][cb::change-detection]
   - [Removal Detection][cb::removal-detection]
   - [Param Sets][cb::paramset]
 - [Programming Patterns][chapter::patterns]
   - [Generic Systems][cb::system-generic]
   - [Component Storage][cb::component-storage]
 - [Bevy Asset Management][chapter::assets]:
   - [Access the Asset Data][cb::asset-data]
   - [React to Changes with Asset Events][cb::assetevent]
   - [Hot-Reloading Assets][cb::asset-hotreload]

## Advanced

These are more specialized topics, may be useful in complex projects. Most
typical Bevy users are unlikely to need to know these.

 - [Bevy Programming Framework][chapter::programming]
   - [Run Criteria][cb::runcriteria]
   - [System Piping][cb::system-pipe]
   - [Direct World Access][cb::world]
   - [Exclusive Systems][cb::exclusive]
   - [Non-Send][cb::nonsend]
 - [Programming Patterns][chapter::patterns]
   - [Manual Event Clearing][cb::event-manual]

## Solutions to Specific Problems

These are pages that teach you solutions to specific tasks that you might
encounter in your project.

 - [Convert cursor to world coordinates][cookbook::cursor2world]
 - [Write tests for systems][cb::system-tests]
 - [Track asset loading progress][cb::asset-ready]
 - [Grab/Capture the Mouse Cursor][cookbook::mouse-grab]
 - [Set the Window Icon][cookbook::window-icon]
 - [Input Text][input::char]
 - [Drag-and-Drop files][input::dnd]
 - [Custom Camera Projection][cookbook::cursor2world]
