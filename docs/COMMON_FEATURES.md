# Common Features Between Modes
As[mode]us, shared of between uh… repositories… or something.

### What you need to know
Malachite is fairly fleshed out in Repository mode, and not so much in Workspace mode.

This isn't of course because I'm lazy and hate Workspace mode or anything, there's just not
a lot *to* add.

Without further ado, let's take a look at this example config file.

```toml
# mlc.toml

[base]
mode = "workspace"
smart_pull = true

[mode.workspace]
git_info = true
colorblind = true

[repositories]
repos = [
  "foo:repo1:2",
  "foo:repo2/testing",
  "bar:baz!",
  "bar:qux/testing!:1",
]

[repositories.urls]
foo = "https://example.org/%repo%.git"
bar = "https://example.org/other/%repo%.git"
```

Now, this is going to look really confusing at first, but bear with me. 

In this document, we'll cover only what is required to know for **both** modes.
More specialized tutorials will be linked for each mode at the bottom of this page.

Let's start with the base(ics).


### Base Config
The base config defines a few common parameters between all the Malachite modes.

```toml
[base]
mode = "workspace"
smart_pull = true
```

In this snippet, we define `mode` to be `"workspace"`.

`base.mode` in Malachite can only ever be one of `"workspace"` or `"repository"`, and defines, drumroll…
The mode in which it operates. If it is set to anything but those 2 modes, it crashes.

Also defined in this snippet is `smart_pull`, which controls whether to pull… smartly.

What that actually means is that instead of just performing a simple `git pull` in each repository, Malachite
will:

- First run `git remote update` to fetch new remotes from origin 
- Then run `git status` and parse the output to see if the current branch is behind
- If the current branch is behind, it runs a regular `git pull`, which will take advantage of the remotes
  already being updated. 

Theoretically, this only actually speeds things up by a minute amount (think milliseconds, really). Where this feature shines however is in repository mode,
where it enables helpful automation features such as `build_on_update`.

Regardless, it's recommended to keep this enabled for the slight speed-up, and only disable it if it causes issues.
I've never personally had issues with it in the past, but who knows what could happen. This is Git we're talking about.


### Repositories Config 

The repositories config is realistically what makes Malachite churn repo butter internally. It's the whole
purpose of what it does, and because of that we've tried to come up with a neat little system to help
facilitate many packages without having to type each URL out a million times.

```toml
[repositories]
repos = [
  "foo:repo1:2",
  "foo:repo2/testing",
  "bar:baz!",
  "bar:qux/testing!:1",
]

[repositories.urls]
foo = "https://example.org/{}.git"
bar = "https://example.org/other/{}.git"
```

The way this works is simple: 
- We have 2 URLs in the `repositories.urls` key.
- Each `repo` in the `repositories.repos` key is prefixed with an identifier.
- If the number is `foo`, it'll insert the URL with the id `foo`.
  - Specifically, in the URL, it'll insert the defined `repo`'s name in place of the `%repo%` substring.

#### Hang on, what are the special symbols????

I'm glad you asked!
- If you want to clone a specific branch, simply use the `/` delimiter. To clone repository `foo` on branch `bar`, use `id:foo/bar`.
- If you want a specific package to build first, use instances of `!` to set priority. This is explained later in the [Repository Mode](REPOSITORY_MODE.md) page

The last `:` delimiter is entirely optional, and behaves differently depending on the mode:
- In Repository mode, it defines the desired commit hash/rev/tag to checkout on repository clone
- In Workspace mode, it defines the desired depth to clone the repository, useful with large git repositories, such as `nixpkgs`.

That's literally it!


### Mode-Specific Config

For mode-specific config, avert your eyes to the following links!

- [Workspace Mode](WORKSPACE_MODE.md)
- [Repository Mode](REPOSITORY_MODE.md)

### Examples

Functioning config examples for both modes are available in the [examples](../examples) directory!

### Usage 

Alternatively, you can look at the [Usage](USAGE.md) guide!

