# Repository Mode
PacManage your repositories in style!

### Repository Config

As opposed to the rather barren Workspace mode, the Repository mode config is rather fleshed out,
and we have a few options to choose from.

Let's take an example config,

```toml
[mode.repository]
name = "example"
build_on_update = true

[mode.repository.signing]
enabled = true
key = "you@example.org"
on_gen = true
```

### Basic Repository Config

To start with, there are 2 main config keys to Repository mode:
- `name`: Defines what pacman calls your repository.
- `build_on_update`: In conjunction with `smart_pull`, defines whether to rebuild packages automatically when an update is detected.

### Signing

Malachite also supports, and encourages the signing of packages.
GPG Signing packages ensures that the user receives exactly what you packaged, without any chance of tampering.

Calling back to the example above, we can see 3 config keys:

- `enabled`: Defines whether to sign packages (heavily encouraged).
- `key`: Defines the GPG key ID to use for signing.
- `on_gen`: Defines whether to sign packages when they are built, or all at once on repository generation (this is also recommended).

