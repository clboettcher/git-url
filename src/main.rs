// gh='xdg-open ${$(git remote get-url origin)//.git/}'

use std::{env, io};
use std::process::{Command, exit, Output};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: Option<String> = if args.len() == 2 { Some(String::from(&args[1])) } else { None };
    let remote = if args.len() == 3 { String::from(&args[2]) } else { String::from("origin") };

    let out = get_git_url(&remote);

    // println!("status: {}", out.status);
    // println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&out.stderr));
    if !out.status.success() {
        eprint!("{} {}", "Could not get url:", String::from_utf8_lossy(&out.stderr));
        exit(out.status.code().unwrap_or(1));
    }

    let remote_url_raw = String::from_utf8_lossy(&out.stdout).trim_end().to_owned();
    let remote_url = &remote_url_raw[..remote_url_raw.len() - 4]; // remove ".git"

    if path.is_some() {
        let branch = get_branch().expect("could not get branch");
        let relative_path = get_relative_path(path.unwrap().as_str())
            .expect("could not get relative path");

        // echo $remote_url/blob/$branch/$relative_file_path
        println!("{remote_url}/blob/{branch}/{relative_path}",
                 remote_url = remote_url,
                 branch = branch,
                 relative_path = relative_path
        );
    } else {
        println!("{}", remote_url);
    }
}

fn get_git_url(remote: &str) -> Output {
    return Command::new("git")
        .args(&["remote", "get-url", remote])
        .output()
        .expect("git command failed");
}

// branch=$(git rev-parse --abbrev-ref HEAD)
fn get_branch() -> io::Result<String> {
    run_git(vec!["rev-parse", "--abbrev-ref", "HEAD"])
}

// relative_file_path=$(git ls-files --full-name $1)
fn get_relative_path(path: &str) -> io::Result<String> {
    run_git(vec!["ls-files", "--full-name", path])
}

fn run_git(args: Vec<&str>) -> io::Result<String> {
    let output = Command::new("git").args(&args).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim_end().to_owned())
}