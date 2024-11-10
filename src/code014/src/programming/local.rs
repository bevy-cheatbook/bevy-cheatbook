use bevy::prelude::*;

// ANCHOR: local-resource
#[derive(Default)]
struct MyState {
    // ...
}

fn my_system1(mut local: Local<MyState>) {
    // you can do anything you want with the local here
}

fn my_system2(mut local: Local<MyState>) {
    // the local in this system is a different instance
}
// ANCHOR_END: local-resource

#[allow(dead_code)]
mod localconfig {
use bevy::prelude::*;
#[derive(Resource)]
struct MyStuff;
// ANCHOR: closure
#[derive(Default)]
struct MyConfig {
    magic: usize,
}

fn my_system(
    mut cmd: Commands,
    my_res: Res<MyStuff>,
    // note this isn't a valid system parameter
    config: &MyConfig,
) {
    // TODO: do stuff
}

fn main() {
    let config = MyConfig {
        magic: 420,
    };

    App::new()
        .add_plugins(DefaultPlugins)

        // create a "move closure", so we can use the `config`
        // variable that we created above

        // Note: we specify the regular system parameters we need.
        // The closure needs to be a valid Bevy system.
        .add_systems(Update, move |cmd: Commands, res: Res<MyStuff>| {
            // call our function from inside the closure,
            // passing in the system params + our custom value
            my_system(cmd, res, &config);
        })
        .run();
}
// ANCHOR_END: closure
}

#[allow(dead_code)]
mod localconfig2 {
use bevy::prelude::*;
#[derive(Resource)]
struct MyStuff;
// ANCHOR: constructor
#[derive(Default)]
struct MyConfig {
    magic: usize,
}

// create a "constructor" function, which can initialize
// our data and move it into a closure that Bevy can run as a system
fn my_system_constructor() -> impl FnMut(Commands, Res<MyStuff>) {
    // create the `MyConfig`
    let config = MyConfig {
        magic: 420,
    };

    // this is the actual system that bevy will run
    move |mut commands, res| {
        // we can use `config` here, the value from above will be "moved in"
        // we can also use our system params: `commands`, `res`
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // note the parentheses `()`
        // we are calling the "constructor" we made above,
        // which will return the actual system that gets added to bevy
        .add_systems(Update, my_system_constructor())

        .run();
}
// ANCHOR_END: constructor
}
