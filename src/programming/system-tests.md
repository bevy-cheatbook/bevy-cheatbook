# Writing Tests for Systems

You might want to write and run automated tests for your [systems](./systems.md).

You can use the regular Rust testing features (`cargo test`) with Bevy.

To do this, you can create an ECS World in your tests, and then, using
[direct World access](./world-exclusive.md), insert whatever entities and
resources you need for testing. Create a standalone [stage](./stages.md)
with the systems you want to run, and manually run it on the World.

Bevy's official repository has a fantastic [example of how to do
this](https://github.com/bevyengine/bevy/blob/main/tests/how_to_test_systems.rs).
