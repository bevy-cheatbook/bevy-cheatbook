# Profiling

This chapter is based on the [doc available in the bevy
repo](https://github.com/bevyengine/bevy/blob/main/docs/profiling.md) for
profiling.

When creating a bevy app, even after [enabling
optimization](../pitfalls/performance.md), your app might still be too slow,
(typically, this is a problem if you have very low frame rate). If
this is the case, you will need to figure out what slows down your app.

To see in which functions your app is spending most of its time, you have a
few options:
* [cargo flamegraphs](https://github.com/flamegraph-rs/flamegraph)
* [tracy](https://github.com/wolfpld/tracy)
* [chrome traces](https://ui.perfetto.dev/)

All three have upsides and downsides.

## Cargo flamegraphs

Cargo flamegraphs is the easiest to use. However, unlike the other
options, you will not have a real time view of your system execution nor will
you be able to see the order in which your systems run. Flamegraphs may also be
less accurate and intuitive than the trace-based methods.

To get a flamegraph, you need to:
1. [install the dependencies described in the flamegraph doc](https://github.com/flamegraph-rs/flamegraph#installation)
2. Install cargo flamegraph, with `cargo install flamegraph`
3. Run your game (or an example) with `cargo flamegraph --open` (it runs in
   release mod by default)

That's it. You should now see a flamegraph, **the plot is interactive** you can
click on sections to "zoom" on specific call graphs.

![bevy flamegraph illustration](https://user-images.githubusercontent.com/2694663/141657609-0089675d-fb6a-4dc4-9a59-871e95e31c8a.png)

The flamegraph represents each functions as boxes, their width represent how much time is
spent in them. At the base there are the "thread pool workers", they are the
"main" function of any multithreaded application, they will call functions that
themselves call other functions so forth and so forth. The deeper in the chain
of function calls you are, the higher in the flamegraph you are, and naturally,
the thinner you are.

You should be looking for unusually large functions,
typically a fat column. You can use the search feature at the top right to
highlight functions with specific names.

At this point, the flamegraph might not be recording all your functions calls.
To see more functions in the graph, you should do the following:
* If you get an error when running flamegraph, make sure that you disable the
  `bevy/dynamic` feature!
* Run `RUSTFLAGS='-C force-frame-pointers=y' cargo flamegraph`
* You may have more luck with `RUSTFLAGS='-C force-frame-pointers=y' cargo flamegraph -c "record -g"`
* Add the `profile.release.debug = true` line to your `Cargo.toml`

## Tracy

Tracy and chrome-trace let you see in real time which systems are running in
your bevy app. Unlike `flamegraph`, you need to install an application locally
for this visualization.

To get the traces of your bevy app, you need:
1. To install [tracy](https://github.com/wolfpld/tracy)
2. Start tracy
3. Run your bevy app with the `trace` and `trace_tracy` features, like so:
   `cargo run --release --features="bevy/trace bevy/trace_tracy"`
4. In tracy, click the "connect" button
5. There is a lot going on, **do not panic**. Click on the "Statistics" button
   at the top of the screen

This view should shows you in decreasing order which systems take the most time
in your app.

You can also have look at the main view with the colored lines. To make sense
of it, zoom with the scroll wheel. At one point, you'll see at
the top a series of boxes named "frame". Those are each update cycle of your
game. If you right-click one of the frame boxes, you'll see an overlay
delimiting all system running during a specific frame. You can chose to "add
annotation" in the context menu to make the overlay permanent.

### Adding more tracing spans

If the _system_ level of tracing is not enough for you, you can add _tracing
spans_ to your functions. This way, you can see when individual spans of code
are executed.

```rust
fn gardening_system(input: Res<Input>, flowers: Query<&mut Flower>) {
    //...
        let span = info_span!("span_name").entered();//←Start the code to trace
        flower.cut_with_tool(device);
        span.exit();//←End the tracing span this way
    //...
}
impl Flower {
    fn cut_with_tool(&mut self, device: Tool) {
        // ...
    }
}
```
