{{#include ./include/links-common.md}}

# Unofficial Bevy Cheat Book

This is a reference-style book for the [Bevy game engine][bevy::website]
([GitHub][project::bevy]).

It aims to teach Bevy concepts in a concise way, help you be productive,
and discover the knowledge you need.

This book aggregates a lot of community wisdom that is often not covered
by official documentation, saving you the need to struggle with issues that
others have figured out already!

While it aims to be exhaustive, documenting an entire game engine is
a monumental task. I focus my time on whatever I believe the community
needs most.

Therefore, there are still a lot of omissions, both for basics and advanced
topics. Nevertheless, I am confident this book will prove to be a valuable
resource to you!

***Welcome! May this book serve you well!***

(don't forget to
<a class="github-button" href="https://github.com/bevy-cheatbook/bevy-cheatbook" data-icon="octicon-star" aria-label="Star bevy-cheatbook/bevy-cheatbook on GitHub">Star</a>
the book's [GitHub repository][project::cb],
and consider [donating](https://github.com/sponsors/inodentry) 🙂)

## How to use this book

The pages in this book are not designed to be read in order. Each page covers
a standalone topic. Feel free to jump to whatever interests you.

If you have a specific topic in mind that you would like to learn about, you
can find it from the table-of-contents (sidebar) or using the search function
(in the top bar).

The [Chapter Overview][chapter::overview] page will give you a general idea
of how the book is structured.

The text on each page will link to other pages, where you can learn about other
things mentioned in the text. This helps you jump around the book.

If you are new to Bevy, or would like a more guided experience, try the
[Guided Tour tutorial][cbtut::guide]. It will help you navigate the book in
an order that makes sense for learning, from beginner to advanced topics.

The [Bevy Builtins][chapter::builtins] page is a concise cheatsheet of useful
information about types and features provided by Bevy.

## Recommended Additional Resources

Bevy has a rich collection of [official code examples][bevy::examples].

Check out [bevy-assets][bevyassets], for community-made resources.

Our community is very friendly and helpful. Feel welcome to join the [Bevy
Discord][bevy::discord] to chat, ask questions, or get involved in the project!

If you want to see some games made with Bevy, see [itch.io][itchio::bevy]
or [Bevy Assets][bevyassets::games].

## Is this book up to date?

Bevy has a very rapid pace of development, with new major releases around
3 times a year. Every version brings a lot of changes, so keeping this book
updated can be a major challenge.

To ease the maintenance burden, the policy of the project is that the book may
contain content for different versions of Bevy. However, mixing Bevy versions
on the same page is not allowed.

At the top of every page, you will see the version it was last updated for.
All content on that page must be relevant for the stated Bevy version.

## Support Me

<a href="https://github.com/sponsors/inodentry"><button class="ghsponsors-button">GitHub Sponsors</button></a>
<a href="https://patreon.com/iyesgames"><button class="patreon-button">Patreon</button></a>
<a href="bitcoin:bc1qaf32uqsg6mngw9g4aqc3l2jvuv46qx0zw2438p"><button class="bitcoin-button">Bitcoin</button></a>

If you like this book, please consider sponsoring me. Thank you! ❤️

I'd like to keep improving and maintaining this book, to provide a high-quality
independent learning resource for the Bevy community.

## Support Bevy

If you like the Bevy Game Engine, you should consider donating to the project.

You can do that via the [Bevy Foundation][bevy::foundation].

## License

Copyright © 2021-2024 Ida Borisova (IyesGames)

All code in the book is provided under the
[MIT-0 License](https://github.com/bevy-cheatbook/mit-0).
At your option, you may also use it under the regular MIT License.

The text of the book is provided under the
[CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/).

Exception: If used for the purpose of contribution to the "Official Bevy
Project", the entire content of the book may be used under the [MIT-0
License](https://github.com/bevy-cheatbook/mit-0).

"Official Bevy Project" is defined as:
 - Contents of the Git repository hosted at [https://github.com/bevyengine/bevy](https://github.com/bevyengine/bevy)
 - Contents of the Git repository hosted at [https://github.com/bevyengine/bevy-website](https://github.com/bevyengine/bevy-website)
 - Anything publicly visible on the [bevyengine.org](https://bevyengine.org) website

The MIT-0 license applies as soon as your contribution has been accepted upstream.

GitHub Forks and Pull Requests created for the purposes of contributing to
the Official Bevy Project are given the following license exception: the
Attribution requirements of CC BY-NC-SA 4.0 are waived for as long as the
work is pending upstream review (Pull Request Open). If upstream rejects
your contribution, you are given a period of 1 month to comply with the
full terms of the CC BY-NC-SA 4.0 license or delete your work. If upstream
accepts your contribution, the MIT-0 license applies.

## Contributions

Development of this book is hosted on [GitHub][project::cb].

Please file GitHub Issues for any wrong/confusing/misleading information,
as well as suggestions for new content you'd like to be added to the book.

I generally work on the book on my own and ignore pull requests.

Please do not create PRs. Or if you do, be prepared for them to be ignored
or closed if I find that they take up too much of my time or don't help me
enough.

See the [Contributing][cb::contributing] section for all the details.

## Stability Warning

Bevy is still a new and experimental game engine! It has only been public
since August 2020!

While improvements have been happening at an incredible pace, and development
is active, Bevy simply hasn't yet had the time to mature.

*There are no stability guarantees and breaking changes happen often!*

Usually, it not hard to adapt to changes with new releases, but you have been
warned!
