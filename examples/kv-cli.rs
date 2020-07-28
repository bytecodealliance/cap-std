//! A simple key-value store that stores data in a project data directory.
//!
//! Keys are filesystem paths, which isn't a great idea in general, because
//! it bubbles up filesystem idiosyncracies such as the case sensitivity
//! scheme the filesystem uses, but it makes a simple illustration of the
//! `cap-std` API.

use cap_directories::ProjectDirs;
use std::{env::args, path::PathBuf, str};

fn main() -> anyhow::Result<()> {
    // Parse command-line arguments.
    let mut args = args();
    args.next(); // skip the program name
    let key: PathBuf = args.next().ok_or_else(usage)?.into();
    let value = args.next();
    if args.next().is_some() {
        return Err(usage());
    }

    // Obtain the `data_dir` for this program.
    let project_dirs = ProjectDirs::from(
        "com.example",
        "Example Organization",
        "Cap-std Key-Value CLI Example",
    )
    .ok_or_else(no_project_dirs)?;
    let mut data_dir = project_dirs.data_dir()?;

    if let Some(value) = value {
        // `kv-cli key value` -- set the value of `key` to `value`

        // If the key contains separators, create the directory.
        let parent = key.parent();
        let file_name = key.file_name().ok_or_else(need_filename)?;
        if let Some(parent) = parent {
            if !parent.as_os_str().is_empty() {
                data_dir.create_dir_all(parent)?;
                data_dir = data_dir.open_dir(parent)?;
            }
        }

        // Write the value.
        data_dir.write(file_name, value)?;
    } else {
        // `kv-cli key` -- get the value of `key` and print it.
        println!("{}", str::from_utf8(&data_dir.read(key)?)?);
    }

    Ok(())
}

fn usage() -> anyhow::Error {
    anyhow::Error::msg("usage: kv-cli <key> [<value>]")
}

fn need_filename() -> anyhow::Error {
    anyhow::Error::msg("kv-cli key must end with a filename component (and not `..`)")
}

fn no_project_dirs() -> anyhow::Error {
    anyhow::Error::msg("kv-cli requires a home directory")
}
