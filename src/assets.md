Warning: this chapter has not been fully updated for Bevy 0.10 yet!

# Bevy Asset Management

{{#include ./include/links.md}}

Assets are the data that the game engine is working with: all of your images,
3D models, sounds, scenes, game-specific things like item descriptions,
and more!

Bevy has a flexible system for loading and managing your game assets
asynchronously (in the background, without causing lag spikes in your game).

In your code, you refer to individual assets using [handles][cb::handle].

Asset data can be [loaded from files][cb::assetserver] and also [accessed from
code][cb::asset-data]. [Hot-reloading][cb::asset-hotreload] is supported to
help you during development, by reloading asset files if they change while the
game is running.

If you want to write some code to do something when assets finish loading, get
modified, or are unloaded, you can use [asset events][cb::assetevent].
