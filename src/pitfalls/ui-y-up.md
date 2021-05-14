# UI layout is inverted

[In bevy, the Y axis always points *UP*. When working with UI, the origin
is at the *bottom left* corner of the screen.](../features/transforms.md)

This means that UI is laid out from bottom to top.

This is the opposite of the typical behavior of web pages and other UI
toolkits, where layout works from top to bottom.

Bevy uses the Flexbox layout model for UI, but unlike in web pages / CSS,
the vertical axis is inverted.

Unintuitively, this means that to build UIs that flow from top to bottom,
you need to use `FlexDirection::ColumnReverse`.
