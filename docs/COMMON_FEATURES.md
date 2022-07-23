# Common Features Between Modes
As[mode]us, shared of between uh... repositories... or something.

### What you need to know
Malachite is fairly fleshed out in Repository mode, and not so much in Workspace mode.

This isn't of course because I'm lazy and hate Workspace mode or anything, there's just not
a whole lot *to* add.

Without further ado, let's take a look at this example config file.

```toml
# mlc.toml

[base]
mode = "workspace"
smart_pull = true

[mode.repository]
name = ""
build_on_update = false

[mode.repository.signing]
enabled = false
key = ""
on_gen = false

[mode.workspace]

[repositories]
name = [
  "1::foo",
  "1::bar"
  "2::baz",
  "2::qux",
]
urls = [
  "https://example.org/%repo%.git",
  "https://example.org/other/%repo%.git",
]
```

Now this is going to look really confusing at first, but bear with me. 

In this document we'll cover only what is required to know for **both** modes.
More specialised tutorials will be linked for each mode at the bottom of this page.

Let's start with the base(ics).


### Base Config
The base config defines a few common parameters between all the Malachite modes.

```toml
[base]
mode = "workspace"
smart_pull = true
```

In this snippet, we define `mode` to be `"workspace"`.

`base.mode` in Malachite can only ever be one of `"workspace"` or `"repository"`, and defines, drumroll...
The mode in which it operates. If it is set to anything but those 2 modes, it crashes.

Also defined in this snippet is `smart_pull`, which controls whether or not to pull... smartly.

What that actually means is that instead of just performing a simple `git pull` in each repository, Malachite
will:

- First run `git remote update` to fetch new remotes from origin 
- Then run `git status` and parse the output to see if the current branch is behind
- If the current branch is behind, it runs a regular `git pull`, which will take advantage of the remotes
  already being updated. 

Theoretically, this only actually speeds things up by a minute amount (think milliseconds, really). Where this feature shines however is in repository mode,
where it enables helpful automation features such as `build_on_update`.

Regardless, it's recommended to keep this enabled for the slight speedup, and only disable it if it causes issues.
I've never personally had issues with it in the past, but who knows what could happen. This is Git we're talking about.


### Repositories Config 

The repositories config is realistically what makes Malachite churn repo butter internally. It's the whole
purpose of what it does, and because of that we've tried to come up with a neat little system so as to help
facilitate many packages without having to type each url out a million times.

```toml
[repositories]
name = [
  "1::foo",
  "1::bar"
  "2::baz",
  "2::qux",
]
urls = [
  "https://example.org/%repo%.git",
  "https://example.org/other/%repo%.git",
]
```

The way this works is simple: 
- We have 2 urls in the `repositories.urls` key.
- Each `name` in the `repositories.name` key is prefixed with an index.
- If the number is `N`, it'll insert the name into the `N`th URL.
  - Specifically, in the repo's URL, it'll insert the defined `name` in place of the `%repo%` substring.

That's literally it!


### Mode-Specific Config

For mode-specific config, avert your eyes to the following links!

- [Workspace Mode](WORKSPACE_MODE.md)
- [Repository Mode](REPOSITORY_MODE.md)

Alternatively, you can look at more [Detailed Usage](USAGE.md)
