# git_file_hooks
Run git hooks based off of a yaml file.

## Installation
1. curl binary into .git/hooks/post-checkout
```bash
# linux
curl -L https://github.com/ericwooley/git_file_hooks/releases/download/0.0.2/git_file_hooks-linux-gnu -o .git/hooks/post-checkout

# osx
curl -L https://github.com/ericwooley/git_file_hooks/releases/download/0.0.2/git_file_hooks-darwin -o .git/hooks/post-checkout
```
2. create .file_hooks.yml with the content
```yml
post-checkout:
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
```

## About .file_hooks.yml
At the root is the name of the hook (currently only post-checkout is supported).

The next level down is an array of `{patterns: [], commands: []}`

The patterns are globs that will be run against the files that changed, if any of them match, the commands will be run (in parallel).

The commands for a pattern are run in parallel, but each command pattern object is run in synchournously, and in order.
