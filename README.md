# git_file_hooks
Run git hooks based off of a yaml file.

## Installation
1. curl binary into .git/hooks/post-checkout
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
