//! Microbenchmarks for `cap_std`. These have pathological behavior and are
//! not representative of typical real-world use cases.

#![feature(test)]

extern crate cap_tempfile;
extern crate tempfile;
extern crate test;

use std::{fs, path::PathBuf};

#[bench]
fn nested_directories_open(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    let mut path = PathBuf::new();
    for _ in 0..256 {
        path.push("abc");
    }
    dir.create_dir_all(&path).unwrap();

    b.iter(|| {
        let _file = dir.open(&path).unwrap();
    });
}

#[bench]
fn nested_directories_open_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    let mut path = PathBuf::new();
    for _ in 0..256 {
        path.push("abc");
    }
    fs::create_dir_all(dir.path().join(&path)).unwrap();

    b.iter(|| {
        let _file = fs::File::open(dir.path().join(&path)).unwrap();
    });
}

#[bench]
fn nested_directories_metadata(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    let mut path = PathBuf::new();
    for _ in 0..256 {
        path.push("abc");
    }
    dir.create_dir_all(&path).unwrap();

    b.iter(|| {
        let _metadata = dir.metadata(&path).unwrap();
    });
}

#[bench]
fn nested_directories_metadata_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    let mut path = PathBuf::new();
    path.push(dir);
    for _ in 0..256 {
        path.push("abc");
    }
    fs::create_dir_all(&path).unwrap();

    b.iter(|| {
        let _metadata = fs::metadata(&path).unwrap();
    });
}

#[bench]
fn nested_directories_canonicalize(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    let mut path = PathBuf::new();
    for _ in 0..256 {
        path.push("abc");
    }
    dir.create_dir_all(&path).unwrap();

    b.iter(|| {
        let _canonical = dir.canonicalize(&path).unwrap();
    });
}

#[bench]
fn nested_directories_canonicalize_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    let mut path = PathBuf::new();
    for _ in 0..256 {
        path.push("abc");
    }
    fs::create_dir_all(dir.path().join(&path)).unwrap();

    b.iter(|| {
        let _canonical = fs::canonicalize(dir.path().join(&path)).unwrap();
    });
}

#[cfg(unix)]
#[bench]
fn nested_directories_readlink(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    let mut path = PathBuf::new();
    for _ in 0..256 {
        path.push("abc");
    }
    dir.create_dir_all(&path).unwrap();

    path.push("symlink");
    dir.symlink("source", &path).unwrap();

    b.iter(|| {
        let _destination = dir.read_link(&path).unwrap();
    });
}

#[cfg(unix)]
#[bench]
fn nested_directories_readlink_baseline(b: &mut test::Bencher) {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();

    let mut path = PathBuf::new();
    path.push(dir);
    for _ in 0..256 {
        path.push("abc");
    }
    fs::create_dir_all(&path).unwrap();

    path.push("symlink");
    symlink("source", &path).unwrap();

    b.iter(|| {
        let _destination = fs::read_link(&path).unwrap();
    });
}

#[bench]
fn curdir(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    let mut path = PathBuf::new();
    for _ in 0..256 {
        path.push(".");
    }
    path.push("def");
    dir.create("def").unwrap();

    b.iter(|| {
        let _file = dir.open(&path).unwrap();
    });
}

#[bench]
fn curdir_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    let mut path = PathBuf::new();
    path.push(&dir);
    for _ in 0..256 {
        path.push(".");
    }
    path.push("def");
    fs::File::create(dir.path().join("def")).unwrap();

    b.iter(|| {
        let _file = fs::File::open(&path).unwrap();
    });
}

#[bench]
fn parentdir(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    let mut path = PathBuf::new();
    for _ in 0..256 {
        path.push("abc");
    }
    dir.create_dir_all(&path).unwrap();

    for _ in 0..256 {
        path.push("..");
    }
    path.push("def");
    dir.create("def").unwrap();

    b.iter(|| {
        let _file = dir.open(&path).unwrap();
    });
}

#[bench]
fn parentdir_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    let mut path = PathBuf::new();
    path.push(&dir);
    for _ in 0..256 {
        path.push("abc");
    }
    fs::create_dir_all(&path).unwrap();

    for _ in 0..256 {
        path.push("..");
    }
    path.push("def");
    fs::File::create(dir.path().join("def")).unwrap();

    b.iter(|| {
        let _file = fs::File::open(&path).unwrap();
    });
}

#[bench]
fn directory_iteration(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    for i in 0..256 {
        dir.create(i.to_string()).unwrap();
    }

    b.iter(|| {
        for entry in dir.entries().unwrap() {
            let _file = dir.open(entry.unwrap().file_name()).unwrap();
        }
    });
}

#[bench]
fn directory_iteration_fast(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    for i in 0..256 {
        dir.create(i.to_string()).unwrap();
    }

    b.iter(|| {
        for entry in dir.entries().unwrap() {
            let _file = entry.unwrap().open().unwrap();
        }
    });
}

#[bench]
fn directory_iteration_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    for i in 0..256 {
        fs::File::create(dir.path().join(i.to_string())).unwrap();
    }

    b.iter(|| {
        for entry in fs::read_dir(&dir).unwrap() {
            let _file = fs::File::open(dir.path().join(entry.unwrap().file_name())).unwrap();
        }
    });
}

#[cfg(unix)]
#[bench]
fn symlink_chasing_open(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    dir.create("0").unwrap();
    for i in 0..32 {
        dir.symlink(i.to_string(), (i + 1).to_string()).unwrap();
    }

    let name = "32";
    b.iter(|| {
        let _file = dir.open(name).unwrap();
    });
}

#[cfg(unix)]
#[bench]
fn symlink_chasing_open_baseline(b: &mut test::Bencher) {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();

    fs::File::create(dir.path().join("0")).unwrap();
    for i in 0..32 {
        symlink(
            dir.path().join(i.to_string()),
            dir.path().join((i + 1).to_string()),
        )
        .unwrap();
    }

    let name = dir.path().join("32");
    b.iter(|| {
        let _file = fs::File::open(&name).unwrap();
    });
}

#[cfg(unix)]
#[bench]
fn symlink_chasing_metadata(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    dir.create("0").unwrap();
    for i in 0..32 {
        dir.symlink(i.to_string(), (i + 1).to_string()).unwrap();
    }

    let name = "32";
    b.iter(|| {
        let _metadata = dir.metadata(name).unwrap();
    });
}

#[cfg(unix)]
#[bench]
fn symlink_chasing_metadata_baseline(b: &mut test::Bencher) {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();

    fs::File::create(dir.path().join("0")).unwrap();
    for i in 0..32 {
        symlink(
            dir.path().join(i.to_string()),
            dir.path().join((i + 1).to_string()),
        )
        .unwrap();
    }

    let name = dir.path().join("32");
    b.iter(|| {
        let _metadata = fs::metadata(&name).unwrap();
    });
}

#[cfg(unix)]
#[bench]
fn symlink_chasing_canonicalize(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    dir.create("0").unwrap();
    for i in 0..32 {
        dir.symlink(i.to_string(), (i + 1).to_string()).unwrap();
    }

    let name = "32";
    b.iter(|| {
        let _canonical = dir.canonicalize(name).unwrap();
    });
}

#[cfg(unix)]
#[bench]
fn symlink_chasing_canonicalize_baseline(b: &mut test::Bencher) {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();

    fs::File::create(dir.path().join("0")).unwrap();
    for i in 0..32 {
        symlink(
            dir.path().join(i.to_string()),
            dir.path().join((i + 1).to_string()),
        )
        .unwrap();
    }

    let name = dir.path().join("32");
    b.iter(|| {
        let _canonical = fs::canonicalize(&name).unwrap();
    });
}

#[cfg(any(not(windows), feature = "windows_file_type_ext"))]
#[bench]
fn recursive_create_delete(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    let mut path = PathBuf::new();
    for _ in 0..256 {
        path.push("abc");
    }

    b.iter(|| {
        dir.create_dir_all(&path).unwrap();
        dir.remove_dir_all(&path).unwrap();
    });
}

#[cfg(any(not(windows), feature = "windows_file_type_ext"))]
#[bench]
fn recursive_create_delete_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    let mut path = PathBuf::new();
    path.push(dir);
    for _ in 0..256 {
        path.push("abc");
    }

    b.iter(|| {
        fs::create_dir_all(&path).unwrap();
        fs::remove_dir_all(&path).unwrap();
    });
}

#[bench]
fn copy_4b(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    dir.write("file", &vec![1u8; 0x4]).unwrap();

    b.iter(|| {
        dir.copy("file", &dir, "copy").unwrap();
    });
}

#[bench]
fn copy_4b_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    let file = dir.path().join("file");
    let copy = dir.path().join("copy");
    fs::write(&file, &vec![1u8; 0x4]).unwrap();

    b.iter(|| {
        fs::copy(&file, &copy).unwrap();
    });
}

#[bench]
fn copy_4k(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    dir.write("file", &vec![1u8; 0x1000]).unwrap();

    b.iter(|| {
        dir.copy("file", &dir, "copy").unwrap();
    });
}

#[bench]
fn copy_4k_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    let file = dir.path().join("file");
    let copy = dir.path().join("copy");
    fs::write(&file, &vec![1u8; 0x1000]).unwrap();

    b.iter(|| {
        fs::copy(&file, &copy).unwrap();
    });
}

#[bench]
fn copy_4m(b: &mut test::Bencher) {
    let dir = unsafe { cap_tempfile::tempdir().unwrap() };

    dir.write("file", &vec![1u8; 0x400000]).unwrap();

    b.iter(|| {
        dir.copy("file", &dir, "copy").unwrap();
    });
}

#[bench]
fn copy_4m_baseline(b: &mut test::Bencher) {
    let dir = tempfile::tempdir().unwrap();

    let file = dir.path().join("file");
    let copy = dir.path().join("copy");
    fs::write(&file, &vec![1u8; 0x400000]).unwrap();

    b.iter(|| {
        fs::copy(&file, &copy).unwrap();
    });
}
