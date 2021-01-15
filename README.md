<div align="center">
  <h1><code>cap-std</code></h1>

  <p>
    <strong>Capability-oriented version of the Rust standard library</strong>
  </p>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <a href="https://github.com/bytecodealliance/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
  </p>
</div>

The `cap-std` project is organized around the eponymous [`cap-std`] crate, and
develops libraries to make it easy to write capability-oriented code, including:

 - [`cap-std`] itself, which provides capability-oriented versions of `std` APIs
 - [`cap-async-std`], which is to [`async-std`] what `cap-std` is to `std`
 - [`cap-directories`] which provides capability-oriented access to
   [standard application directories]
 - [`cap-tempfile`], which provides capability-oriented access to
   [temporary directories]
 - [`cap-fs-ext`], which provides additional filesystem features beyond
   what's available in `std`
 - [`cap-time-ext`], which provides additional time features beyond
   what's available in `std`
 - [`cap-rand`], which provides capability-oriented access to
   [random number generators]

[`std`]: https://doc.rust-lang.org/std/
[`cap-std`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-std/README.md
[`cap-async-std`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-async-std/README.md
[`cap-directories`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-directories/README.md
[`cap-tempfile`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-tempfile/README.md
[`cap-fs-ext`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-fs-ext/README.md
[`cap-time-ext`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-time-ext/README.md
[`cap-rand`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-rand/README.md
[`cap_std::fs`]: https://docs.rs/cap-std/latest/cap_std/fs/index.html
[`async-std`]: https://docs.rs/async-std/
[standard application directories]: https://docs.rs/directories-next/
[temporary directories]: https://docs.rs/tempfile/
[random number generators]: https://docs.rs/rand/

## Capability-oriented security

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

Capability-oriented security seeks to avoid ambient authority, to make sandboxing
finer-grained and composable. To open a file, one needs a [`Dir`], representing
an open directory it's in:

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

This allows application logic to configure its own access, without changing the
behavior of the whole host process, setting up a separate host process, or
requiring external configuration.

For a complete example of cap-std in action, see
[this port of tide-naive-static-files], a simple static-file Web server, to use
cap-std to access the static files. [The diff] shows the kinds of changes
needed to use this API.

[this port of tide-naive-static-files]: https://github.com/sunfishcode/tide-naive-static-files/
[The diff]: https://github.com/eignnx/tide-naive-static-files/compare/master...sunfishcode:main

[`Dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html

## How do I obtain a [`Dir`]?

If every resource requires some other resource to obtain, how does one obtain
the first resource?

There currently are three main ways:
 - Use the [`cap-directories`] crate to create `Dir`s for config, cache and
   other data directories.
 - Use the [`cap-tempfile`] crate to create `Dir`s for temporary directories.
 - Use the `unsafe` [`Dir::open_ambient_dir`] to open a plain path. This
   function is not sandboxed, and may open any file the host process has
   access to.

See the [`kv-cli` example] for a simple example of a program using `cap-directories`
and `cap-std` APIs.

[`Dir::open_ambient_dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html#method.open_ambient_dir
[`kv-cli` example]: https://github.com/bytecodealliance/cap-std/blob/main/examples/kv-cli.rs

## What can I use `cap-std` for?

`cap-std` is not a sandbox for untrusted Rust code. Among other things,
untrusted Rust code could use `unsafe` or the unsandboxed APIs in `std::fs`.

`cap-std` allows code to declare its intent and to opt in to protection from
malicious path names. Code which takes a [`Dir`] from which to open files,
rather than taking bare filenames, declares its intent to only open files
underneath that `Dir`. And, `Dir` automatically protects against paths which
might include `..`, symlinks, or absolute paths that might lead outside of that
`Dir`.

`cap-std` also has another role, within WASI, because `cap-std`'s filesystem
APIs closely follow WASI's sandboxing APIs. In WASI, `cap-std` becomes a very
thin layer, thinner than `libstd`'s filesystem APIs because it doesn't need
extra code to handle absolute paths.

## How fast is it?

On Linux 5.6 and newer, `cap-std` uses [`openat2`] to implement `Dir::open`
with a single system call in common cases. Several other operations internally
utilize [`openat2`], [`O_PATH`], and [`/proc/self/fd`] (though only when /proc
is mounted, it's really `procfs`, and there are no mounts on top of it) for
fast path resolution as well.

Otherwise, `cap-std` opens each component of a path individually, in order to
specially handle `..` and symlinks. The algorithm is carefully designed to
minimize system calls, so opening `red/green/blue` performs just 5 system
calls—it opens `red`, `green`, and then `blue`, and closes the handles for `red`
and `green`.

[`openat2`]: https://lwn.net/Articles/796868/
[`O_PATH`]: https://man7.org/linux/man-pages/man2/open.2.html
[`/proc/self/fd`]: https://man7.org/linux/man-pages/man5/proc.5.html

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
