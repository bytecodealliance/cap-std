// Copied from https://doc.rust-lang.org/rust-by-example/std_misc/fs.html and
// adapted to use this crate instead.

use cap_std::fs::{Dir, OpenOptions};
use std::{io, io::prelude::*};
//use std::os::unix;
use std::path::Path;

// A simple implementation of `% cat path`
fn cat(dir: &mut Dir, path: &Path) -> io::Result<String> {
    let mut f = dir.open_file(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A simple implementation of `% echo s > path`
fn echo(s: &str, dir: &mut Dir, path: &Path) -> io::Result<()> {
    let mut f = dir.create_file(path)?;

    f.write_all(s.as_bytes())
}

// A simple implementation of `% touch path` (ignores existing files)
fn touch(dir: &mut Dir, path: &Path) -> io::Result<()> {
    match dir.open_file_with(path, OpenOptions::new().create(true).write(true)) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    let mut cwd = Dir::from_std_file(std::fs::File::open(".").expect("!"));

    println!("`mkdir a`");

    // Create a directory, returns `io::Result<()>`
    match cwd.create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("`echo hello > a/b.txt`");
    // The previous match can be simplified using the `unwrap_or_else` method
    echo("hello", &mut cwd, &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
    // Recursively create a directory, returns `io::Result<()>`
    cwd.create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&mut cwd, &Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    // Create a symbolic link, returns `io::Result<()>`
    if cfg!(target_family = "unix") {
        cwd.symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&mut cwd, &Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    match cwd.read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().0);
            }
        }
    }

    println!("`rm a/c/e.txt`");
    // Remove a file, returns `io::Result<()>`
    cwd.remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");
    // Remove an empty directory, returns `io::Result<()>`
    cwd.remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
