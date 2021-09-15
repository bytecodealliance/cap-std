<div align="center">
  <h1><code>cap-std</code></h1>

  <p>
    <strong>Capability-based version of the Rust standard library</strong>
  </p>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <a href="https://github.com/bytecodealliance/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/217126-wasmtime"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
  </p>
</div>

The `cap-std` project is organized around the eponymous [`cap-std`] crate, and
develops libraries to make it easy to write capability-based code, including:

 - [`cap-std`] itself, which provides capability-based versions of `std` APIs
 - [`cap-async-std`], which is to [`async-std`] what `cap-std` is to `std`
 - [`cap-directories`] which provides capability-based access to
   [standard application directories]
 - [`cap-tempfile`], which provides capability-based access to
   [temporary directories]
 - [`cap-fs-ext`], which provides additional filesystem features beyond
   what's available in `std`
 - [`cap-time-ext`], which provides additional time features beyond
   what's available in `std`
 - [`cap-rand`], which provides capability-based access to
   [random number generators]

Cap-std features protection against [CWE-22], "Improper Limitation of a
Pathname to a Restricted Directory ('Path Traversal')", which is #8 in the
[2021 CWE Top 25 Most Dangerous Software Weaknesses]. It can also be used to
prevent untrusted input from inducing programs to open "/proc/self/mem" on
Linux.

[CWE-22]: https://cwe.mitre.org/data/definitions/22.html
[2021 CWE Top 25 Most Dangerous Software Weaknesses]: https://cwe.mitre.org/top25/archive/2021/2021_cwe_top25.html
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

## Capability-based security

Operating systems have a concept of resource handles, or file descriptors, which
are values that can be passed around within and sometimes between programs, and
which represent access to external resources. Programs typically have the
*ambient authority* to request any file or network handle simply by providing
its name or address:

```rust
let file = File::open("/anything/you/want.txt")?;
```

There may be access-control lists, namespaces, firewalls, or virtualization
mechanisms governing which resources can actually be accessed, but those are
typically coarse-grained and configured outside of the application.

Capability-based security seeks to avoid ambient authority, to make sandboxing
finer-grained and composable. To open a file, one needs a [`Dir`], representing
an open directory it's in:

```rust
let file = dir.open("the/thing.txt")?;
```

Attempts to access paths not contained within the directory:

```rust
let hidden = dir.open("../hidden.txt")?;

dir.symlink("/hidden.txt", "misdirection.txt")?;
let secret = dir.open("misdirection.txt")?;
```

return `PermissionDenied` errors.

This allows application logic to configure its own access, without changing the
behavior of the whole host process, setting up a separate host process, or
requiring external configuration.

[`Dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html

## How do I obtain a [`Dir`]?

If every resource requires some other resource to obtain, how does one obtain
the first resource?

There currently are three main ways:
 - Use the [`cap-directories`] crate to create `Dir`s for config, cache and
   other data directories.
 - Use the [`cap-tempfile`] crate to create `Dir`s for temporary directories.
 - Use [`Dir::open_ambient_dir`] to open a plain path. This function is not
   sandboxed, and may open any file the host process has access to.

## Examples

There are several examples of cap-std in use:

 - As a sandbox: For a simple yet complete example of cap-std in action, see
   [this port of tide], to use cap-std to access static files, where it
   prevents path resolution from following symlinks outside of the designated
   root directory. [The diff] shows the kinds of changes needed to use this
   API.

 - As a general-purpose `Dir` type for working with directories: The io-streams
   crate [uses `cap-tempdir`] to create temporary directories for unit tests.
   Here, the main benefit of `Dir` is just convenience—`Dir`'s API lets tests
   just say `dir.open(...)` instead of using `open(path.join(...))` or dealing
   with `chdir` and global mutable state. The fact that it also sandboxes the
   unit tests is just a nice side effect.

 - As an application data store: See the [`kv-cli` example] for a simple example
   of a program using `cap-directories` and `cap-std` APIs to store
   application-specific data.

 - And, cap-std is a foundation for the [`WASI`] implementation in [`Wasmtime`],
   providing sandboxing and support for Linux, macOS, Windows, and more.

[uses `cap-tempdir`]: https://github.com/sunfishcode/io-streams/blob/main/tests/tests.rs#L16
[this port of tide]: https://github.com/sunfishcode/tide/
[The diff]: https://github.com/sunfishcode/tide/commit/e431ebbba3ac9d5799ad0c661868b8a4a339a255
[`Dir::open_ambient_dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html#method.open_ambient_dir
[`kv-cli` example]: https://github.com/bytecodealliance/cap-std/blob/main/examples/kv-cli.rs
[`WASI`]: https://github.com/WebAssembly/WASI/
[`Wasmtime`]: https://wasmtime.dev/

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

cap-std also contains a simple capability-based version of `std::net`, with a
[`Pool`] type that represents a pool of network addresses and ports that can be
accessed, which serves an analogous role to [`Dir`]. It's usable for basic use
cases, though it's not yet very sophisticated.

[`Pool`]: https://docs.rs/cap-std/latest/cap_std/net/struct.Pool.html

## What is `cap_std::fs_utf8`?

It's similar to `cap_std::fs`, but uses [`camino`] for its `Path` types, so
paths are always valid UTF-8. To use it, opt in by enabling the `fs_utf8`
feature and using `std::fs_utf8` in place of `std::fs`.

There's also an experimental extension to `fs_utf8` which allows losslessly
encoding arbitrary host byte sequences within UTF-8 strings, using the
[`arf-strings`] technique. To try this experiment, opt in by enabling the
`arf_strings` feature.

## Similar crates

`cap-std` provides similar functionality to the [`openat`] crate, with a similar
`Dir` type with associated functions corresponding to `*at` functions.
`cap-std`'s `Dir` type performs sandboxing, including for multiple-component
paths.

`cap-std` has some similar functionality to [`pathrs`] in that it also
explicitly verifies that `/proc` has actual `/proc` mounted on it and nothing
mounted on top, and it can also use `openat2`. However, `cap-std` uses
`RESOLVE_BENEATH`-style resolution where absolute paths are considered errors,
while `pathrs` uses `RESOLVE_IN_ROOT` where absolute paths are interpreted as
references to the base file descriptor. And overall, `cap-std` seeks to provide
a portable `std`-like API which supports Windows in addition to Unix-like
platforms, while `pathrs` provides a lower-level API that exposes more of the
underlying `openat2` options and only supports Linux.

[`obnth`] is a new crate which appears to be very similar to `cap_std::fs`.
It's not mature yet, and it doesn't support Windows. It does support
`openat2`-like features such as `RESOLVE_NO_XDEV`, `RESOLVE_NO_SYMLINKS`,
and `RESOLVE_IN_ROOT`, including emulation when `openat2` isn't available.

[`arf-strings`]: https://github.com/bytecodealliance/arf-strings/
[`openat`]: https://crates.io/crates/openat
[`pathrs`]: https://crates.io/crates/pathrs
[`obnth`]: https://crates.io/crates/obnth
[`camino`]: https://crates.io/crates/camino
