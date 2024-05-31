# The Mabel Compiler

[![CI](https://github.com/heinthanth/mabel/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/heinthanth/mabel/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/heinthanth/mabel/branch/main/graph/badge.svg?token=L4P15DZ1UM)](https://codecov.io/gh/heinthanth/mabel)

This is the main repo for the WIP Mabel compiler.

## Development

For ease of development, there're several [cargo-make](https://github.com/sagiegurari/cargo-make) tasks.

### Build and Run

This is equivalent to running `RUSTFLAGS="-Awarnings" cargo run -q -- [...args]`.
This will suppress rustc warnings and cargo outputs to make it feel like running the executable itself.

```
cargo make mabel-dev [...args]
```

### Testing

This is equivalent to running `cargo test [...args]`.

```
cargo make test [...args]
```

### Coverage

This is equivalent to running `cargo llvm-cov args`. [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) needs to be installed.

```
cargo make coverage [...args]
```

## License

Mabel is distributed under MIT license. This will be updated later based on used dependencies.

See [LICENSE](LICENSE) for details.
