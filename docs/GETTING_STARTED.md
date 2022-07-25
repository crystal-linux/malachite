# Getting Started With Malachite
Baby's first Malachite repository!

### What you need to know

Malachite is:
- A pacman repository manager
- A workspace manager
- ~~Awesome~~

Malachite isn't:
- The end-all solution for all pacman repositories
- Perfect


### With that out of the way

Hi! My name is Michal, and I wrote this tool pretty much on my own for [Crystal Linux](https://getcryst.al);
but it is not at all exclusive to Crystal. This tool should and will work on and for any pacman-based
distribution (so long as it packages all of Malachite's dependencies, of course).

Throughout this tutorial, I'll explain each little feature of Malachite in what I hope to be bite-sized and
programmatic chunks.

Without further ado, let's begin with the first, most important question:


### Modes

What mode are you using malachite in?

Currently, malachite supports 2 modes:

#### Repository Mode
- Allows the user to configure and manage a remote (or local) pacman-based package repository
- Allows for customisability in repository name, signing preferences, signing key etc.
- Allows for basic levels of automation, by using features such as build_on_update

#### Workspace Mode
- The most basic functionality of Malachite
- Just clones git directories into a "Workspace" directory for easier management
- Allows for basic pulling operations to keep your repositories up-to-date

These modes essentially dictate everything about how Malachite functions, so much so that I now need to
split this page off before it gets too long!

For more info, get started with the [Common Features](COMMON_FEATURES.md) page!
