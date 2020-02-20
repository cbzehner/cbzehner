---
title: Search Strategies for Esoteric Languages
---

<img src="/images/github-search.gif" alt="Searching 'All Github' code search" style="width: 140%; margin-left: -4rem; border-radius: 1rem;" />

## What to do when Stackoverflow won't?
I recently started using [Rust](https://www.rust-lang.org/) for hobby projects and especially the [Rocket](https://rocket.rs/) framework. The exceedingly pleasant and useful documentation for both of these projects continues to delight, but I frequently run into issues unhandled in the documentation or where the library provides a solution that's too abstract for me to tie the concepts back the issue I'm facing. In both situations, seeing examples of the code in use is enlightening.

However the Rust community isn't huge and the Rocket community [is even smaller](https://rocket.rs/v0.4/guide/conclusion/#getting-help), so unlike [Rails](https://rubyonrails.org/) or other established web frameworks, few other folks have encountered the same problem. This makes every programmer's usual approach of Google + Stackoverflow less effective than we're used to.

There is one other place it's common to find software solutions and even actively maintained source code though...

## Github's Global Search to the Rescue

When trying to implement a custom provider for [`rocket_oauth2`](https://github.com/jebrosen/rocket_oauth2), it was difficult for me to understand how to make it work with the existing documentation. Luckily I was able to search through Github's public codebase for examples of Rocket apps using `rocket_oauth2` and examine their implementations.

I simply looked for any instances matching the crate declaration in my `Cargo.toml` file. The same strategy works almost unaltered for packages in Javascript or gems in Ruby.

The "All Github" search feature was crucial when I was building my first Rocket application and trying to integrate with OAuth2 providers. After reading through the source of several other Rocket apps, I not only understood the pieces I'd missed in the documentation but seeing how different developers implemented the same framework showed me what structures were conventional and what were customizable within the framework.

