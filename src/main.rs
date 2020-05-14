use std::{env, io};
use std::process::{Command, exit, Output};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_option: Option<String> = if args.len() == 2 { Some(String::from(&args[1])) } else { None };
    let remote = if args.len() == 3 { String::from(&args[2]) } else { String::from("origin") };
    run(path_option, remote);
}

fn run(path_option : Option<String>, remote : String) {
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
            let relative_path = get_relative_path(path.as_str())
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