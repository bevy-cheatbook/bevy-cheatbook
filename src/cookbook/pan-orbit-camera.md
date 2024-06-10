{{#include ../include/header014.md}}

# Pan + Orbit Camera

This is a camera controller similar to the ones in 3D editors like Blender.

To make the implementation simpler, we do not manipulate the
[transform][cb::transform] directly. Instead, we work with values inside of
a custom [component][cb::component] struct and then compute the transform
at the end.

Furthermore, for completeness, this example will also show a simple way of
making the input controls reconfigurable / rebindable.

---

First, let's define our data. Create some [component][cb::component]
types, which we will store on the [3D camera][cb::camera-3d]
[entity][cb::ecs-intro-data], and a [bundle][cb::bundle] to make it easy to
spawn the camera:

<details>
  <summary>Code:</summary>

```rust,no_run,noplayground
{{#include ../code014/examples/pan-orbit-camera.rs:types}}
```

</details>

We can implement [`Default`] to give them reasonable default values:

<details>
  <summary>Code:</summary>

```rust,no_run,noplayground
{{#include ../code014/examples/pan-orbit-camera.rs:defaults}}
```

</details>

We need a setup [system][cb::system] to spawn our camera:

<details>
  <summary>Code:</summary>

```rust,no_run,noplayground
{{#include ../code014/examples/pan-orbit-camera.rs:setup}}
```

```rust,no_run,noplayground
{{#include ../code014/examples/pan-orbit-camera.rs:setup-app}}
```

</details>

And finally, the actual implementation of the camera controller:

<details>
  <summary>Code:</summary>

```rust,no_run,noplayground
{{#include ../code014/examples/pan-orbit-camera.rs:impl}}
```

We can add a [Run Condition][cb::rc] to tell Bevy to run
our system only if pan-orbit entities exist:

```rust,no_run,noplayground
{{#include ../code014/examples/pan-orbit-camera.rs:impl-app-rc}}
```

</details>
