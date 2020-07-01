// This file is derived from stc/libstd/fs.rs in the Rust repository at revision
// 50fc24d8a172a853b5dfe40702d6550e3b8562ba.
//
// This is just the contents of the `tests` module, ported to use `cap_std`.

mod sys_common;

use std::io::prelude::*;

use cap_std::fs::{self, OpenOptions};
use std::{
    io::{ErrorKind, SeekFrom},
    path::{Path, PathBuf},
    str,
};
/*use std::thread;*/
use sys_common::io::tmpdir;

use rand::{rngs::StdRng, RngCore, SeedableRng};

/*
#[cfg(unix)]
use cap_std::os::unix::fs::symlink as symlink_dir;
#[cfg(unix)]
use cap_std::os::unix::fs::symlink as symlink_file;
#[cfg(unix)]
use cap_std::os::unix::fs::symlink as symlink_junction;
#[cfg(windows)]
use cap_std::os::windows::fs::{symlink_dir, symlink_file};
#[cfg(windows)]
use cap_std::sys::fs::symlink_junction;
*/

/*
// Several test fail on windows if the user does not have permission to
// create symlinks (the `SeCreateSymbolicLinkPrivilege`). Instead of
// disabling these test on Windows, use this function to test whether we
// have permission, and return otherwise. This way, we still don't run these
// tests most of the time, but at least we do if the user has the right
// permissions.
pub fn got_symlink_permission(tmpdir: &TempDir) -> bool {
    if cfg!(unix) {
        return true;
    }
    let link = "some_hopefully_unique_link_name";

    match symlink_file(r"nonexisting_target", link) {
        Ok(_) => true,
        // ERROR_PRIVILEGE_NOT_HELD = 1314
        Err(ref err) if err.raw_os_error() == Some(1314) => false,
        Err(_) => true,
    }
}
*/

#[test]
fn file_test_io_smoke_test() {
    let message = "it's alright. have a good time";
    let tmpdir = tmpdir();
    let filename = "file_rt_io_file_test.txt";
    {
        let mut write_stream = check!(tmpdir.create_file(filename));
        check!(write_stream.write(message.as_bytes()));
    }
    {
        let mut read_stream = check!(tmpdir.open_file(filename));
        let mut read_buf = [0; 1028];
        let read_str = match check!(read_stream.read(&mut read_buf)) {
            0 => panic!("shouldn't happen"),
            n => str::from_utf8(&read_buf[..n]).unwrap().to_string(),
        };
        assert_eq!(read_str, message);
    }
    check!(tmpdir.remove_file(filename));
}

#[test]
fn invalid_path_raises() {
    let tmpdir = tmpdir();
    let filename = "file_that_does_not_exist.txt";
    let result = tmpdir.open_file(filename);

    #[cfg(all(unix, not(target_os = "vxworks")))]
    error!(result, "No such file or directory");
    #[cfg(target_os = "vxworks")]
    error!(result, "no such file or directory");
    #[cfg(windows)]
    error!(result, 2); // ERROR_FILE_NOT_FOUND
}

#[test]
fn file_test_iounlinking_invalid_path_should_raise_condition() {
    let tmpdir = tmpdir();
    let filename = "file_another_file_that_does_not_exist.txt";

    let result = tmpdir.remove_file(filename);

    #[cfg(all(unix, not(target_os = "vxworks")))]
    error!(result, "No such file or directory");
    #[cfg(target_os = "vxworks")]
    error!(result, "no such file or directory");
    #[cfg(windows)]
    error!(result, 2); // ERROR_FILE_NOT_FOUND
}

#[test]
fn file_test_io_non_positional_read() {
    let message: &str = "ten-four";
    let mut read_mem = [0; 8];
    let tmpdir = tmpdir();
    let filename = "file_rt_io_file_test_positional.txt";
    {
        let mut rw_stream = check!(tmpdir.create_file(filename));
        check!(rw_stream.write(message.as_bytes()));
    }
    {
        let mut read_stream = check!(tmpdir.open_file(filename));
        {
            let read_buf = &mut read_mem[0..4];
            check!(read_stream.read(read_buf));
        }
        {
            let read_buf = &mut read_mem[4..8];
            check!(read_stream.read(read_buf));
        }
    }
    check!(tmpdir.remove_file(filename));
    let read_str = str::from_utf8(&read_mem).unwrap();
    assert_eq!(read_str, message);
}

#[test]
fn file_test_io_seek_and_tell_smoke_test() {
    let message = "ten-four";
    let mut read_mem = [0; 4];
    let set_cursor = 4 as u64;
    let tell_pos_pre_read;
    let tell_pos_post_read;
    let tmpdir = tmpdir();
    let filename = "file_rt_io_file_test_seeking.txt";
    {
        let mut rw_stream = check!(tmpdir.create_file(filename));
        check!(rw_stream.write(message.as_bytes()));
    }
    {
        let mut read_stream = check!(tmpdir.open_file(filename));
        check!(read_stream.seek(SeekFrom::Start(set_cursor)));
        tell_pos_pre_read = check!(read_stream.seek(SeekFrom::Current(0)));
        check!(read_stream.read(&mut read_mem));
        tell_pos_post_read = check!(read_stream.seek(SeekFrom::Current(0)));
    }
    check!(tmpdir.remove_file(filename));
    let read_str = str::from_utf8(&read_mem).unwrap();
    assert_eq!(read_str, &message[4..8]);
    assert_eq!(tell_pos_pre_read, set_cursor);
    assert_eq!(tell_pos_post_read, message.len() as u64);
}

#[test]
fn file_test_io_seek_and_write() {
    let initial_msg = "food-is-yummy";
    let overwrite_msg = "-the-bar!!";
    let final_msg = "foo-the-bar!!";
    let seek_idx = 3;
    let mut read_mem = [0; 13];
    let tmpdir = tmpdir();
    let filename = "file_rt_io_file_test_seek_and_write.txt";
    {
        let mut rw_stream = check!(tmpdir.create_file(filename));
        check!(rw_stream.write(initial_msg.as_bytes()));
        check!(rw_stream.seek(SeekFrom::Start(seek_idx)));
        check!(rw_stream.write(overwrite_msg.as_bytes()));
    }
    {
        let mut read_stream = check!(tmpdir.open_file(filename));
        check!(read_stream.read(&mut read_mem));
    }
    check!(tmpdir.remove_file(filename));
    let read_str = str::from_utf8(&read_mem).unwrap();
    assert!(read_str == final_msg);
}

#[test]
fn file_test_io_seek_shakedown() {
    //                   01234567890123
    let initial_msg = "qwer-asdf-zxcv";
    let chunk_one: &str = "qwer";
    let chunk_two: &str = "asdf";
    let chunk_three: &str = "zxcv";
    let mut read_mem = [0; 4];
    let tmpdir = tmpdir();
    let filename = "file_rt_io_file_test_seek_shakedown.txt";
    {
        let mut rw_stream = check!(tmpdir.create_file(filename));
        check!(rw_stream.write(initial_msg.as_bytes()));
    }
    {
        let mut read_stream = check!(tmpdir.open_file(filename));

        check!(read_stream.seek(SeekFrom::End(-4)));
        check!(read_stream.read(&mut read_mem));
        assert_eq!(str::from_utf8(&read_mem).unwrap(), chunk_three);

        check!(read_stream.seek(SeekFrom::Current(-9)));
        check!(read_stream.read(&mut read_mem));
        assert_eq!(str::from_utf8(&read_mem).unwrap(), chunk_two);

        check!(read_stream.seek(SeekFrom::Start(0)));
        check!(read_stream.read(&mut read_mem));
        assert_eq!(str::from_utf8(&read_mem).unwrap(), chunk_one);
    }
    check!(tmpdir.remove_file(filename));
}

#[test]
fn file_test_io_eof() {
    let tmpdir = tmpdir();
    let filename = "file_rt_io_file_test_eof.txt";
    let mut buf = [0; 256];
    {
        let oo = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .clone();
        let mut rw = check!(tmpdir.open_file_with(filename, &oo));
        assert_eq!(check!(rw.read(&mut buf)), 0);
        assert_eq!(check!(rw.read(&mut buf)), 0);
    }
    check!(tmpdir.remove_file(filename));
}

#[test]
#[cfg(unix)]
fn file_test_io_read_write_at() {
    use std::os::unix::fs::FileExt;

    let tmpdir = tmpdir();
    let filename = "file_rt_io_file_test_read_write_at.txt";
    let mut buf = [0; 256];
    let write1 = "asdf";
    let write2 = "qwer-";
    let write3 = "-zxcv";
    let content = "qwer-asdf-zxcv";
    {
        let oo = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .clone();
        let mut rw = check!(tmpdir.open_file_with(filename, &oo));
        assert_eq!(check!(rw.write_at(write1.as_bytes(), 5)), write1.len());
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 0);
        assert_eq!(check!(rw.read_at(&mut buf, 5)), write1.len());
        assert_eq!(str::from_utf8(&buf[..write1.len()]), Ok(write1));
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 0);
        assert_eq!(
            check!(rw.read_at(&mut buf[..write2.len()], 0)),
            write2.len()
        );
        assert_eq!(str::from_utf8(&buf[..write2.len()]), Ok("\0\0\0\0\0"));
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 0);
        assert_eq!(check!(rw.write(write2.as_bytes())), write2.len());
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 5);
        assert_eq!(check!(rw.read(&mut buf)), write1.len());
        assert_eq!(str::from_utf8(&buf[..write1.len()]), Ok(write1));
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 9);
        assert_eq!(
            check!(rw.read_at(&mut buf[..write2.len()], 0)),
            write2.len()
        );
        assert_eq!(str::from_utf8(&buf[..write2.len()]), Ok(write2));
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 9);
        assert_eq!(check!(rw.write_at(write3.as_bytes(), 9)), write3.len());
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 9);
    }
    {
        let mut read = check!(tmpdir.open_file(filename));
        assert_eq!(check!(read.read_at(&mut buf, 0)), content.len());
        assert_eq!(str::from_utf8(&buf[..content.len()]), Ok(content));
        assert_eq!(check!(read.seek(SeekFrom::Current(0))), 0);
        assert_eq!(check!(read.seek(SeekFrom::End(-5))), 9);
        assert_eq!(check!(read.read_at(&mut buf, 0)), content.len());
        assert_eq!(str::from_utf8(&buf[..content.len()]), Ok(content));
        assert_eq!(check!(read.seek(SeekFrom::Current(0))), 9);
        assert_eq!(check!(read.read(&mut buf)), write3.len());
        assert_eq!(str::from_utf8(&buf[..write3.len()]), Ok(write3));
        assert_eq!(check!(read.seek(SeekFrom::Current(0))), 14);
        assert_eq!(check!(read.read_at(&mut buf, 0)), content.len());
        assert_eq!(str::from_utf8(&buf[..content.len()]), Ok(content));
        assert_eq!(check!(read.seek(SeekFrom::Current(0))), 14);
        assert_eq!(check!(read.read_at(&mut buf, 14)), 0);
        assert_eq!(check!(read.read_at(&mut buf, 15)), 0);
        assert_eq!(check!(read.seek(SeekFrom::Current(0))), 14);
    }
    check!(tmpdir.remove_file(filename));
}

#[test]
#[ignore] // not implemented in cap-std yet
#[cfg(unix)]
fn set_get_unix_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let tmpdir = tmpdir();
    let filename = "set_get_unix_permissions";
    check!(tmpdir.create_dir(filename));
    let mask = 0o7777;

    check!(tmpdir.set_permissions(filename, fs::Permissions::from_mode(0)));
    let metadata0 = check!(tmpdir.metadata(filename));
    assert_eq!(mask & metadata0.permissions().mode(), 0);

    check!(tmpdir.set_permissions(filename, fs::Permissions::from_mode(0o1777)));
    let metadata1 = check!(tmpdir.metadata(filename));
    #[cfg(all(unix, not(target_os = "vxworks")))]
    assert_eq!(mask & metadata1.permissions().mode(), 0o1777);
    #[cfg(target_os = "vxworks")]
    assert_eq!(mask & metadata1.permissions().mode(), 0o0777);
}

#[test]
#[cfg(windows)]
fn file_test_io_seek_read_write() {
    use std::os::windows::fs::FileExt;

    let tmpdir = tmpdir();
    let filename = "file_rt_io_file_test_seek_read_write.txt";
    let mut buf = [0; 256];
    let write1 = "asdf";
    let write2 = "qwer-";
    let write3 = "-zxcv";
    let content = "qwer-asdf-zxcv";
    {
        let oo = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .clone();
        let mut rw = check!(oo.open(filename));
        assert_eq!(check!(rw.seek_write(write1.as_bytes(), 5)), write1.len());
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 9);
        assert_eq!(check!(rw.seek_read(&mut buf, 5)), write1.len());
        assert_eq!(str::from_utf8(&buf[..write1.len()]), Ok(write1));
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 9);
        assert_eq!(check!(rw.seek(SeekFrom::Start(0))), 0);
        assert_eq!(check!(rw.write(write2.as_bytes())), write2.len());
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 5);
        assert_eq!(check!(rw.read(&mut buf)), write1.len());
        assert_eq!(str::from_utf8(&buf[..write1.len()]), Ok(write1));
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 9);
        assert_eq!(
            check!(rw.seek_read(&mut buf[..write2.len()], 0)),
            write2.len()
        );
        assert_eq!(str::from_utf8(&buf[..write2.len()]), Ok(write2));
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 5);
        assert_eq!(check!(rw.seek_write(write3.as_bytes(), 9)), write3.len());
        assert_eq!(check!(rw.seek(SeekFrom::Current(0))), 14);
    }
    {
        let mut read = check!(File::open(filename));
        assert_eq!(check!(read.seek_read(&mut buf, 0)), content.len());
        assert_eq!(str::from_utf8(&buf[..content.len()]), Ok(content));
        assert_eq!(check!(read.seek(SeekFrom::Current(0))), 14);
        assert_eq!(check!(read.seek(SeekFrom::End(-5))), 9);
        assert_eq!(check!(read.seek_read(&mut buf, 0)), content.len());
        assert_eq!(str::from_utf8(&buf[..content.len()]), Ok(content));
        assert_eq!(check!(read.seek(SeekFrom::Current(0))), 14);
        assert_eq!(check!(read.seek(SeekFrom::End(-5))), 9);
        assert_eq!(check!(read.read(&mut buf)), write3.len());
        assert_eq!(str::from_utf8(&buf[..write3.len()]), Ok(write3));
        assert_eq!(check!(read.seek(SeekFrom::Current(0))), 14);
        assert_eq!(check!(read.seek_read(&mut buf, 0)), content.len());
        assert_eq!(str::from_utf8(&buf[..content.len()]), Ok(content));
        assert_eq!(check!(read.seek(SeekFrom::Current(0))), 14);
        assert_eq!(check!(read.seek_read(&mut buf, 14)), 0);
        assert_eq!(check!(read.seek_read(&mut buf, 15)), 0);
    }
    check!(tmpdir.remove_file(filename));
}

/*
#[test]
fn file_test_stat_is_correct_on_is_file() {
    let tmpdir = tmpdir();
    let filename = "file_stat_correct_on_is_file.txt";
    {
        let mut opts = OpenOptions::new();
        let mut fs = check!(opts.read(true).write(true).create(true).open(filename));
        let msg = "hw";
        fs.write(msg.as_bytes()).unwrap();

        let fstat_res = check!(fs.metadata());
        assert!(fstat_res.is_file());
    }
    let stat_res_fn = check!(tmpdir.metadata(filename));
    assert!(stat_res_fn.is_file());
    let stat_res_meth = check!(filename.metadata());
    assert!(stat_res_meth.is_file());
    check!(tmpdir.remove_file(filename));
}

#[test]
fn file_test_stat_is_correct_on_is_dir() {
    let tmpdir = tmpdir();
    let filename = "file_stat_correct_on_is_dir";
    check!(fs::create_dir(filename));
    let stat_res_fn = check!(fs::metadata(filename));
    assert!(stat_res_fn.is_dir());
    let stat_res_meth = check!(filename.metadata());
    assert!(stat_res_meth.is_dir());
    check!(tmpdir.remove_dir(filename));
}

#[test]
fn file_test_fileinfo_false_when_checking_is_file_on_a_directory() {
    let tmpdir = tmpdir();
    let dir = "fileinfo_false_on_dir";
    check!(fs::create_dir(dir));
    assert!(!dir.is_file());
    check!(tmpdir.remove_dir(dir));
}

#[test]
fn file_test_fileinfo_check_exists_before_and_after_file_creation() {
    let tmpdir = tmpdir();
    let file = "fileinfo_check_exists_b_and_a.txt";
    check!(check!(tmpdir.create_file(file)).write(b"foo"));
    assert!(file.exists());
    check!(tmpdir.remove_file(file));
    assert!(!file.exists());
}

#[test]
fn file_test_directoryinfo_check_exists_before_and_after_mkdir() {
    let tmpdir = tmpdir();
    let dir = "before_and_after_dir";
    assert!(!dir.exists());
    check!(fs::create_dir(dir));
    assert!(dir.exists());
    assert!(dir.is_dir());
    check!(tmpdir.remove_dir(dir));
    assert!(!dir.exists());
}
*/

#[test]
#[ignore] // `read_dir` not yet implemented in cap-std
fn file_test_directoryinfo_readdir() {
    let tmpdir = tmpdir();
    let dir = "di_readdir";
    check!(tmpdir.create_dir(dir));
    let prefix = "foo";
    for n in 0..3 {
        let f = format!("{}.txt", n);
        let mut w = check!(tmpdir.create_file(&f));
        let msg_str = format!("{}{}", prefix, n.to_string());
        let msg = msg_str.as_bytes();
        check!(w.write(msg));
    }
    let files = check!(tmpdir.read_dir(dir));
    let mut mem = [0; 4];
    for f in files {
        let f = f.unwrap().file_name();
        {
            check!(check!(tmpdir.open_file(&f)).read(&mut mem));
            let read_str = str::from_utf8(&mem).unwrap();
            let expected = format!("{}{}", prefix, f.to_str().unwrap());
            assert_eq!(expected, read_str);
        }
        check!(tmpdir.remove_file(&f));
    }
    check!(tmpdir.remove_dir(dir));
}

#[test]
fn file_create_new_already_exists_error() {
    let tmpdir = tmpdir();
    let file = "file_create_new_error_exists";
    check!(tmpdir.create_file(file));
    let e = tmpdir
        .open_file_with(file, &fs::OpenOptions::new().write(true).create_new(true))
        .unwrap_err();
    assert_eq!(e.kind(), ErrorKind::AlreadyExists);
}

#[test]
fn mkdir_path_already_exists_error() {
    let tmpdir = tmpdir();
    let dir = "mkdir_error_twice";
    check!(tmpdir.create_dir(dir));
    let e = tmpdir.create_dir(dir).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::AlreadyExists);
}

#[test]
fn recursive_mkdir() {
    let tmpdir = tmpdir();
    let dir = "d1/d2";
    check!(tmpdir.create_dir_all(dir));
    assert!(tmpdir.is_dir(dir));
}

#[test]
fn recursive_mkdir_failure() {
    let tmpdir = tmpdir();
    let dir = "d1";
    let file = "f1";

    check!(tmpdir.create_dir_all(&dir));
    check!(tmpdir.create_file(&file));

    let result = tmpdir.create_dir_all(&file);

    assert!(result.is_err());
}

/*
#[test]
fn concurrent_recursive_mkdir() {
    for _ in 0..100 {
        let dir = tmpdir();
        let mut name = PathBuf::from("a");
        for _ in 0..40 {
            name = name.join("a");
        }
        let mut join = vec![];
        for _ in 0..8 {
            let dir = dir.clone();
            let name = name.clone();
            join.push(thread::spawn(move || {
                check!(dir.create_dir_all(&name));
            }))
        }

        // No `Display` on result of `join()`
        join.drain(..).map(|join| join.join().unwrap()).count();
    }
}
*/

#[test]
#[ignore] // is_dir is not yet fully implemented in cap-std
fn recursive_mkdir_slash() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir_all(Path::new("/")));
}

#[test]
fn recursive_mkdir_dot() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir_all(Path::new(".")));
}

#[test]
fn recursive_mkdir_empty() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir_all(Path::new("")));
}

/*
#[test]
#[ignore] // remove_dir_all is not yet implemented in cap-std
fn recursive_rmdir() {
    let tmpdir = tmpdir();
    let d1 = PathBuf::from("d1");
    let dt = d1.join("t");
    let dtt = dt.join("t");
    let d2 = PathBuf::from("d2");
    let canary = d2.join("do_not_delete");
    check!(tmpdir.create_dir_all(&dtt));
    check!(tmpdir.create_dir_all(d2));
    check!(check!(tmpdir.create_file(&canary)).write(b"foo"));
    check!(symlink_junction(d2, &dt.join("d2")));
    let _ = symlink_file(&canary, &d1.join("canary"));
    check!(tmpdir.remove_dir_all(&d1));

    assert!(!d1.is_dir());
    assert!(canary.exists());
}

#[test]
fn recursive_rmdir_of_symlink() {
    // test we do not recursively delete a symlink but only dirs.
    let tmpdir = tmpdir();
    let link = "d1";
    let dir = "d2";
    let canary = dir.join("do_not_delete");
    check!(fs::create_dir_all(&dir));
    check!(check!(File::create(&canary)).write(b"foo"));
    check!(symlink_junction(&dir, link));
    check!(fs::remove_dir_all(link));

    assert!(!link.is_dir());
    assert!(canary.exists());
}

#[test]
// only Windows makes a distinction between file and directory symlinks.
#[cfg(windows)]
fn recursive_rmdir_of_file_symlink() {
    let tmpdir = tmpdir();
    if !got_symlink_permission(&tmpdir) {
        return;
    };

    let f1 = "f1";
    let f2 = "f2";
    check!(check!(File::create(&f1)).write(b"foo"));
    check!(symlink_file(&f1, &f2));
    match fs::remove_dir_all(&f2) {
        Ok(..) => panic!("wanted a failure"),
        Err(..) => {}
    }
}

#[test]
fn unicode_path_is_dir() {
    assert!(Path::new(".").is_dir());
    assert!(!Path::new("test/stdtest/fs.rs").is_dir());

    let tmpdir = tmpdir();

    let mut dirpath = tmpdir.path().to_path_buf();
    dirpath.push("test-가一ー你好");
    check!(fs::create_dir(&dirpath));
    assert!(dirpath.is_dir());

    let mut filepath = dirpath;
    filepath.push("unicode-file-\u{ac00}\u{4e00}\u{30fc}\u{4f60}\u{597d}.rs");
    check!(File::create(&filepath)); // ignore return; touch only
    assert!(!filepath.is_dir());
    assert!(filepath.exists());
}

#[test]
fn unicode_path_exists() {
    assert!(Path::new(".").exists());
    assert!(!Path::new("test/nonexistent-bogus-path").exists());

    let tmpdir = tmpdir();
    let unicode = tmpdir.path();
    let unicode = unicode.join("test-각丁ー再见");
    check!(fs::create_dir(&unicode));
    assert!(unicode.exists());
    assert!(!Path::new("test/unicode-bogus-path-각丁ー再见").exists());
}
*/

#[test]
fn copy_file_does_not_exist() {
    let tmpdir = tmpdir();
    let from = Path::new("test/nonexistent-bogus-path");
    let to = Path::new("test/other-bogus-path");

    match tmpdir.copy(&from, &to) {
        Ok(..) => panic!(),
        Err(..) => {
            assert!(!from.exists());
            assert!(!to.exists());
        }
    }
}

#[test]
fn copy_src_does_not_exist() {
    let tmpdir = tmpdir();
    let from = Path::new("test/nonexistent-bogus-path");
    let to = "out.txt";
    check!(check!(tmpdir.create_file(&to)).write(b"hello"));
    assert!(tmpdir.copy(&from, &to).is_err());
    assert!(!from.exists());
    let mut v = Vec::new();
    check!(check!(tmpdir.open_file(&to)).read_to_end(&mut v));
    assert_eq!(v, b"hello");
}

#[test]
#[ignore] // `Dir::set_permissions` not yet implemented in cap-std
fn copy_file_ok() {
    let tmpdir = tmpdir();
    let input = "in.txt";
    let out = "out.txt";

    check!(check!(tmpdir.create_file(&input)).write(b"hello"));
    check!(tmpdir.copy(&input, &out));
    let mut v = Vec::new();
    check!(check!(tmpdir.open_file(&out)).read_to_end(&mut v));
    assert_eq!(v, b"hello");

    assert_eq!(
        check!(tmpdir.metadata(input)).permissions(),
        check!(tmpdir.metadata(out)).permissions()
    );
}

#[test]
fn copy_file_dst_dir() {
    let tmpdir = tmpdir();
    let out = "out";

    check!(tmpdir.create_file(&out));
    match tmpdir.copy(&*out, ".") {
        Ok(..) => panic!(),
        Err(..) => {}
    }
}

#[test]
#[ignore] // `Dir::set_permissions` not yet implemented in cap-std
fn copy_file_dst_exists() {
    let tmpdir = tmpdir();
    let input = "in";
    let output = "out";

    check!(check!(tmpdir.create_file(&input)).write("foo".as_bytes()));
    check!(check!(tmpdir.create_file(&output)).write("bar".as_bytes()));
    check!(tmpdir.copy(&input, &output));

    let mut v = Vec::new();
    check!(check!(tmpdir.open_file(&output)).read_to_end(&mut v));
    assert_eq!(v, b"foo".to_vec());
}

#[test]
fn copy_file_src_dir() {
    let tmpdir = tmpdir();
    let out = "out";

    match tmpdir.copy(".", &out) {
        Ok(..) => panic!(),
        Err(..) => {}
    }
    assert!(!tmpdir.exists(out));
}

#[test]
#[ignore] // `Dir::set_permissions` not yet implemented in cap-std
fn copy_file_preserves_perm_bits() {
    let tmpdir = tmpdir();
    let input = "in.txt";
    let out = "out.txt";

    let attr = check!(check!(tmpdir.create_file(&input)).metadata());
    let mut p = attr.permissions();
    p.set_readonly(true);
    check!(tmpdir.set_permissions(&input, p));
    check!(tmpdir.copy(&input, &out));
    assert!(check!(tmpdir.metadata(out)).permissions().readonly());
    check!(tmpdir.set_permissions(&input, attr.permissions()));
    check!(tmpdir.set_permissions(&out, attr.permissions()));
}

#[test]
#[cfg(windows)]
fn copy_file_preserves_streams() {
    let tmp = tmpdir();
    check!(check!(tmp.create_file("in.txt:bunny")).write("carrot".as_bytes()));
    assert_eq!(check!(tmp.copy("in.txt", "out.txt")), 0);
    assert_eq!(check!(tmp.metadata("out.txt")).len(), 0);
    let mut v = Vec::new();
    check!(check!(tmp.open_file("out.txt:bunny")).read_to_end(&mut v));
    assert_eq!(v, b"carrot".to_vec());
}

#[test]
#[ignore] // `Dir::set_permissions` not yet implemented in cap-std
fn copy_file_returns_metadata_len() {
    let tmp = tmpdir();
    let in_path = "in.txt";
    let out_path = "out.txt";
    check!(check!(tmp.create_file(&in_path)).write(b"lettuce"));
    #[cfg(windows)]
    check!(check!(tmp.create_file(tmp.join("in.txt:bunny"))).write(b"carrot"));
    let copied_len = check!(tmp.copy(&in_path, &out_path));
    assert_eq!(check!(tmp.metadata(out_path)).len(), copied_len);
}

/*
#[test]
fn copy_file_follows_dst_symlink() {
    let tmp = tmpdir();
    if !got_symlink_permission(&tmp) {
        return;
    };

    let in_path = tmp.join("in.txt");
    let out_path = tmp.join("out.txt");
    let out_path_symlink = tmp.join("out_symlink.txt");

    check!(tmpdir.write_file(&in_path, "foo"));
    check!(tmpdir.write_file(&out_path, "bar"));
    check!(symlink_file(&out_path, &out_path_symlink));

    check!(tmpdir.copy(&in_path, &out_path_symlink));

    assert!(check!(out_path_symlink.symlink_metadata())
        .file_type()
        .is_symlink());
    assert_eq!(check!(tmpdir.read_file(&out_path_symlink)), b"foo".to_vec());
    assert_eq!(check!(tmpdir.read_file(&out_path)), b"foo".to_vec());
}

#[test]
fn symlinks_work() {
    let tmpdir = tmpdir();
    if !got_symlink_permission(&tmpdir) {
        return;
    };

    let input = "in.txt";
    let out = "out.txt";

    check!(check!(File::create(&input)).write("foobar".as_bytes()));
    check!(symlink_file(&input, &out));
    assert!(check!(out.symlink_metadata()).file_type().is_symlink());
    assert_eq!(
        check!(fs::metadata(&out)).len(),
        check!(fs::metadata(&input)).len()
    );
    let mut v = Vec::new();
    check!(check!(File::open(&out)).read_to_end(&mut v));
    assert_eq!(v, b"foobar".to_vec());
}

#[test]
fn symlink_noexist() {
    // Symlinks can point to things that don't exist
    let tmpdir = tmpdir();
    if !got_symlink_permission(&tmpdir) {
        return;
    };

    // Use a relative path for testing. Symlinks get normalized by Windows,
    // so we may not get the same path back for absolute paths
    check!(symlink_file(&"foo", "bar"));
    assert_eq!(
        check!(fs::read_link(&tmpdir.join("bar"))).to_str().unwrap(),
        "foo"
    );
}

#[test]
fn read_link() {
    if cfg!(windows) {
        // directory symlink
        assert_eq!(
            check!(fs::read_link(r"C:\Users\All Users"))
                .to_str()
                .unwrap(),
            r"C:\ProgramData"
        );
        // junction
        assert_eq!(
            check!(fs::read_link(r"C:\Users\Default User"))
                .to_str()
                .unwrap(),
            r"C:\Users\Default"
        );
        // junction with special permissions
        assert_eq!(
            check!(fs::read_link(r"C:\Documents and Settings\"))
                .to_str()
                .unwrap(),
            r"C:\Users"
        );
    }
    let tmpdir = tmpdir();
    let link = "link";
    if !got_symlink_permission(&tmpdir) {
        return;
    };
    check!(symlink_file(&"foo", &link));
    assert_eq!(check!(fs::read_link(&link)).to_str().unwrap(), "foo");
}

#[test]
fn readlink_not_symlink() {
    let tmpdir = tmpdir();
    match fs::read_link(tmpdir.path()) {
        Ok(..) => panic!("wanted a failure"),
        Err(..) => {}
    }
}

#[test]
fn links_work() {
    let tmpdir = tmpdir();
    let input = "in.txt";
    let out = "out.txt";

    check!(check!(File::create(&input)).write("foobar".as_bytes()));
    check!(fs::hard_link(&input, &out));
    assert_eq!(
        check!(fs::metadata(&out)).len(),
        check!(fs::metadata(&input)).len()
    );
    assert_eq!(
        check!(fs::metadata(&out)).len(),
        check!(input.metadata()).len()
    );
    let mut v = Vec::new();
    check!(check!(File::open(&out)).read_to_end(&mut v));
    assert_eq!(v, b"foobar".to_vec());

    // can't link to yourself
    match fs::hard_link(&input, &input) {
        Ok(..) => panic!("wanted a failure"),
        Err(..) => {}
    }
    // can't link to something that doesn't exist
    match fs::hard_link(&tmpdir.join("foo"), &tmpdir.join("bar")) {
        Ok(..) => panic!("wanted a failure"),
        Err(..) => {}
    }
}

#[test]
fn chmod_works() {
    let tmpdir = tmpdir();
    let file = "in.txt";

    check!(File::create(&file));
    let attr = check!(fs::metadata(&file));
    assert!(!attr.permissions().readonly());
    let mut p = attr.permissions();
    p.set_readonly(true);
    check!(fs::set_permissions(&file, p.clone()));
    let attr = check!(fs::metadata(&file));
    assert!(attr.permissions().readonly());

    match fs::set_permissions(&tmpdir.join("foo"), p.clone()) {
        Ok(..) => panic!("wanted an error"),
        Err(..) => {}
    }

    p.set_readonly(false);
    check!(fs::set_permissions(&file, p));
}

#[test]
fn fchmod_works() {
    let tmpdir = tmpdir();
    let path = "in.txt";

    let file = check!(tmpdir.create_file(&path));
    let attr = check!(tmpdir.metadata(&path));
    assert!(!attr.permissions().readonly());
    let mut p = attr.permissions();
    p.set_readonly(true);
    check!(file.set_permissions(p.clone()));
    let attr = check!(tmpdir.metadata(&path));
    assert!(attr.permissions().readonly());

    p.set_readonly(false);
    check!(file.set_permissions(p));
}
*/

#[test]
fn sync_doesnt_kill_anything() {
    let tmpdir = tmpdir();
    let path = "in.txt";

    let mut file = check!(tmpdir.create_file(&path));
    check!(file.sync_all());
    check!(file.sync_data());
    check!(file.write(b"foo"));
    check!(file.sync_all());
    check!(file.sync_data());
}

#[test]
fn truncate_works() {
    let tmpdir = tmpdir();
    let path = "in.txt";

    let mut file = check!(tmpdir.create_file(&path));
    check!(file.write(b"foo"));
    check!(file.sync_all());

    // Do some simple things with truncation
    assert_eq!(check!(file.metadata()).len(), 3);
    check!(file.set_len(10));
    assert_eq!(check!(file.metadata()).len(), 10);
    check!(file.write(b"bar"));
    check!(file.sync_all());
    assert_eq!(check!(file.metadata()).len(), 10);

    let mut v = Vec::new();
    check!(check!(tmpdir.open_file(&path)).read_to_end(&mut v));
    assert_eq!(v, b"foobar\0\0\0\0".to_vec());

    // Truncate to a smaller length, don't seek, and then write something.
    // Ensure that the intermediate zeroes are all filled in (we have `seek`ed
    // past the end of the file).
    check!(file.set_len(2));
    assert_eq!(check!(file.metadata()).len(), 2);
    check!(file.write(b"wut"));
    check!(file.sync_all());
    assert_eq!(check!(file.metadata()).len(), 9);
    let mut v = Vec::new();
    check!(check!(tmpdir.open_file(&path)).read_to_end(&mut v));
    assert_eq!(v, b"fo\0\0\0\0wut".to_vec());
}

#[test]
#[ignore] // cap-std's OpenOptions doesn't handle all the flags correctly yet
fn open_flavors() {
    use cap_std::fs::OpenOptions as OO;
    fn c<T: Clone>(t: &T) -> T {
        t.clone()
    }

    let tmpdir = tmpdir();

    let mut r = OO::new();
    r.read(true);
    let mut w = OO::new();
    w.write(true);
    let mut rw = OO::new();
    rw.read(true).write(true);
    let mut a = OO::new();
    a.append(true);
    let mut ra = OO::new();
    ra.read(true).append(true);

    #[cfg(windows)]
    let invalid_options = 87; // ERROR_INVALID_PARAMETER
    #[cfg(all(unix, not(target_os = "vxworks")))]
    let invalid_options = "Invalid argument";
    #[cfg(target_os = "vxworks")]
    let invalid_options = "invalid argument";

    // Test various combinations of creation modes and access modes.
    //
    // Allowed:
    // creation mode           | read  | write | read-write | append | read-append |
    // :-----------------------|:-----:|:-----:|:----------:|:------:|:-----------:|
    // not set (open existing) |   X   |   X   |     X      |   X    |      X      |
    // create                  |       |   X   |     X      |   X    |      X      |
    // truncate                |       |   X   |     X      |        |             |
    // create and truncate     |       |   X   |     X      |        |             |
    // create_new              |       |   X   |     X      |   X    |      X      |
    //
    // tested in reverse order, so 'create_new' creates the file, and 'open existing' opens it.

    // write-only
    check!(tmpdir.open_file_with("a", c(&w).create_new(true)));
    check!(tmpdir.open_file_with("a", c(&w).create(true).truncate(true)));
    check!(tmpdir.open_file_with("a", c(&w).truncate(true)));
    check!(tmpdir.open_file_with("a", c(&w).create(true)));
    check!(tmpdir.open_file_with("a", &c(&w)));

    // read-only
    error!(
        tmpdir.open_file_with("b", c(&r).create_new(true)),
        invalid_options
    );
    error!(
        tmpdir.open_file_with("b", c(&r).create(true).truncate(true)),
        invalid_options
    );
    error!(
        tmpdir.open_file_with("b", c(&r).truncate(true)),
        invalid_options
    );
    error!(
        tmpdir.open_file_with("b", c(&r).create(true)),
        invalid_options
    );
    check!(tmpdir.open_file_with("a", &c(&r))); // try opening the file created with write_only

    // read-write
    check!(tmpdir.open_file_with("c", c(&rw).create_new(true)));
    check!(tmpdir.open_file_with("c", c(&rw).create(true).truncate(true)));
    check!(tmpdir.open_file_with("c", c(&rw).truncate(true)));
    check!(tmpdir.open_file_with("c", c(&rw).create(true)));
    check!(tmpdir.open_file_with("c", &c(&rw)));

    // append
    check!(tmpdir.open_file_with("d", c(&a).create_new(true)));
    error!(
        tmpdir.open_file_with("d", c(&a).create(true).truncate(true)),
        invalid_options
    );
    error!(
        tmpdir.open_file_with("d", c(&a).truncate(true)),
        invalid_options
    );
    check!(tmpdir.open_file_with("d", c(&a).create(true)));
    check!(tmpdir.open_file_with("d", &c(&a)));

    // read-append
    check!(tmpdir.open_file_with("e", c(&ra).create_new(true)));
    error!(
        tmpdir.open_file_with("e", c(&ra).create(true).truncate(true)),
        invalid_options
    );
    error!(
        tmpdir.open_file_with("e", c(&ra).truncate(true)),
        invalid_options
    );
    check!(tmpdir.open_file_with("e", c(&ra).create(true)));
    check!(tmpdir.open_file_with("e", &c(&ra)));

    // Test opening a file without setting an access mode
    let mut blank = OO::new();
    error!(
        tmpdir.open_file_with("f", blank.create(true)),
        invalid_options
    );

    // Test write works
    check!(check!(tmpdir.create_file("h")).write("foobar".as_bytes()));

    // Test write fails for read-only
    check!(tmpdir.open_file_with("h", &r));
    {
        let mut f = check!(tmpdir.open_file_with("h", &r));
        assert!(f.write("wut".as_bytes()).is_err());
    }

    // Test write overwrites
    {
        let mut f = check!(tmpdir.open_file_with("h", &c(&w)));
        check!(f.write("baz".as_bytes()));
    }
    {
        let mut f = check!(tmpdir.open_file_with("h", &c(&r)));
        let mut b = vec![0; 6];
        check!(f.read(&mut b));
        assert_eq!(b, "bazbar".as_bytes());
    }

    // Test truncate works
    {
        let mut f = check!(tmpdir.open_file_with("h", c(&w).truncate(true)));
        check!(f.write("foo".as_bytes()));
    }
    assert_eq!(check!(tmpdir.metadata("h")).len(), 3);

    // Test append works
    assert_eq!(check!(tmpdir.metadata("h")).len(), 3);
    {
        let mut f = check!(tmpdir.open_file_with("h", &c(&a)));
        check!(f.write("bar".as_bytes()));
    }
    assert_eq!(check!(tmpdir.metadata("h")).len(), 6);

    // Test .append(true) equals .write(true).append(true)
    {
        let mut f = check!(tmpdir.open_file_with("h", c(&w).append(true)));
        check!(f.write("baz".as_bytes()));
    }
    assert_eq!(check!(tmpdir.metadata("h")).len(), 9);
}

#[test]
fn _assert_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<OpenOptions>();
}

#[test]
fn binary_file() {
    let mut bytes = [0; 1024];
    StdRng::from_entropy().fill_bytes(&mut bytes);

    let tmpdir = tmpdir();

    check!(check!(tmpdir.create_file("test")).write(&bytes));
    let mut v = Vec::new();
    check!(check!(tmpdir.open_file("test")).read_to_end(&mut v));
    assert!(v == &bytes[..]);
}

#[test]
fn write_then_read() {
    let mut bytes = [0; 1024];
    StdRng::from_entropy().fill_bytes(&mut bytes);

    let tmpdir = tmpdir();

    check!(tmpdir.write_file("test", &bytes[..]));
    let v = check!(tmpdir.read_file("test"));
    assert!(v == &bytes[..]);

    check!(tmpdir.write_file("not-utf8", &[0xFF]));
    error_contains!(
        tmpdir.read_to_string("not-utf8"),
        "stream did not contain valid UTF-8"
    );

    let s = "𐁁𐀓𐀠𐀴𐀍";
    check!(tmpdir.write_file("utf8", s.as_bytes()));
    let string = check!(tmpdir.read_to_string("utf8"));
    assert_eq!(string, s);
}

#[test]
fn file_try_clone() {
    let tmpdir = tmpdir();

    let mut f1 = check!(tmpdir.open_file_with(
        "test",
        OpenOptions::new().read(true).write(true).create(true)
    ));
    let mut f2 = check!(f1.try_clone());

    check!(f1.write_all(b"hello world"));
    check!(f1.seek(SeekFrom::Start(2)));

    let mut buf = vec![];
    check!(f2.read_to_end(&mut buf));
    assert_eq!(buf, b"llo world");
    drop(f2);

    check!(f1.write_all(b"!"));
}

#[test]
#[ignore] // `metadata` not yet implemented in cap-std
#[cfg(not(windows))]
fn unlink_readonly() {
    let tmpdir = tmpdir();
    let path = "file";
    check!(tmpdir.create_file(&path));
    let mut perm = check!(tmpdir.metadata(&path)).permissions();
    perm.set_readonly(true);
    check!(tmpdir.set_permissions(&path, perm));
    check!(tmpdir.remove_file(&path));
}

#[test]
#[ignore] // `create_dir_all` not yet implemented in cap-std
fn mkdir_trailing_slash() {
    let tmpdir = tmpdir();
    let path = PathBuf::from("file");
    check!(tmpdir.create_dir_all(&path.join("a/")));
}

/*
#[test]
fn canonicalize_works_simple() {
    let tmpdir = tmpdir();
    let tmpdir = fs::canonicalize(tmpdir.path()).unwrap();
    let file = "test";
    File::create(&file).unwrap();
    assert_eq!(fs::canonicalize(&file).unwrap(), file);
}

#[test]
fn realpath_works() {
    let tmpdir = tmpdir();
    if !got_symlink_permission(&tmpdir) {
        return;
    };

    let tmpdir = fs::canonicalize(tmpdir.path()).unwrap();
    let file = "test";
    let dir = "test2";
    let link = dir.join("link");
    let linkdir = "test3";

    File::create(&file).unwrap();
    fs::create_dir(&dir).unwrap();
    symlink_file(&file, &link).unwrap();
    symlink_dir(&dir, &linkdir).unwrap();

    assert!(link.symlink_metadata().unwrap().file_type().is_symlink());

    assert_eq!(fs::canonicalize(&tmpdir).unwrap(), tmpdir);
    assert_eq!(fs::canonicalize(&file).unwrap(), file);
    assert_eq!(fs::canonicalize(&link).unwrap(), file);
    assert_eq!(fs::canonicalize(&linkdir).unwrap(), dir);
    assert_eq!(fs::canonicalize(&linkdir.join("link")).unwrap(), file);
}

#[test]
fn realpath_works_tricky() {
    let tmpdir = tmpdir();
    if !got_symlink_permission(&tmpdir) {
        return;
    };

    let tmpdir = fs::canonicalize(tmpdir.path()).unwrap();
    let a = "a";
    let b = a.join("b");
    let c = b.join("c");
    let d = a.join("d");
    let e = d.join("e");
    let f = a.join("f");

    fs::create_dir_all(&b).unwrap();
    fs::create_dir_all(&d).unwrap();
    File::create(&f).unwrap();
    if cfg!(not(windows)) {
        symlink_file("../d/e", &c).unwrap();
        symlink_file("../f", &e).unwrap();
    }
    if cfg!(windows) {
        symlink_file(r"..\d\e", &c).unwrap();
        symlink_file(r"..\f", &e).unwrap();
    }

    assert_eq!(fs::canonicalize(&c).unwrap(), f);
    assert_eq!(fs::canonicalize(&e).unwrap(), f);
}
*/

#[test]
#[ignore] // `read_dir` not yet implemented in cap-std
fn dir_entry_methods() {
    let tmpdir = tmpdir();

    tmpdir.create_dir_all("a").unwrap();
    tmpdir.create_file("b").unwrap();

    for file in tmpdir.read_dir(".").unwrap().map(|f| f.unwrap()) {
        let fname = file.file_name();
        match fname.to_str() {
            Some("a") => {
                assert!(file.file_type().unwrap().is_dir());
                assert!(file.metadata().unwrap().is_dir());
            }
            Some("b") => {
                assert!(file.file_type().unwrap().is_file());
                assert!(file.metadata().unwrap().is_file());
            }
            f => panic!("unknown file name: {:?}", f),
        }
    }
}

#[test]
#[ignore] // `read_dir` not yet implemented in cap-std
fn dir_entry_debug() {
    let tmpdir = tmpdir();
    tmpdir.create_file("b").unwrap();
    let mut read_dir = tmpdir.read_dir(".").unwrap();
    let dir_entry = read_dir.next().unwrap().unwrap();
    let actual = format!("{:?}", dir_entry);
    let expected = format!("DirEntry({:?})", dir_entry.file_name());
    assert_eq!(actual, expected);
}

#[test]
#[ignore] // `read_dir` not yet implemented in cap-std
fn read_dir_not_found() {
    let tmpdir = tmpdir();
    let res = tmpdir.read_dir("/path/that/does/not/exist");
    assert_eq!(res.err().unwrap().kind(), ErrorKind::NotFound);
}

/*
#[test]
fn create_dir_all_with_junctions() {
    let tmpdir = tmpdir();
    let target = "target";

    let junction = "junction";
    let b = junction.join("a/b");

    let link = "link";
    let d = link.join("c/d");

    fs::create_dir(&target).unwrap();

    check!(symlink_junction(&target, &junction));
    check!(fs::create_dir_all(&b));
    // the junction itself is not a directory, but `is_dir()` on a Path
    // follows links
    assert!(junction.is_dir());
    assert!(b.exists());

    if !got_symlink_permission(&tmpdir) {
        return;
    };
    check!(symlink_dir(&target, &link));
    check!(fs::create_dir_all(&d));
    assert!(link.is_dir());
    assert!(d.exists());
}
*/

#[test]
#[ignore] // not yet implemented in cap-std
fn metadata_access_times() {
    let tmpdir = tmpdir();

    let b = "b";
    tmpdir.create_file(&b).unwrap();

    let a = check!(tmpdir.metadata("."));
    let b = check!(tmpdir.metadata(&b));

    assert_eq!(check!(a.accessed()), check!(a.accessed()));
    assert_eq!(check!(a.modified()), check!(a.modified()));
    assert_eq!(check!(b.accessed()), check!(b.modified()));

    if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        check!(a.created());
        check!(b.created());
    }

    if cfg!(target_os = "linux") {
        // Not always available
        match (a.created(), b.created()) {
            (Ok(t1), Ok(t2)) => assert!(t1 <= t2),
            (Err(e1), Err(e2))
                if e1.kind() == ErrorKind::Other && e2.kind() == ErrorKind::Other => {}
            (a, b) => panic!(
                "creation time must be always supported or not supported: {:?} {:?}",
                a, b,
            ),
        }
    }
}
