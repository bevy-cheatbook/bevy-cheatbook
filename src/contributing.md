# Contributing

Be civil. If you need a code of conduct, have a look at Bevy's.

If you have any suggestions for the book, such as ideas for new content, or
if you notice anything that is incorrect or misleading, please file issues in
[the GitHub repository](https://github.com/bevy-cheatbook/bevy-cheatbook)!

## Contributing Code

If you simply want to contribute code examples to the book, feel free to
make a PR. I can take care of writing the book text / page that your code
will be displayed on.

### Cookbook Examples

The code for cookbook examples should be provided as a full, runnable,
example file, under `src/code/examples`. The book page will only show the
relevant parts of the code, without unnecessary boilerplate.

Always use [mdbook anchor syntax](https://rust-lang.github.io/mdBook/format/mdbook.html#including-portions-of-a-file),
not line numbers, to denote the parts of the code to be shown on the page.

### Credits

If you contribute a cookbook example, I will credit you in the book by your
github username with a link to the PR. Please let me know if you prefer not
to be credited, or if you would like to be credited in another way (but no
commercial self-promotion allowed).

## Contributing Book Text

I do not directly merge book text written by other people. This is because
I want the book to follow a consistent editorial style.

If you would like to write new content for the book, feel free to make a
PR with the content to be included, but note that it will likely not be
preserved exactly as you wrote it.

I will likely merge it into a temporary branch and then edit or rewrite it
as I see fit, for publishing into the book.

## Licensing

To avoid complications with copyright and licensing, you agree to provide
any contributions you make to the project under the [MIT-0 No Attribution
License](https://github.com/bevy-cheatbook/mit-0).

Note that this allows your work to be relicensed without preserving your
copyright.

As described previously, the actual published content in the book will be my
own derivative work based on your contributions. I will license it consistently
with the rest of the book; see: [License](./introduction.md#license).

## Bevy version

Content written for the current Bevy release, is accepted for the `main`
branch of the book.

Content written for new developments in Bevy's main branch, is accepted for the
`next` branch of the book, in preparation for the next upcoming Bevy release.

## Style Guidelines

Aim for simplicity and minimalism. Do not include things irrelevant to
getting the point across.

"Perfection is achieved not when there is nothing more to add, but when
there is nothing more to remove."

Don't forget to point out potential gotchas and other relevant practical
considerations.

Try to use the most common/standard terminology and keywords, to make things
easy to find. Don't come up with new/extra terminology of your own.

Avoid repeating information found elsewhere in the book, prefer linking to
it instead.

### Code Style

Code snippets in the Cheatsheet should be as concise as possible. Feel free
to use meaningless placeholder names; don't try to make them "realistic".

Code in other sections, however, should aim to "look realistic", as to
illustrate what the feature might actually be used for.

Avoid long lines of code, to keep it readable on small screens.

Use reasonable formatting that does not deviate much from the common
conventions used by the Rust language community. I don't enforce it strictly;
there is no need to use `rustfmt`. If deviating from those standards allows
for the code to be presented better in the context of the book, then doing
so is preferable.

### Text Style

Make it easy to read.

- Be brief. Try to cover all important information without verbose explanations.
- Prefer simple English with short sentences.
- Avoid information overload:
  - Split things into short paragraphs.
  - Avoid introducing many (even if related) topics at the same time.
  - Cover advanced usage separately from the basics.

