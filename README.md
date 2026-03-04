# gen-fmt

Code formatter for [GenExpr](https://docs.cycling74.com/max8/vignettes/gen_genexpr), Cycling '74's DSP expression language used in Max/MSP gen~ patchers.

Built on [Topiary](https://github.com/tweag/topiary) and [tree-sitter-genexpr](https://github.com/isabelgk/tree-sitter-genexpr).

## Install

### From source

```sh
cargo install --path .
```

### Pre-built binaries

Pre-built binaries are available on the [releases page](https://github.com/isabelgk/gen-fmt/releases).

On macOS, downloaded binaries will be blocked by Gatekeeper. To allow it:

```sh
xattr -d com.apple.quarantine gen-fmt
```

Or right-click the binary in Finder and choose Open.

## Usage

Format a file in place:

```sh
gen-fmt -i patch.genexpr
```

Format to stdout:

```sh
gen-fmt patch.genexpr
```

Read from stdin:

```sh
cat patch.genexpr | gen-fmt
```

## C library

gen-fmt can be built as a C-compatible library for embedding in C (or C++) programs.

### Build

```sh
cargo build --release
```

This produces:

- `target/release/libgen_fmt.dylib` (macOS) / `libgen_fmt.so` (Linux) — dynamic library
- `target/release/libgen_fmt.a` — static library

### API

Include `include/gen_fmt.h` and link against one of the above:

```c
#include "include/gen_fmt.h"

char *result = gen_fmt_format(input, 0, 0);
if (result) {
    // use result...
    gen_fmt_free(result);
}
```

`gen_fmt_format` returns a newly-allocated string on success, or `NULL` on error. Always free it with `gen_fmt_free` instead of  `free()`.

### Link

```sh
# dynamic
cc main.c -Iinclude -Ltarget/release -lgen_fmt -o main

# static
cc main.c -Iinclude target/release/libgen_fmt.a -o main
```

## Configuration

gen-fmt currently has no user-configurable options. Support for customizing formatting style is planned.

## Development

```sh
cargo test       # run integration tests
cargo build      # compile
```

Formatting rules live in `queries/genexpr.scm`.
