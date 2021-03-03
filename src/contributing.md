# Contributing

If you have any suggestions for the book, such as ideas for new content, or if
you notice anything that is incorrect or misleading, please file issues in [the
GitHub repository](https://github.com/bevy-cheatbook/bevy-cheatbook)!

Code contributions are welcome! Feel free to directly make PRs for new examples,
or changes to existing code. Code only. I will take care of the book text / page
that your code will be displayed on.

By contributing, you agree to waive all copyright, to the extent possible under
law, and provide your work under the CC0 public domain license.

If you contribute a cookbook example, I will credit you in the book by your
github username with a link to the PR. Please let me know if you prefer not to
be credited, or if you would like to be credited in another way (but no
commercial self-promotion allowed).

Direct contributions to the book text are *not* accepted. I want to author it
all myself, so that the book can follow a coherent editorial style, and so that
there are no complications with copyrights and licensing.

Be civil. If you need a code of conduct, have a look at Bevy's.

## Help Wanted

Specific things that I would really appreciate help with:

 - Adding new syntax to the [cheatsheet for the latest Bevy Git](./cheatsheet/master.md).
 - More [Cookbook](./cookbook/_index.md) examples.
 - More content for [Advanced Patterns](./patterns/_index.md) and [Common Pitfalls](./pitfalls/_index.md).
   - Especially if you frequent the Bevy Discord (or other communities) and know what kinds of things people come across.

## Bevy version

The book is targeted at the latest bevy release (0.4).

Content that covers new additions in bevy git, is also welcome! It needs to be clearly marked.

Anything that exists in both the release version and the git version, should be written for the release version.

## Example Code

All code that uses should live under `src/code_bevy_release` or `src/code_bevy_master`, depending on the version of Bevy used.

Cookbook recipes should be independent files under `examples`.

Cheatsheet code lives in `src/cheatsheet.rs`.

## Style Guidelines
 
Code snippets in the Cheatsheet should be as concise as possible. Feel free to use meaningless placeholder names; don't try to make them "realistic".
 
Code in other sections, however, should aim to "look realistic", as to illustrate what the feature might actually be used for.

Aim for simplicity and minimalism. Do not include things irrelevant to getting the point across. "Perfection is achieved not when there is nothing more to add, but when there is nothing more to remove."

Try to use the most common/standard terminology and keywords, to make things easy to find. Don't come up with new/extra terminology of your own.

Avoid long lines of code, to keep it readable on small screens.
