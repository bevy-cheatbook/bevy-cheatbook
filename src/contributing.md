# Contributing

Any help with maintaining or improving the book is welcome! Contribute via [GitHub](https://github.com/bevy-cheatbook/bevy-cheatbook)!

Feel free to submit PRs for anything you consider useful. I will review them with you, so we can edit as needed to bring them to the style and quality desired for this book.

You should aim to adhere to the guidelines on this page, but don't worry too much about it; I'll happily work with you to improve your contributions.

If you prefer to only write code, that's fine. I can write the text to go with the code for you.

Even if you just have an idea, but don't want to write the content yourself, suggest it in an issue!

Be civil. If you need a code of conduct, have a look at Bevy's.

## Help Wanted

Specific things that I would really appreciate help with:

 - Adding new syntax to the [cheatsheet for the latest Bevy Git](./cheatsheet/master.md).
 - More content for [Advanced Patterns](./patterns/_index.md), more [Cookbook](./cookbook/_index.md) examples.
   - Especially if you frequent the Bevy Discord (or other communities) and know what kinds of things people come across.

## Credits

When I merge your contributions to the book, I will mention you by your github username with a link to your PR.

Let me know if you don't want to be credited, or if you prefer to be credited in some other way.

## Bevy version

The book is targeted at the latest bevy release (0.4).

Content that covers new additions in bevy git, is also welcome! It needs to be clearly marked.

Anything that exists in both the release version and the git version, should be written for the release version.

## Example Code

All code that uses Bevy APIs should live under `src/code_bevy_release` or `src/code_bevy_master`, depending on the version of Bevy used.

Cookbook recipes should be independent files under `examples`. Only embed the relevant parts into the book page, without unnecessary boilerplate. Link to the full example file at the top of the page.

Cheatsheet code lives in `src/cheatsheet.rs`.

Miscellaneous code should live in `src/lib.rs`.

Always use mdbook anchor syntax to embed code, never line numbers.
[See here](https://rust-lang.github.io/mdBook/format/mdbook.html#including-portions-of-a-file).

## Style Guidelines
 
Code snippets in the Cheatsheet should be as concise as possible. Feel free to use meaningless placeholder names; don't try to make them "realistic".
 
Code in other sections, however, should aim to "look realistic", as to illustrate what the feature might actually be used for.

If you are writing text for the book, follow these guidelines to avoid the need for edits to your PRs and get them merged more easily:

 - Reduce verbosity. Try to get the point across with simpler code and explanations.
   - "Perfection is achieved not when there is nothing more to add, but when there is nothing more to remove."
 - Avoid repeating information found elsewhere in the book.
 - Don't forget to point out gotchas, workarounds, and other relevant practical considerations.
 - Make it easy to read:
   - Try to cover all important information without wordy explanations.
   - Prefer simple English with short sentences.
 - Use the most common/standard terminology and keywords, to make things easy to find.
   - Don't come up with new/extra terminology of your own.
 - Avoid information overload
   - Cover advanced usage separately from the basics.
   - Prefer showing related features in a single code snippet (when not confusing). Avoid repeated/similar snippets.
 - Avoid long lines of code, to keep it readable on small screens.
