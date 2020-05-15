# Git URL

CLI tool that prints the remote URL of a file in a git repository.

Written in rust.

# Usage

Get repo url

```shell
~/Projekte/rust
❯ git-url                    
https://github.com/rust-lang/rust
```

Get URL of file in repo

```shell
~/Projekte/rust
❯ git-url src/etc/ctags.rust 
https://github.com/rust-lang/rust/blob/master/src/etc/ctags.rust
```

Get URL of file in repo from different remote (default is origin)

```shell
~/Projekte/rust
❯ git-url src/etc/ctags.rust ckw        
https://github.com/ckw/rust/blob/master/src/etc/ctags.rust
```

`--help` output:

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
