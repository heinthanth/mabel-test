# The Mabel Compiler

This is the main repo for the WIP Mabel compiler.

## Development

For ease of development, there're several [cargo-make](https://github.com/sagiegurari/cargo-make) tasks.

### Build and Run

This is equivalent to running `RUSTFLAGS="-Awarnings" cargo run -q -- [...args]`.

```
cargo make mabel-dev [...args]   # to build and run the mabel compiler with arguments: args
```

### Testing

This is equivalent to running `cargo test [...args]`.

```
cargo make test [...args]       # to run unit tests
```

### Coverage

This is equivalent to running `cargo llvm-cov args`. [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) needs to be installed.

```
cargo make coverage [...args]   # to generate coverage report using cargo-llvm-cov
```

## License

Mabel is distributed under MIT license. This will be updated later based on used dependencies.

See [LICENSE](LICENSE) for details.
