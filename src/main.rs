// gh='xdg-open ${$(git remote get-url origin)//.git/}'

use std::process::{Command, exit};

// Not in git repo -> print err and exit
// No arguments -> result of git remote get-url origin

fn main() {
    let out = Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .output()
        .expect("git command failed");

    // println!("status: {}", out.status);
    // println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&out.stderr));
    if !out.status.success() {
        eprint!("{} {}", "Could not get url:", String::from_utf8_lossy(&out.stderr));
        exit(out.status.code().unwrap_or(1));
    }

    println!("{}", String::from_utf8_lossy(&out.stdout).trim_end().to_owned());
}