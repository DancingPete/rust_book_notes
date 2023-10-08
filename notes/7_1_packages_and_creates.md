# Packages and Crates

A *crate* is the smallest amount of code that the compiler considers
at a time.
Even where a single file is passed to `rustc`, the compiler treats that
file as a crate.
Crates may contain modules, and modules may be defined in other files that get
compiled with the crate.

A crate may take the form of either a binary or a library.

*Binary crates* are programs that may be compiled into an executable program,
such as a command-line program or a server.
Each must have a function called `main` that defines what happens when the
executable runs.

*Library crates* don't have a main function, and they don't compile to an
executable.
Instead, they defined functionality inteded to be shared with multiple projects.
For example, the `rand` crate provides functionality for generating random
numbers.
*Crate* and *library* are often used interchangeably.

The *crate root* is a source file from which the Rust compiler starts.

A *package* is a bundle of one or more crates that provides a set of
functionality.
A package contains a *Cargo.toml* file that describes how to build those
crates.
A package may contain as many binary crates as desired, but may only contain
one library crate.
A package must contain at least one crate.

Cargo, in creating a new project directory, as well as a package (by virtue
of the Cargo.toml), follows a convention of using the `src/main.rs` as the
crate root of a binary crate with the same name as the package.
Likewise, Cargo knows that if the package directory contains `src/lib.rs`,
the package contains a library crate with the same name as the package,
and `src/lib.rs` as its crate root.
Cargo passes the crate root files to `rustc` to build the library or binary.

If a project contains both `src/main.rs` and `src/lib.rs`, it has two crates:
a binary and a library, both with the same name as the package.

A package can have multiple binary crates by placing files in the `src/bin`
directory: each file will be a separate binary crate.
