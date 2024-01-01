[![License](https://img.shields.io/github/license/cogsandsquigs/dotbak?style=for-the-badge)](https://github.com/cogsandsquigs/dotbak/blob/main/LICENSE)
[![CircleCI](https://img.shields.io/circleci/build/github/cogsandsquigs/dotbak/main?style=for-the-badge)](https://app.circleci.com/pipelines/github/cogsandsquigs/dotbak)
[![Crates.io](https://img.shields.io/crates/v/dotbak?style=for-the-badge)](https://crates.io/crates/dotbak)

# dotbak

Manage and backup dotfiles with ease!

## Why dotbak?

Because everyone else did their own thing, and I wanted to do my own thing too. After all, why should _I_ trust _someone else's_ shitty CLI when I can make my own brilliant, amazingly awesome, and totally not shitty CLI?

## Installation

Install with `cargo install dotbak`. This will install the `dotbak` binary to `$HOME/.cargo/bin/dotbak`. Make sure that this directory is in your `$PATH`. If you want to upgrade, run the same command.

> TIP: You can add `--force` to the command to force a reinstall, even if there's no new version.

## Dotfile Management

Dotfiles are symlinked and stored in `$HOME/.dotbak/dotfiles`. This directory is created automatically when `dotbak init` is run for the first time. `dotbak` manages a git reposiotry in this directory, and all dotfiles are stored in this repository. This repository is automatically pushed to the remote repository specified in the configuration file.

## Configuration

Configuration for `dotbak` is stored in `$XDG_HOME_DIR/.dotbak/config.toml` or `$HOME/.dotbak/config.toml`. This file is created automatically when `dotbak init` is run for the first time.

#### `repository_url`

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

-   [ ] Stream stderr/out of arbitrary commands to the terminal when running.
-   [ ] Run `dotbak sync` in the background as a daemon (on login/every x minutes).
