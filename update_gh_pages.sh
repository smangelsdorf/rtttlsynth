#!/usr/bin/env bash

# Builds and updates the local `gh-pages` branch from the latest source.
#
# Doesn't require checking out the `gh-pages` branch.

set -e
set -u

cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen --target web --out-dir static/ target/wasm32-unknown-unknown/release/rtttlsynth.wasm

# Bail out if the `--dry-run` argument was given
if [[ "${1:-}" = "--dry-run" ]]; then
  exit 0
fi

target_branch="gh-pages"
current_commit_hash=$(git rev-parse $target_branch)

export GIT_INDEX_FILE=$(mktemp)
git read-tree --empty

files=(index.html styles.css rtttlsynth.js rtttlsynth_bg.wasm)
for file in "${files[@]}"; do
  git update-index --add --cacheinfo 100644 "$(git hash-object -w "static/$file")" "$file"
done

tree_hash=$(git write-tree)

rm "$GIT_INDEX_FILE"
unset GIT_INDEX_FILE

new_commit_hash=$(git commit-tree $tree_hash -p $current_commit_hash -m "Build from latest source")
git update-ref "refs/heads/$target_branch" $new_commit_hash
git show $new_commit_hash