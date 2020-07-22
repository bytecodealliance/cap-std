#![no_main]

#[macro_use]
extern crate libfuzzer_sys;

use arbitrary::Arbitrary;
use cap_primitives::fs::{DirOptions, FollowSymlinks, OpenOptions};
use std::{fs, path::PathBuf};
use tempfile::tempdir;

// TODO: NUL, SP, invalid UTF-8, non-normalized?
#[derive(Arbitrary, Debug)]
enum PathToken {
    Dot,
    DotDot,
    A,
    B,
    AB,
    BigA,
    Clone,
    Pop,
}

#[derive(Arbitrary, Debug)]
enum Operation {
    Create(usize, usize, usize),
    Open(usize, usize, OpenOptions, usize),
    Stat(usize, usize, FollowSymlinks),
    Mkdir(usize, usize, DirOptions),
    Canonicalize(usize, usize),
    Link(usize, usize, usize, usize),
    Readlink(usize, usize),
    Rename(usize, usize, usize, usize),
    Symlink(usize, usize, usize),
    Unlink(usize, usize),
    Rmdir(usize, usize),
    ReadDir(usize, usize),
    RemoveDirAll(usize, usize),
}

#[derive(Arbitrary, Debug)]
struct Plan {
    tokens: Vec<PathToken>,
    ops: Vec<Operation>,
}

impl Plan {
    fn execute(&self, files: &mut [fs::File], paths: &[PathBuf]) {
        for op in &self.ops {
            match op {
                Operation::Create(dirno, path, fileno) => {
                    if let Ok(file) = cap_primitives::fs::open(
                        &files[*dirno % files.len()],
                        &paths[*path % paths.len()],
                        OpenOptions::new().create(true).write(true),
                    ) {
                        files[*fileno % files.len()] = file;
                    }
                }
                Operation::Open(dirno, path, options, fileno) => {
                    if let Ok(file) = cap_primitives::fs::open(
                        &files[*dirno % files.len()],
                        &paths[*path % paths.len()],
                        options,
                    ) {
                        files[*fileno % files.len()] = file;
                    }
                }
                Operation::Stat(dirno, path, follow) => {
                    cap_primitives::fs::stat(
                        &files[*dirno % files.len()],
                        &paths[*path % paths.len()],
                        *follow,
                    )
                    .ok();
                }
                Operation::Mkdir(dirno, path, options) => {
                    cap_primitives::fs::mkdir(
                        &files[*dirno % files.len()],
                        &paths[*path % paths.len()],
                        options,
                    )
                    .ok();
                }
                Operation::Canonicalize(dirno, path) => {
                    cap_primitives::fs::canonicalize(
                        &files[*dirno % files.len()],
                        &paths[*path % paths.len()],
                    )
                    .ok();
                }
                Operation::Link(old_dirno, old_path, new_dirno, new_path) => {
                    cap_primitives::fs::link(
                        &files[*old_dirno % files.len()],
                        &paths[*old_path % paths.len()],
                        &files[*new_dirno % files.len()],
                        &paths[*new_path % paths.len()],
                    )
                    .ok();
                }
                Operation::Readlink(dirno, path) => {
                    cap_primitives::fs::readlink(
                        &files[*dirno % files.len()],
                        &paths[*path % paths.len()],
                    )
                    .ok();
                }
                Operation::Rename(old_dirno, old_path, new_dirno, new_path) => {
                    cap_primitives::fs::rename(
                        &files[*old_dirno % files.len()],
                        &paths[*old_path % paths.len()],
                        &files[*new_dirno % files.len()],
                        &paths[*new_path % paths.len()],
                    )
                    .ok();
                }
                Operation::Symlink(old_path, new_dirno, new_path) => {
                    cap_primitives::fs::symlink(
                        &paths[*old_path % paths.len()],
                        &files[*new_dirno % files.len()],
                        &paths[*new_path % paths.len()],
                    )
                    .ok();
                }
                Operation::Unlink(dirno, path) => {
                    cap_primitives::fs::unlink(
                        &files[*dirno % files.len()],
                        &paths[*path % paths.len()],
                    )
                    .ok();
                }
                Operation::Rmdir(dirno, path) => {
                    cap_primitives::fs::rmdir(
                        &files[*dirno % files.len()],
                        &paths[*path % paths.len()],
                    )
                    .ok();
                }
                Operation::ReadDir(dirno, path) => {
                    let path = &paths[*path % paths.len()];
                    if let Ok(read_dir) =
                        cap_primitives::fs::read_dir(&files[*dirno % files.len()], path)
                    {
                        for child in read_dir {
                            if let Ok(child) = child {
                                cap_primitives::fs::stat(
                                    &files[*dirno % files.len()],
                                    &path.join(child.file_name()),
                                    FollowSymlinks::Yes,
                                )
                                .ok();
                            }
                        }
                    }
                }
                Operation::RemoveDirAll(dirno, path) => {
                    cap_primitives::fs::remove_dir_all(
                        &files[*dirno % files.len()],
                        &paths[*path % paths.len()],
                    )
                    .ok();
                }
            }
        }
    }
}

fuzz_target!(|plan: Plan| {
    let mut paths = Vec::new();

    paths.push(PathBuf::new());

    for c in &plan.tokens {
        match c {
            PathToken::A => paths.last_mut().unwrap().push("a"),
            PathToken::B => paths.last_mut().unwrap().push("b"),
            PathToken::AB => paths.last_mut().unwrap().push("ab"),
            PathToken::BigA => paths.last_mut().unwrap().push("A"),
            PathToken::Dot => paths.last_mut().unwrap().push("."),
            PathToken::DotDot => paths.last_mut().unwrap().push(".."),
            PathToken::Clone => paths.push(paths.last().unwrap().clone()),
            PathToken::Pop => {
                paths.last_mut().unwrap().pop();
            }
        }
    }

    let tmp = tempdir().unwrap();
    fs::create_dir(tmp.path().join("dir")).unwrap();

    let dir = fs::File::open(tmp.path().join("dir")).unwrap();

    let mut files = (0..8).map(|_| dir.try_clone().unwrap()).collect::<Vec<_>>();

    plan.execute(&mut files, &mut paths);
});
