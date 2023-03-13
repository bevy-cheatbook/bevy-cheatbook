Warning: this page has not been updated for Bevy 0.10 yet!

# Handles

{{#include ../include/links.md}}

Handles are lightweight IDs that refer to a specific asset. You need them
to use your assets, for example to [spawn entities][cb::commands] like
[2D sprites][cb::sprite] or [3D models][cb::gltf], or to [access the data
of the assets][cb::asset-data].

Handles have the Rust type [`Handle<T>`][bevy::Handle], where `T` is the
[asset type][builtins::asset].

You can store handles in your [entity components][cb::component] or
[resources][cb::res].

Handles can refer to not-yet-loaded assets, meaning you can just spawn your
entities anyway, using the handles, and the assets will just "pop in" when
they become ready.

## Obtaining Handles

If you are [loading an asset from a file][cb::assetserver], the
`asset_server.load(…)` call will give you the handle. The loading of the
data happens in the background, meaning that the handle will initially refer
to an unavailable asset, and the actual data will become available later.

If you are [creating your own asset data from code][cb::asset-data::add],
the `assets.add(…)` call will give you the handle.

## Reference Counting; Strong and Weak Handles

Bevy keeps track of how many handles to a given asset exist at any time. Bevy
will automatically unload unused assets, after the last handle is dropped.

For this reason, creating additional handles to the same asset requires you
to call `handle.clone()`. This makes the operation explicit, to ensure you are
aware of all the places in your code where you create additional handles. The
`.clone()` operation is cheap, so don't worry about performance (in most cases).

There are two kinds of handles: "strong" and "weak". Strong assets are
counted, weak handles are not. By default, handles are strong. If you want
to create a weak handle, use `.clone_weak()` (instead of `.clone()`) on an
existing handle. Bevy can unload the asset after all strong handles are gone,
even if you are still holding some weak handles.

## Untyped Handles

Bevy also has a [`HandleUntyped`][bevy::HandleUntyped] type. Use this type
of handle if you need to be able to refer to any asset, regardless of the
asset type.

This allows you to store a collection (such as [`Vec`][std::Vec] or
[`HashMap`][std::HashMap]) containing assets of mixed types.

You can create an untyped handle using `.clone_untyped()` on an existing
handle.

Just like regular handles, untyped handles can be strong or weak.

You need to do this to [access the asset data][cb::asset-data].

You can convert an untyped handle into a typed handle with `.typed::<T>()`,
specifying the type to use. You need to do this to [access the asset
data][cb::asset-data].
