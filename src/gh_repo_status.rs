
//!  verify if CARGO_PKG_VERSION matches Cargo.toml version currently on GitHub

///
///  Cargo.toml
///
///  [dependencies]
///  reqwest = { version = "0.11", features = ["blocking"] }
///  
///  Basic usage:
///
///  ```
///  mod gh_status;
///
///  gh_repo_status::check_version()
///      .expect("check_version error");
///  ```
///

use std::env;
use std::io::Read;

pub fn check_version() -> Result<(), Box<dyn std::error::Error>> {

    let url = format!("https://raw.githubusercontent.com/mwrietz/{}/main/Cargo.toml", get_prog_name());

    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    // split body into vector of lines
    let lines: Vec<&str> = body.split("\n").collect();

    // find version in GitHub Cargo.toml
    let mut github_version = String::new();
    for line in lines {
        if line.starts_with("version") {
            github_version = line
                .replace("\"", "")
                .replace(" ", "")
                .replace("version=", "");
            break;
        }
    }

    let local_version = env!("CARGO_PKG_VERSION");

    if local_version != github_version {
        println!();
        println!("The local version of '{}' is different than the GitHub version.", get_prog_name());
        println!("    Local version  = {}", local_version);
        println!("    GitHub version = {}", github_version);
        if local_version < github_version.as_str() {
            println!("The GitHub version is newer.  Consider upgrading to the newer version.");
        } else {
            println!("The GitHub version is older.  Consider a commit.");
        }
        println!();
    }

    Ok(())
}

fn get_prog_name() -> String {
    let prog_name = env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    prog_name
}
