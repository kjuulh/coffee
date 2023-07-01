# Coffee

Coffee aims to be a github-cli compatible cli for gitea, as github-cli is for
github

The current tool is still in a very early state, and right now is focused on
handling repositories and pull requests, as that is the stuff I use most myself.

The input may be a bit different than `gh`, and I don't aim for full
compatbility, I want however, the tools to be comparable, so that you can use
them in the same way, in an intuitive way.

## Installation

Because the tool right now is only used by me, the best way to install it is to
build it manually. Simply:

```bash
gh repo clone kjuulh/coffee
cd coffee
cargo install --path crates/coffee
coffee --version # profit!
```

If you get an error when doing `coffee --version` you may have to add
`~/.cargo/bin` to your path like so `export $PATH:$HOME/.cargo/bin`.

I will at some point add both `crates` releases, as well as `homebrew`, so you
may wait until then or, raise an issue so that I know someone else is interested
in it.

## Note:

The gitea client used is machine generated for rust using openapi generator,
because of a recent addition of `Sudo` to giteas api, the gitea_client isn't
useful in of itself, as it can't handle authentication. It is on the roadmap to
fix this, in the meantime I set the token in a private reverse proxy for gitea.
This may not be suitable for you, but as of this moment it is a requirement.
