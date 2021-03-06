<!-- Copyright © 2017–2020 University of Malta -->

<!-- Copying and distribution of this file, with or without
modification, are permitted in any medium without royalty provided the
copyright notice and this notice are preserved. This file is offered
as-is, without any warranty. -->

# Rust low-level bindings for GMP, MPFR and MPC

The gmp-mpfr-sys crate provides Rust FFI bindings to the following
[GNU] arbitrary-precision libraries:

  * [GMP] for integers and rational numbers,
  * [MPFR] for floating-point numbers, and
  * [MPC] for complex numbers.

The source of the three libraries is included in the package.

The gmp-mpfr-sys crate is free software: you can redistribute it
and/or modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation, either version 3
of the License, or (at your option) any later version. See the full
text of the [GNU LGPL] and [GNU GPL] for details.

## What’s new

### Version 1.2.0 news (2020-01-18)

  * The crate now requires rustc version 1.37.0 or later.
  * The crate is now `no_std`.
  * [GMP] was updated from version 6.1.2 to 6.2.0.
  * The implementation details of [`gmp::randstate_t`] have been
    changed to reflect that [GMP] can leave some fields unused and
    uninitialized.
  * The experimental feature `use-system-libs` was added.

[`gmp::randstate_t`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/struct.randstate_t.html

### Other releases

Details on other releases can be found in [*RELEASES.md*].

## Basic features

This crate contains three modules:

  * [`gmp`] provides external FFI bindings to [GMP].
  * [`mpfr`] provides external FFI bindings to [MPFR].
  * [`mpc`] provides external FFI bindings to [MPC].

The versions provided by this crate release are [GMP] version 6.2.0,
[MPFR] version 4.0.2-p1, and [MPC] version 1.1.0.

If you want a high-level API, consider using [Rug][rug crate], a crate
which provides integers and floating-point numbers with arbitrary
precision and correct rounding:

  * [`Integer`] is a bignum integer with arbitrary precision,
  * [`Rational`] is a bignum rational number with arbitrary precision,
  * [`Float`] is a multi-precision floating-point number with correct
    rounding, and
  * [`Complex`] is a multi-precision complex number with correct
    rounding.

### Name prefixes

Since modules and enumerated types provide namespacing, most prefixes
in the C names are removed. However, when the prefix is not a whole
word it is not removed. For example [`mp_set_memory_functions`]
becomes [`gmp::set_memory_functions`], but [`mpz_init`] becomes
[`gmp::mpz_init`] not `gmp::z_init`, and [`MPFR_RNDN`] in
[`enum MPFR_RND_T`] becomes [`mpfr::rnd_t::RNDN`] not
`mpfr::rnd_t::N`. Also, the types [`mpfr::mpfr_t`] and [`mpc::mpc_t`]
are *not* shortened to `mpfr::t` or `mpc::t`.

### Types

Unlike in the C libraries, the types [`gmp::mpz_t`], [`gmp::mpq_t`],
[`gmp::mpf_t`], [`gmp::randstate_t`], [`mpfr::mpfr_t`] and
[`mpc::mpc_t`] are defined directly as structs, not as single-element
arrays.

### Undocumented or obsolete functions

The bindings do not cover undocumented or obsolete functions and
macros.

## Using gmp-mpfr-sys

The gmp-mpfr-sys crate is available on [crates.io][sys crate]. To use
gmp-mpfr-sys in your crate, add it as a dependency inside
[*Cargo.toml*]:

```toml
[dependencies]
gmp-mpfr-sys = "1.2"
```

This crate required rustc version 1.37.0 or later.

If the C libraries have a major version bump with some deprecated
functions removed, but no features are removed in the Rust bindings,
then gmp-mpfr-sys will have a minor version bump rather than a major
version bump. This allows more compatiblity across crates that use the
Rust bindings but do not use the C libraries directly.

If on the other hand a dependent crate makes use of internal
implementation details, or includes a C library that directly uses the
header (*.h*) and library (*.a*) files built using C, it can be a good
idea to depend on version `"~1.2"` instead of version `"1.2"` in order
to ensure backwards compatibility at the C level as well.

## Optional features

The gmp-mpfr-sys crate has two optional features:

 1. `mpfr`, enabled by default. Required to include the [MPFR]
    library.
 2. `mpc`, enabled by default. Required to include the [MPC] library.
    This feature requires the `mpfr` feature.

The [GMP] library is always included.

The two optional features are enabled by default; to use features
selectively, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.gmp-mpfr-sys]
version = "1.2"
default-features = false
features = ["mpfr"]
```

Here only the `mpfr` feature is selected.

## Experimental optional features

It is not considered a breaking change if experimental features are
removed. The removal of experimental features would however require a
minor version bump.

There is one experimental feature:

 1. `use-system-libs`, disabled by default. This is *not* supported on
    Windows. Using this feature, the system libraries for [GMP], and
    [MPFR] and [MPC] if enabled, will be used instead of building them
    from source. The versions must be compatible with the versions
    provided by the crate.

## Metadata

The gmp-mpfr-sys crate passes some metadata to its dependents:

 1. `DEP_GMP_LIMB_BITS` contains the number of bits per limb, which is
    32 or 64.
 2. `DEP_GMP_OUT_DIR` contains the path of a directory that contains
    two subdirectories: the first subdirectory is named *lib* and
    contains the generated library (*.a*) files, and the second
    subdirectory is named *include* and contains the corresponding
    header (*.h*) files.
 3. `DEP_GMP_LIB_DIR` contains the path of the *lib* subdirectory of
    the `DEP_GMP_OUT_DIR` directory.
 4. `DEP_GMP_INCLUDE_DIR` contains the path of the *include*
    subdirectory of the `DEP_GMP_OUT_DIR` directory.

A dependent crate can use these environment variables in its build
script.

## Building on GNU/Linux

To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`,
`make` and `m4` installed on your system. For example on Fedora:

```sh
sudo dnf install diffutils gcc make m4
```

## Building on macOS

To build on macOS, you need the command-line developer tools. To
install them, run the following command in a terminal:

```sh
xcode-select --install
```

## Building on Windows

You can build on Windows with the Rust GNU toolchain and an up-to-date
MSYS2 installation. Some steps for a 64-bit environment are listed
below. (32-bit: Changes for a 32-bit environment are written in
brackets like this comment.)

To install MSYS2:

 1. Install MSYS2 using the [installer][msys].

 2. Launch the MSYS2 MinGW 64-bit terminal from the start
    menu. (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)

 3. Install the required tools.

    ```sh
    pacman -S pacman-mirrors
    pacman -S diffutils make mingw-w64-x86_64-gcc
    ```

    (32-bit: Install `mingw-w64-i686-gcc` instead of
    `mingw-w64-x86_64-gcc`.)

Then, to build a crate with a dependency on this crate:

 1. Launch the MSYS MinGW 64-bit terminal from the start menu.
    (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)

 2. Change to the crate directory.

 3. Build the crate using `cargo`.

## Caching the built C libraries

Building the C libraries can take some time. In order to save
compilation time, the built libraries are cached in the user’s cache
directory as follows:

  * on GNU/Linux: inside `$XDG_CACHE_HOME/gmp-mpfr-sys` or
    `$HOME/.cache/gmp-mpfr-sys`
  * on macOS: inside `$HOME/Library/Caches/gmp-mpfr-sys`
  * on Windows: inside `{FOLDERID_LocalAppData}\gmp-mpfr-sys`

To use a different directory, you can set the environment variable
`GMP_MPFR_SYS_CACHE` to the desired cache directory. Setting the
`GMP_MPFR_SYS_CACHE` variable to an empty string will disable caching.

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*RELEASES.md*]: https://gitlab.com/tspiteri/gmp-mpfr-sys/blob/master/RELEASES.md
[GMP]: https://gmplib.org/
[GNU GPL]: https://www.gnu.org/licenses/gpl-3.0.html
[GNU LGPL]: https://www.gnu.org/licenses/lgpl-3.0.en.html
[GNU]: https://www.gnu.org/
[MPC]: http://www.multiprecision.org/mpc/
[MPFR]: https://www.mpfr.org/
[`Complex`]: https://docs.rs/rug/*/rug/struct.Complex.html
[`Float`]: https://docs.rs/rug/*/rug/struct.Float.html
[`Integer`]: https://docs.rs/rug/*/rug/struct.Integer.html
[`MPFR_RNDN`]: https://tspiteri.gitlab.io/gmp-mpfr-sys/mpfr/MPFR-Basics.html#Rounding-Modes
[`Rational`]: https://docs.rs/rug/*/rug/struct.Rational.html
[`enum MPFR_RND_T`]: https://tspiteri.gitlab.io/gmp-mpfr-sys/mpfr/MPFR-Basics.html#index-mpfr_005frnd_005ft
[`gmp::mpf_t`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/struct.mpf_t.html
[`gmp::mpq_t`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/struct.mpq_t.html
[`gmp::mpz_init`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/fn.mpz_init.html
[`gmp::mpz_t`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/struct.mpz_t.html
[`gmp::randstate_t`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/struct.randstate_t.html
[`gmp::set_memory_functions`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/fn.set_memory_functions.html
[`gmp`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/index.html
[`mp_set_memory_functions`]: https://tspiteri.gitlab.io/gmp-mpfr-sys/gmp/Custom-Allocation.html#index-mp_005fset_005fmemory_005ffunctions
[`mpc::mpc_t`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/mpc/struct.mpc_t.html
[`mpc`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/mpc/index.html
[`mpfr::mpfr_t`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/mpfr/struct.mpfr_t.html
[`mpfr::rnd_t::RNDN`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/mpfr/enum.rnd_t.html#variant.RNDN
[`mpfr`]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/mpfr/index.html
[`mpz_init`]: https://tspiteri.gitlab.io/gmp-mpfr-sys/gmp/Integer-Functions.html#index-mpz_005finit
[msys]:     https://msys2.github.io/
[rug crate]: https://crates.io/crates/rug
[sys crate]: https://crates.io/crates/gmp-mpfr-sys
