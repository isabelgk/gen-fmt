# gen-fmt

Code formatter for [GenExpr](https://docs.cycling74.com/max8/vignettes/gen_genexpr), Cycling '74's DSP expression language used in Max/MSP gen~ patchers.

Built on [Topiary](https://github.com/tweag/topiary) and [tree-sitter-genexpr](https://github.com/isabelgk/tree-sitter-genexpr).

## Install

```sh
cargo install --path .
```

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

## Development

```sh
cargo test       # run integration tests
cargo build      # compile
```

Formatting rules live in `queries/genexpr.scm`.
