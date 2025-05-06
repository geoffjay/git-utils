[![cargo build](https://github.com/geoffjay/git-utils/actions/workflows/build.yml/badge.svg)](https://github.com/geoffjay/git-utils/actions/workflows/build.yml)

---

### Git Utils

Some personal `git` commands that have been adapted from aliases. My current
`git` config has several aliases that feed into one that I use regularly for
syncing a local repository with it's remote, this looks like:

```shell
x-repo = !"f() { git config --get remote.origin.url | sed 's/^.*\\:\\(.*\\)\\.git/\\1/'; }; f"
x-fetch-all = fetch --no-tags --all -p
x-fetch-branch = !"f() { git fetch origin $1:$1; }; f"
x-merge-ff = merge --ff-only
x-branch-current = rev-parse --abbrev-ref HEAD
x-branch-default = !"f() { git remote show origin | grep HEAD | awk '{print $3}'; }; f"
x-branch-tidy = "!f() { git branch -vv | grep ': gone]' | awk '{print $1}' | xargs -n 1 git branch -D; }; f"
up = \
!"f() { \
  current=$(git x-branch-current); \
  default=$(git x-branch-default); \
  git x-fetch-all && \
  (git x-merge-ff || true) && \
  git x-branch-tidy && \
  if [ \"$current\" != \"$default\" ]; then git x-fetch-branch $default; fi; \
}; f"
```

The end goal of this project is to have a single `git-sync` that can be used to
do the same, but hopefully faster.

#### `git-current-branch`

Prints the current branch for a local repository.

#### `git-default-branch`

Prints the default branch, eg. `main` or `master`, for a repository.

#### `git-repo-title`

Prints the repository, eg. `geoffjay/git-utils`.

#### `git-repo-url`

Prints the full repository url, eg. `git@github.com:geoffjay/git-utils`.

#### `git-sync`

Synchronizes the local with remote and cleans up branches that have been removed.

### Installation

#### macOS

Currently there are builds in the release for x86_64 and aarch64.

This assumes that `~/.local/bin` exists, is in your `PATH`, and is where you
want them to go. Adjust the command for your environment if this is not what's
desired.

##### Intel

```shell
curl -LO https://github.com/geoffjay/git-utils/releases/latest/download/git-utils-v0.1.3-darwin-x86_64.tar.gz
tar -xf git-utils-v0.1.3-darwin-x86_64.tar.gz -C ~/.local/bin
rm git-utils-v0.1.3-darwin-x86_64.tar.gz
```

##### Aarch64

```shell
curl -LO https://github.com/geoffjay/git-utils/releases/latest/download/git-utils-v0.1.3-darwin-aarch64.tar.gz
tar -xf git-utils-v0.1.3-darwin-aarch64.tar.gz -C ~/.local/bin
rm git-utils-v0.1.3-darwin-aarch64.tar.gz
```

#### Linux

Currently there are builds in the release for x86_64 and arm64.

This assumes that `~/.local/bin` exists, is in your `PATH`, and is where you
want them to go. Adjust the command for your environment if this is not what's
desired.

##### Intel

```shell
curl -LO https://github.com/geoffjay/git-utils/releases/latest/download/git-utils-v0.1.3-linux-x86_64.tar.gz
tar -xf git-utils-v0.1.3-darwin-x86_64.tar.gz -C ~/.local/bin
rm git-utils-v0.1.3-linux-x86_64.tar.gz
```

##### ARM64

```shell
curl -LO https://github.com/geoffjay/git-utils/releases/latest/download/git-utils-v0.1.3-linux-amd64.tar.gz
tar -xf git-utils-v0.1.3-darwin-amd64.tar.gz -C ~/.local/bin
rm git-utils-v0.1.3-linux-amd64.tar.gz
```

#### Windows

There is a build for Windows, but I don't have a way to test that so this
section will be left as-is for the foreseeable future.

### License

Licensed under the MIT license, see [here](./LICENSE).
