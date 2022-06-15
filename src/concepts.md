# List of Bevy Concepts

{{#include ./include/links.md}}

This page offers a quick condensed listing of core bevy concepts.


### ECS

[Entities][cb::ec]: Entities are just a simple integer ID, that identifies a particular set of component values. Queries let you access components of entities.

[Components][cb::component]: Components are the data associated with entities.

[Systems][cb::system]: Systems are functions you write, which are run by Bevy.

### Data

[Bundle][cb::bundle]: Bundles are like "templates", to make it easy to create entities with a common set of components.

[Resources][cb::res]: Resources allow you to store a single global instance of some data type, independently of entities. Any Rust type (struct or enum) can be used as a resource.

[Local Resources][cb::local]: Local resources allow you to have per-system data.

[Non-Send Resources][cb::nonsend]: "Non-send" refers to data types that must only be accessed from the "main thread" of the application. Such data is marked by Rust as !Send (lacking the Send trait).

### Communication

[Commands][cb::commands]: Use Commands to spawn/despawn entities, add/remove components on existing entities, manage resources.

[Events][cb::event]: Send data between systems! Let your systems communicate with each other!

[Change Detection][cb::change-detection]: Bevy allows you to easily detect when data is changed. You can use this to perform actions in response to changes.

### Groups of functions

[Plugins][cb::plugin]: Plugins are simply collections of things to be added to the App Builder.

[System Sets][cb::systemset]: System Sets allow you to easily apply common properties to multiple systems, for purposes such as labeling, ordering, run criteria, and states.

[Exclusive Systems][cb::exclusive]: Exclusive systems are systems that Bevy will not run in parallel with any other system. They can have full unrestricted access to the whole ECS World, by taking a &mut World parameter. Note that exclusive systems can limit performance, as they prevent multi-threading (nothing else runs at the same time).

### Flow

[States][cb::state]: States allow you to structure the runtime "flow" of your app. Such as a menu screen or a loading screen, pausing / unpausing the game, different game modes... (States are implemented using run criteria under the hood. )

[Run Criteria][cb::runcriteria]: Run Criteria are a mechanism for controlling if Bevy should run specific systems, at runtime. This is how you can make functionality that only runs under certain conditions.

[Labels][cb::label]: You need labels to name various things in your app, such as systems (for order control), run criteria, stages, and ambiguity sets..

[Stages][cb::stage]: All systems to be run by Bevy are contained in stages. Every frame update, Bevy executes each stage, in order. Within each stage, Bevy's scheduling algorithm can run many systems in parallel, using multiple CPU cores for good performance.

### Assets

[Handles][cb::handle]: Handles are lightweight IDs that refer to a specific asset.