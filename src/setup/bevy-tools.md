{{#include ../include/header09.md}}

# Dev Tools and Editors for Bevy

Bevy does not yet have an official editor or other such tools. An official
editor is planned as a long-term future goal. In the meantime, here are
some community-made tools to help you.

---

## Editor

[`bevy_inspector_egui`](https://github.com/jakobhellermann/bevy-inspector-egui)
gives you a simple editor-like property inspector window in-game. It lets
you modify the values of your components and resources in real-time as the
game is running.

[`bevy_editor_pls`](https://github.com/jakobhellermann/bevy_editor_pls)
is an editor-like interface that you can embed into your game. It has even
more features, like switching app states, fly camera, performance diagnostics,
and inspector panels.

## Diagnostics

[`bevy_mod_debugdump`](https://github.com/jakobhellermann/bevy_mod_debugdump)
is a tool to help visualize your [App Schedule](../programming/app-builder.md)
(all of the registered [systems](../programming/systems.md) with
their [ordering dependencies](../programming/system-order.md) and
[stages](../programming/stages.md)), and the Bevy Render Graph.

[`bevy_lint`](https://github.com/MinerSebas/bevy_lint) is a linter (based on
`dylint`) that can automatically check your Bevy code for some common issues.

If you are getting confusing/cryptic compiler error messages (like
[these](../pitfalls/into-system.md)) and you cannot figure them out,
[`bevycheck`](https://github.com/jakobhellermann/bevycheck) is a tool you
could use to help diagnose them. It tries to provide more user-friendly
Bevy-specific error messages.
