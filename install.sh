#!/usr/bin/env bash
# https://stackoverflow.com/questions/3231804/in-bash-how-to-add-are-you-sure-y-n-to-any-command-or-alias
confirm() {
    # call with a prompt string or use a default
    read -r -p "${1:-Are you sure?} [y/N] " response
    case "$response" in
        [yY][eE][sS]|[yY]) 
            true
            ;;
        *)
            false
            ;;
    esac
}
OS=$OSTYPE;
if [[ "$OSTYPE" == "darwin"* ]]; then
  OS="darwin";
fi
VERSION="v0.0.5";
NOW=$(date +%Y.%m.%d-%H:%M:%S)
install_hook() {
    HOOK=$1;
    if [ -f .git/hooks/$i ]; then
        if [ -z "$NO_HOOK_BACKUPS" ]; then
            echo "$1 backed up to $1.$NOW.bak"
            mv .git/hooks/$1 .git/hooks/$1.$NOW.bak
        fi
    fi
    if [ -z "$USE_LOCAL" ]; then
        echo "installing $HOOK";
        # TODO: This could be much much faster if we download to a tmp file and just copy.
        curl -L -s https://github.com/ericwooley/git_file_hooks/releases/download/$VERSION/git_file_hooks-$OS -o .git/hooks/$HOOK;
    else
        # useful for testing
        echo "installing local build to $HOOK";
        cp build/git_file_hooks-$OS .git/hooks/$HOOK
    fi
    chmod +x .git/hooks/$HOOK;
}

# main
hooks=(
    "applypatch-msg"
    "pre-applypatch"
    "post-applypatch"
    "pre-commit"
    "prepare-commit-msg"
    "commit-msg"
    "post-commit"
    "pre-rebase"
    "post-checkout"
    "post-merge"
    "pre-push"
    "pre-receive"
    "update"
    "post-receive"
    "post-update"
    "push-to-checkout"
    "pre-auto-gc"
    "post-rewrite"
    "rebase"
    "sendemail-validate"
    "fsmonitor-watchman"
);

echo "set OVERWRITE_ALL_HOOKS=true to skip overwrite checks: $OVERWRITE_ALL_HOOKS";
for i in "${hooks[@]}"
do 
    : 
    if [ ! -f .git/hooks/$i ]; then
        install_hook $i;
    elif [ ! -z "$OVERWRITE_ALL_HOOKS" ]; then
        install_hook $i;
    else
        confirm "overwrite $i" \
            && install_hook $i;
    fi
done