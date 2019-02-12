# git_file_hooks

Manage your git hooks, and only run the ones you need. Commit a yaml file, and live free!

## Features

- 0 dependencies, just a single blob.
- place your .file_hooks.yml file anywhere up your file tree from the git repo.
- concurrently run commands.

## Potential future features

- exclude globs
- streaming output for commands instead of catpuring output and adding it later. [See this issue](https://github.com/sagiegurari/run_script/issues/4)

## Installation

1. cd to your git project.
1. run `bash <(curl -fsSL https://raw.githubusercontent.com/ericwooley/git_file_hooks/master/install.sh)` or curl into a file, inspect and run that.
1. (optionally) create .file_hooks.yml with the content

```yml
## There will be options, but there aren't yet, so this keyword is reserved
# options:

post-checkout: &BASIC_HOOK_COMMAND
  - patterns:
      - "**/yarn.lock"
    commands:
      - yarn;
  - patterns:
      - "**/package-lock.json"
    commands:
      - npm i;
  - patterns:
      - "**/Gemfile"
    commands:
      - bundle install
  - patterns:
      - "**/db/migrate/*.rb"
    commands:
      # each entry is run in parallel, so run these in order.
      - rake migrate:data; rake migrate:db;
# YAML has templates, so you can reuse configs.
hooks:
  # applypatch-msg: *BASIC_HOOK_COMMAND
  # pre-applypatch: *BASIC_HOOK_COMMAND
  # post-applypatch: *BASIC_HOOK_COMMAND
  # pre-commit: *BASIC_HOOK_COMMAND
  # prepare-commit-msg: *BASIC_HOOK_COMMAND
  # commit-msg: *BASIC_HOOK_COMMAND
  post-commit: *BASIC_HOOK_COMMAND
  # pre-rebase: *BASIC_HOOK_COMMAND
  # post-checkout: *BASIC_HOOK_COMMAND
  # post-merge: *BASIC_HOOK_COMMAND
  # pre-push: *BASIC_HOOK_COMMAND
  # pre-receive: *BASIC_HOOK_COMMAND
  # update: *BASIC_HOOK_COMMAND
  # post-receive: *BASIC_HOOK_COMMAND
  # post-update: *BASIC_HOOK_COMMAND
  # push-to-checkout: *BASIC_HOOK_COMMAND
  # pre-auto-gc: *BASIC_HOOK_COMMAND
  # post-rewrite: *BASIC_HOOK_COMMAND
  # rebase: *BASIC_HOOK_COMMAND
  # sendemail-validate: *BASIC_HOOK_COMMAND
  # fsmonitor-watchman: *BASIC_HOOK_COMMAND
```

## Skipping tests

add `SKIP_GIT_HOOKS=1` before your git command to skip tests. EG `SKIP_GIT_HOOKS=1 git push`.

## Upgrading

You can run the install script with a few enviornment variables for easier upgrades.

### Upgrade and backup existing hooks

`OVERWRITE_ALL_HOOKS=true ./install.sh`

### Upgrade and do not backup existing hooks

`OVERWRITE_ALL_HOOKS=true NO_HOOK_BACKUPS=true bash <(curl -fsSL https://raw.githubusercontent.com/ericwooley/git_file_hooks/master/install.sh)`

## Development

Use cargo:1.32.0

The make file has various helpful tasks. It's most of it's integration tests are run against the commits in this repo. Not ideal... but it sure is easy.

`make build` will generate a build for your platform.

`OVERWRITE_ALL_HOOKS=true NO_HOOK_BACKUPS=true USE_LOCAL=true bash <(curl -fsSL https://raw.githubusercontent.com/ericwooley/git_file_hooks/master/install.sh)` Will install the build from the build/\* directory instead of from the latest resease.

## Releasing a new version.

1. Push the changes, `make build` and draft a new release using `vX.X.X` where X is the release number.

2. Upload the binary, for gnu-linux and osx.

3. Publish the release

4. update the install.sh script version variable to match the released version, otherwise the install script will install the previous version.

5. push the install.sh update to master.

## About .file_hooks.yml

At the root is the name of the hook (currently only post-checkout is supported).

The next level down is an array of `{patterns: [], commands: []}`

The patterns are globs that will be run against the files that changed, if any of them match, the commands will be run (in parallel).

The commands for a pattern are run in parallel, but each command pattern object is run in synchournously, and in order.
