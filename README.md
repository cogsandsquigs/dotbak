[![License](https://img.shields.io/github/license/cogsandsquigs/dotbak?style=for-the-badge)](https://github.com/cogsandsquigs/dotbak/blob/main/LICENSE)
[![CircleCI](https://img.shields.io/circleci/build/github/cogsandsquigs/dotbak/main?style=for-the-badge)](https://app.circleci.com/pipelines/github/cogsandsquigs/dotbak)

# dotbak

Manage and backup dotfiles with ease!

## Configuration

Configuration for `dotbak` is stored in `XDG_CONFIG_DIR/dotbak/config.toml` or `$HOME/.config/dotbak/config.toml`. This file is created automatically when `dotbak init` is run for the first time.

### `files`

These tell the `dotbak` your settings about how you want to manage files. Currently, there's only `files.include`, which is a list of all files and folders that you want to manage. For example, if you want to manage your `.dotbak/config.toml` file, you would set `files.include` to `[".dotbak/config.toml"]`. This tells `dotbak` to manage the file at `$HOME/.dotbak/config.toml`. Note that the path is relative to `$HOME`. If you want to manage a file that's not in your home directory, you can use an absolute path. For example, if you want to manage the file at `/etc/config.toml`, you would set `files.include` to `["/etc/config.toml"]`.

```toml
[files]
	include = [".dotbak/config.toml"]
```

Note that this `dotbak` configuration can also work with plain folders, such as `.config` or `.local`. For example, to backup the `.config` folder, you would set `files.include` to `[".config"]`.

## Dotfile Management

Dotfiles are symlinked and stored in `$HOME/.dotbak/dotfiles`. This directory is created automatically when `dotbak init` is run for the first time. `dotbak` manages a git reposiotry in this directory, and all dotfiles are stored in this repository. This repository is automatically pushed to the remote repository specified in the configuration file.
