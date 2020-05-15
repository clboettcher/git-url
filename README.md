# Git URL

CLI tool that prints the remote URL of a file in a git repository.

Written in rust.

# Usage

```shell
Examples: 
	git-url         prints https://github.com/myuser/myrepo
	git-url file.rs prints https://github.com/myuser/myrepo/blob/master/file.rs

USAGE:
    git-url [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>      The path of the file to get the URL for. Relative to the current working directory. Must be a file -
                directories are not supported. Optional, if FILE is omitted, the base URL of the repo is printed.
    <REMOTE>    The name of the git remote to use [default: origin]
```

# Maintainer

Claudius Boettcher, <claudius.boettcher@qaware.de>.

# License

This software is provided under the MIT open source license, read the `LICENSE` file for details.
