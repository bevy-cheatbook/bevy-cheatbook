# Writing Tests for Systems

{{#include ../include/links.md}}

You might want to write and run automated tests for your [systems][cb::system].

You can use the regular Rust testing features (`cargo test`) with Bevy.

To do this, you can create an empty ECS [`World`][bevy::World] in your
tests, and then, using [direct World access][cb::world], insert whatever
[entities][cb::entity] and [resources][cb::res] you need for testing. Create
a standalone [stage][cb::stage] with the [systems][cb::system] you want to
run, and manually run it on the [`World`][bevy::World].

Bevy's official repository has a fantastic [example of how to do
this][bevy::system-test].
