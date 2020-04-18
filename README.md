# Fix GnuPG Permissions

Fix the permissions on the GnuPG config directory.

## Usage

``` bash
fix-gnupg-permissions
```

## Installing

First tap my homebrew repo

``` shell
brew tap PurpleBooth/repo
```

Next install the binary

``` shell
brew install fix-gnupg-permissions
```

You can also download the [latest
release](https://github.com/PurpleBooth/fix-gnupg-permissions/releases/latest)
and run it.

## How does it work

Well here's a version of this in bash, if that makes it clearer

``` shell
#!/usr/bin/env bash

set -euo pipefail

# Same as doing this
chmod 700 "$HOME/.gnupg"
chmod 600 "$HOME/.gnupg/gnu-agent.conf"
```

## License

[CC0](LICENSE.md) - Public Domain
