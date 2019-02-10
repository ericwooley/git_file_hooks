# git_file_hooks

Run git hooks based off of a yaml file.

## Installation

1.  `bash <(curl -fsSL https://raw.githubusercontent.com/ericwooley/git_file_hooks/master/install.sh)` or curl into a file, inspect and run that.
2.  create .file_hooks.yml with the content

```yml
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

# applypatch-msg: *BASIC_HOOK_COMMAND
# pre-applypatch: *BASIC_HOOK_COMMAND
# post-applypatch: *BASIC_HOOK_COMMAND
# pre-commit: *BASIC_HOOK_COMMAND
# prepare-commit-msg: *BASIC_HOOK_COMMAND
# commit-msg: *BASIC_HOOK_COMMAND
# post-commit: *BASIC_HOOK_COMMAND
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

## About .file_hooks.yml

At the root is the name of the hook (currently only post-checkout is supported).

The next level down is an array of `{patterns: [], commands: []}`

The patterns are globs that will be run against the files that changed, if any of them match, the commands will be run (in parallel).

The commands for a pattern are run in parallel, but each command pattern object is run in synchournously, and in order.
