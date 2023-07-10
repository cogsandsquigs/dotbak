[![License](https://img.shields.io/github/license/cogsandsquigs/dotbak?style=for-the-badge)](https://github.com/cogsandsquigs/dotbak/blob/main/LICENSE)
[![CircleCI](https://img.shields.io/circleci/build/github/cogsandsquigs/dotbak/main?style=for-the-badge)](https://app.circleci.com/pipelines/github/cogsandsquigs/dotbak)

# dotbak

Manage and backup dotfiles with ease!

## Configuration

Configuration for `dotbak` is stored in `$HOME/.dotbak/config.toml`. This file is created automatically when `dotbak init` is run for the first time.

### `files.include` and `files.exclude`

These tell the `dotbak` which files to include and exclude from the backup. These are specified as glob patterns. The default values are:

```toml
[files]
	include = [".dotbak/config.toml"]
	exclude = [".dotbak/dotfiles/**/*"]
```

Note that this `dotbak` configuration can also work with plain folders, such as `.config` or `.local`. For example, to backup the `.config` folder, you would set `files.include` to `[".config"]`. Under the hood, for every folder in the `include` or `exclude` fields, it's appended with `/**/*` and then turned into a glob pattern.

TODO: Fix this so it works with folders. Idea: maybe append

## Dotfile Management

Dotfiles are symlinked and stored in `$HOME/.dotbak/dotfiles`. This directory is created automatically when `dotbak init` is run for the first time. `dotbak` manages a git reposiotry in this directory, and all dotfiles are stored in this repository. This repository is automatically pushed to the remote repository specified in the configuration file.
