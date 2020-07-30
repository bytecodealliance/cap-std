<div align="center">
  <h1><code>cap-std</code></h1>

  <p>
    <strong>Capability-based version of Rust standard library</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://cirrus-ci.com/github/sunfishcode/cap-std"><img src="https://api.cirrus-ci.com/github/sunfishcode/cap-std.svg" alt="Cirrus CI Status" /></a>
  </p>
</div>

`cap-std` crate provides a capability-based version of [`std`]. It provides all the
interfaces you are used to, but in a capability-based version.

[`std`]: https://doc.rust-lang.org/std/

The filesystem module, `fs`, is known to work on Linux, macOS, and FreeBSD, and
probably can be easily ported to other modern Unix-family platforms. Ports to
Windows and WASI platforms are in development, though not yet usable.

The networking module, `net`, is not yet usable.

## Capability-based security

Operating systems have a concept of resource handles, or file descriptors, which
are values that can be passed around within and sometimes between programs, and
which represent access to external resources. Programs typically have the
*ambient authority* to request any file or network handle simply by providing
its name or address:

```
    let file = File::open("/anything/you/want.txt")?;
```

There may be access-control lists, namespaces, firewalls, or virtualization
mechanisms governing which resources can actually be accessed, but those are
typically coarse-grained and configured outside of the application.

Capability-based security seeks to avoid ambient authority, to make sandboxing
finer-grained and composable. To open a file, one needs a handle to a directory
it's in:

```
    let file = dir.open("the/thing.txt")?;
```

Attempts to access paths not contained within the directory:

```
    let hidden = dir.open("../hidden.txt")?;

    dir.symlink("/hidden.txt", "misdirection.txt")?;
    let secret = dir.open("misdirection.txt")?;
```

return `PermissionDenied` errors.

This allows application logic to configure its own access, without changing
the behavior of the whole host process, setting up a separate host process, or
requiring external configuration.

## How do I obtain a `Dir`?

If every resource requires some other resource to obtain, how does one obtain
the first resource?

There are three main ways:
 - Use the [`cap-directories`] crate to create `Dir`s for config, cache and
   other data directories.
 - Use the [`cap-tempfile`] crate to create `Dir`s for temporary directories.
 - Use the `unsafe` [`Dir::open_ambient_dir`] to open a plain path. This
   function is not sandboxed, and may open any file the host process has
   access to.

See the [`kv-cli` example] for a simple example of a program using `cap-directories`
and `cap-std` APIs.

[`cap-directories`]: https://crates.io/crates/cap-directories
[`cap-tempfile`]: https://crates.io/crates/cap-tempfile
[`Dir::open_ambient_dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html#method.open_ambient_dir
[`kv-cli` example]: https://github.com/sunfishcode/cap-std/blob/main/examples/kv-cli.rs

## How do I use a `Dir`?

Once you have a `Dir`, it's very similar to the Rust standard library:

 - Instead of using `File::open(...)`, call `dir.open(...)`.
 - Instead of using `File::create(...)`, call `dir.create(...)`.
 - Instead of using `fs::metadata(...)`, call `dir.metadata(...)`.

and so on.

## What can I use `cap-std` for?

`cap-std` is not a sandbox for untrusted Rust code. Among other things,
untrusted Rust code could use `unsafe` or the unsandboxed APIs in `std::fs`.

`cap-std` allows code to declare its intent, and opt in to protection from
malicious path names. Code which takes a `Dir` from which to open files, rather
than taking bare filenames, declares its intent to only open files underneath
that `Dir`. And, `Dir` automatically protects against paths which might include
`..`, symlinks, or absolute paths that might lead outside of that `Dir`.

`cap-std` also has another role, within WASI, because `cap-std`'s filesystem
APIs closely follow WASI's sandboxing APIs. In WASI, `cap-std` becomes a very
thin layer, thinner than `libstd`'s filesystem APIs because it doesn't need
extra code to handle absolute paths.

## How fast is it?

On Linux 5.6 and newer, `cap-std` uses [`openat2`] to implement `open` and with
a single system call in common cases. Several other operations internally
utilize `openat2` for fast path resolution as well.

Otherwise, `cap-std` opens each component of a path individually, in order to
specially handle `..` and symlinks. The algorithm is carefully designed to
minimize system calls, so opening `red/green/blue` performs just 5 system
calls—it opens `red`, `green`, and then `blue`, and closes the handles for `red`
and `green`.

[`openat2`]: https://lwn.net/Articles/796868/

## Async support

Async APIs are available in the [`cap-async-std`] crate.

[`cap-async-std`]: https://crates.io/crates/cap-async-std

## What about networking?

This library contains a few sketches of how to apply similar ideas to
networking, but it's very incomplete at this time. If you're interested in this
area, let's talk about what this might become!

## What is `cap_std::fs_utf8`?

It's an experiment in what an API with UTF-8 filesystem paths (but which still
allow you to access any file with any byte-sequence name) might look like. For
more information on the technique, see the [`arf-strings` package]. To try it,
opt in by enabling the `fs_utf8` feature and using `std::fs_utf8` in place of
`std::fs`.

[`arf-strings` package]: https://github.com/bytecodealliance/arf-strings/
