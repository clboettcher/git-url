extern crate clap;

use std::io;
use std::process::{Command, exit, Output};

use clap::{App, Arg};

fn main() {
    let matches = App::new("Git URL")
        .version("0.1.0")
        .author("Claudius Boettcher <claudius.boettcher@qaware.de>")
        .about("Prints the remote URL of a file in a git repository.\n\
            Examples: \n\
            \tgit-url         prints https://github.com/myuser/myrepo\n\
            \tgit-url file.rs prints https://github.com/myuser/myrepo/blob/master/file.rs"
        )
        .arg(Arg::with_name("path")
            .required(false)
            .index(1)
            .value_name("PATH")
            .help("The path to the file to get the URL for")
            .takes_value(true))
        .arg(Arg::with_name("remote")
            .required(false)
            .index(2)
            .default_value("origin")
            .value_name("REMOTE")
            .help("The name of the git remote to use")
            .takes_value(true))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let path_option = matches.value_of("path");
    let remote = matches.value_of("remote").unwrap();
    run(path_option, remote);
}

fn run(path_option: Option<&str>, remote: &str) {
    let output = get_git_url(&remote).expect("failed to get git url");

    if !output.status.success() {
        eprint!("{} {}", "Could not get url:", String::from_utf8_lossy(&output.stderr));
        exit(output.status.code().unwrap_or(1));
    }

    let remote_url_raw = get_trimmed_stdout(output);
    let remote_url = &remote_url_raw[..remote_url_raw.len() - 4]; // remove ".git"

    match path_option {
        Some(path) => {
            let branch = get_branch().expect("could not get branch");
            let relative_path = get_relative_path(path)
                .expect("could not get relative path");

            // {remote_url}/blob/{branch}/{relative_path}
            println!("{remote_url}/blob/{branch}/{relative_path}",
                     remote_url = remote_url,
                     branch = branch,
                     relative_path = relative_path
            );
        }
        None => println!("{}", remote_url)
    }
}

// git remote get-url {remote}
fn get_git_url(remote: &str) -> io::Result<Output> {
    run_git(vec!["remote", "get-url", remote])
}

// git rev-parse --abbrev-ref HEAD
fn get_branch() -> io::Result<String> {
    run_cmd_get_stdout(vec!["rev-parse", "--abbrev-ref", "HEAD"])
}

// git ls-files --full-name {path}
fn get_relative_path(path: &str) -> io::Result<String> {
    run_cmd_get_stdout(vec!["ls-files", "--full-name", path])
}

fn run_cmd_get_stdout(args: Vec<&str>) -> io::Result<String> {
    run_git(args).map(get_trimmed_stdout)
}

fn run_git(args: Vec<&str>) -> io::Result<Output> {
    let output = Command::new("git")
        .args(&args)
        .output()?;

    Ok(output)
}

fn get_trimmed_stdout(output: Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim_end().to_owned()
}