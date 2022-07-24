# Workspace Mode
You'll never have to work(space) another day in your life!

### Workspace Config

Taking an example section from the Workspace mode config,

```toml
[mode.workspace]
git_info = true
colorblind = true
```

Currently, Workspace mode only has 2 options, both pertaining to the display of information. (`mlc info`)

The first key is `git_info`, which is a boolean value. If it is true, the git information will be displayed alongside repository information.

This information will be formatted as so: `D Pl Ps <Latest Commit Hash>`

The key for the values is as follows:
- D:  Whether the repository is dirty or not (unstaged changes)
- Pl: Whether there are unpulled changes at the remote
- Ps: Whether there are unpushed changes in your local repository

These will be typically displayed in either Green (Clean) or Red (Dirty)

However, if `colorblind` is set to true, the colors will instead be set to Blue (Clean) or Dark Red (Dirty), to be more discernible to colorblind users

---

You can return to [Getting Started](GETTING_STARTED.md) page here!