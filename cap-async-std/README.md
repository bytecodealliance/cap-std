<div align="center">
  <h1><code>cap-async-std</code></h1>

  <p>
    <strong>Capability-based version of `async-std`</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-async-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-async-std/workflows/CI/badge.svg" alt="build status" /></a>
  </p>
</div>

This crate provides a capability-based version of [`async-std`]. It provides all the
interfaces you are used to, but in a capability-based version.

This is a very simplistic port of [`cap-std`] to `async-std`. Key `fs` functions
including opening files still use synchronous API calls. Quite a few comments still
talk about `std` rather than `async-std`.

See the [`cap-std` README.md] for more information about capability-based security.

[`async-std`]: https://docs.rs/async_std/
[`cap-std`]: https://docs.rs/cap_std/
[`cap-std` README.md]: https://github.com/sunfishcode/cap-std/blob/main/README.md

**It is a work in progress and many things aren't implemented yet.**

## Capability-based security.

Conventional operating systems have a concept of resource handles, or file
descriptors, which are values that can be passed around within and sometimes 
etween programs, and which represent access to external resources. However,
programs typically have *ambient authority* to request a file or network handle
simply by providing its name:

```
    let file = File::open("/anything/you/want.txt")?;
```

There may be access-control or namespace mechanisms governing which resources
can actually be accessed, but those are typically coarse-grained.

Capability-based security seeks to avoid ambient authority, to make sandboxing
finer-grained and more composable. To open a file, one needs a handle to a
directory it's in:

```
    let file = dir.open_file("the/thing.txt")?;
```

This way, it doesn't require creating new users or groups, filesystem namespaces,
or separate host processes, just to set up a simple sandbox.

## How do I obtain a `Dir`?

If every resource requires some other resource to obtain, how does one obtain
the first resource?

For now, `cap-std`'s answer is that you use conventional ambient authority
methods such as `std::fs::File::open` to open directories, and then you can
call `Dir::from_std_file`.

In the future, this space may get more interesting :-).

## How do I use a `Dir`?

Once you have a `Dir`, it's very similar to the Rust standard library:

 - Instead of using `File::open(...)`, call `dir.open_file(...)`.
 - Insead of using `File::create(...)`, call `dir.create_file(...)`.

Most other methods have the same name as their `libstd` counterparts.

 - Insead of using `fs::foo(...)`, call `dir.foo(...)`.

## What can I use `cap-std` for?

`cap-std` is not a sandbox for arbitrary Rust code. Among other things,
arbitrary Rust code could use `unsafe` or the unsandboxed APIs in `std::fs`.

What `cap-std` can do is allow code to declare its intent, and opt in to
protections from malicious path names. Code which takes a `Dir` from which to
open files, rather than bare filenames, declares its intent to only open files
underneath that `Dir`. And, `Dir` automatically protects against any paths
which might include `..` or symlinks that might lead outside of that `Dir`.

`cap-std` also has another role, within WASI, because cap-std's filesystem
APIs closely follow WASI's sandboxing APIs. On WASI, cap-std becomes a very
thin layer, thinner than libstd's filesystem APIs because it doesn't need
extra code to handle absolute paths.

## How fast is it?

On Linux 5.6 and newer, `cap-std` uses the [`openat2`] to implement `open`
and with a single system call in common cases. Several other operations
internally utilize `openat2` for fast path resolution as well.

Otherwise, opens each component of a path individually, in order to specially
handle `..` and symlinks. The algorithm is carefully designed to minimize
system calls, so opening "red/green/blue" performs just 5 system calls - it
opens "red", "green", and then "blue", and closes the handles for "red" and
"green".

[`openat2`]: https://lwn.net/Articles/796868/

## What about networking?

This library contains a few sketches of how to apply similar ideas to
networking, but it's very incomplete at this time. If you're interested in
this area, let's talk about what this might become!

## What is `cap_std::fs_utf8`?

It's an experiment in what an API with UTF-8 filesystem paths (but which
still allow you to access any file with any byte-sequence name) might look
like. For more information on the technique, see the [`arf-strings` package].
To try it, opt in by enabling the `fs_utf8` feature and using `std::fs_utf8`
in place of `std::fs`.

[`arf-strings` documentation]: https://github.com/bytecodealliance/arf-strings/
