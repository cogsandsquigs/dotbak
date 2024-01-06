[![License](https://img.shields.io/github/license/cogsandsquigs/dotbak?style=for-the-badge)](https://github.com/cogsandsquigs/dotbak/blob/main/LICENSE)
[![CircleCI](https://img.shields.io/circleci/build/github/cogsandsquigs/dotbak/main?style=for-the-badge)](https://app.circleci.com/pipelines/github/cogsandsquigs/dotbak)
[![Crates.io](https://img.shields.io/crates/v/dotbak?style=for-the-badge)](https://crates.io/crates/dotbak)

# dotbak

Manage and backup dotfiles with ease!

```terminal
$ dotbak sync
⏳ Syncing dotfiles...
   [1/4] 📦 Committing changes ... ✅
   [2/4] 📥 Pulling changes ...... ✅
   [3/4] 📤 Pushing changes ...... ✅
   [4/4] 🔄 Synching state ....... ✅
✨ Done! [1 second]
```

## Why dotbak?

Because everyone else did their own thing, and I wanted to do my own thing too. After all, why should _I_ trust _someone else's_ shitty CLI when I can make my own brilliant, amazing, and totally not shitty CLI?

## Installation

Install with `cargo install dotbak`. This will install the `dotbak` binary to `$HOME/.cargo/bin/dotbak`. Make sure that this directory is in your `$PATH`. If you want to upgrade, run the same command.

> TIP: You can add `--force` to the command to force a reinstall, even if there's no new version.

## Dotfile Management

Dotfiles are symlinked and stored in `$HOME/.dotbak/dotfiles`. This directory is created automatically when `dotbak init` is run for the first time. `dotbak` manages a git reposiotry in this directory, and all dotfiles are stored in this repository.

To add or remove dotfiles, use `dotbak add` and `dotbak remove`. These commands will add or remove files from the repository, and then symlink or restore the files to `$HOME`. When providing a path to your file, make sure that the path is relative to `$HOME`. For example, if you want to add `$HOME/.dotbak/config.toml`, you would run `dotbak add .dotbak/config.toml`.

> TIP: `dotbak` will not remove files from `$HOME` if they are not managed by `dotbak`.

When `dotbak sync` is run, `dotbak` will commit all changes to the repository, push the changes to the remote repository, and then pull any changes from the remote repository. Unless otherwise specified, all other commands do not push or pull changes from the remote repository (besides, yaknow, `push` and `pull`).

> TIP: Run `dotbak sync` after adding or removing files to push or pull changes from the remote repository. If you don't want the changes, run `dotbak undo` to undo the changes. **This only affects changes not yet pushed to the remote repository**.

## Configuration

Configuration for `dotbak` is stored in `$XDG_HOME_DIR/.dotbak/config.toml` or `$HOME/.dotbak/config.toml`. This file is created automatically when `dotbak init` is run for the first time.

### `repository_url`

The URL for the remote git repository. This is the URL that will be used to clone the repository if it doesn't exist, and to push and pull changes to and from the repository. Also, incase the local repository is deleted or corrupted, this URL will be used to clone the repository again.

### `files`

These tell the `dotbak` your settings about how you want to manage files.

#### `files.include`

Currently, there's only `files.include`, which is a list of all files and folders that you want to manage. For example, if you want to manage your `.dotbak/config.toml` file, you would set `files.include` to `[".dotbak/config.toml"]`. This tells `dotbak` to manage the file at `$HOME/.dotbak/config.toml`. Note that the path is relative to `$HOME`.

```toml
[files]
	include = [".dotbak/config.toml"]
```

Note that this `dotbak` configuration can also work with plain folders, such as `.config` or `.local`. For example, to backup the `.config` folder, you would set `files.include` to `[".config"]`, or run `dotbak add .config` which automatically adds the folder to the `files.include` list.

## TODO:

-   [x] Update UI to be more user friendly.
    -   [x] Adjust spacing after "steps" so that the spinner/emoji is always on the same column.
-   [x] Display stdout/stderr of commands run by `dotbak` in the terminal.
    -   [x] Fix extra newlines on output.
-   [x] Refactor code to be more modular.
-   [ ] Make an `undo`/`rollback` command to undo recent changes made by `dotbak`.
-   [ ] Run `dotbak sync` in the background as a daemon (on login/every x minutes).
-   [ ] Create binary releases via CI (CircleCI) for Linux and macOS.
-   [ ] Create AUR/Homebrew packages for `dotbak`.
