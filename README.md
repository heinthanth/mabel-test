# The Mabel Compiler

[![CI](https://github.com/heinthanth/mabel/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/heinthanth/mabel/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/heinthanth/mabel/branch/main/graph/badge.svg?token=L4P15DZ1UM)](https://codecov.io/gh/heinthanth/mabel)
![Rust LOC](https://tokei.rs/b1/github/heinthanth/mabel?category=code&type=Rust&label=Lines%20of%20Rust%20Code&color=FF281C1C&logo=https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/main/icons/rust.svg)

This is the main repo for the WIP Mabel compiler.

## Development

For ease of development, there're several [cargo-make](https://github.com/sagiegurari/cargo-make) tasks.

```mermaid
---
title: Current Roadmap
---
graph TB;
    subgraph roadmap[ ]
        direction TB
        subgraph ParserGraph[ ]
            direction LR
            Lexer[✅ Lexer] --> Tokens{{✅ Tokens}} --> Parser[✅ Parser] --> AST{{✅ AST}}
        end
        subgraph CompilerGraph[ ]
            direction LR
            SemCheck[🚧 Semantic Checker] --> Compiler
        end
        subgraph LLVMCodeGenGraph[ ]
            direction TB
            LLVMIR{{LLVM IR}} --> JITExecutionEngine[JIT Execution Engine]
            LLVMIR{{LLVM IR}} --> TargetExecutable[Target Executable]
        end
        subgraph JsCdoeGenGraph[ ]
            direction TB
            JsAst{{JavaScript AST}} --> JavaScript[JavaScript Code]
        end
        Start([✅ Lib Frontend]) -->  ParserGraph
        ParserGraph --> CompilerGraph
        CompilerGraph --> LLVMCodeGenGraph
        CompilerGraph --> JsCdoeGenGraph
    end
```

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

As of now, the following files are excluded from coverage since it's very inconvenient to write unit test cases.

- [main.rs](src/main.rs)
- [compiler/mod.rs](src/compiler/mod.rs)
- [parser/ast.rs](src/parser/ast.rs)
- [parser/mod.rs](src/parser/mod.rs)

Those files will be later tested with [mabel source codes](tests/scripts) when, at least, the JIT execution engine is finished.

## License

Mabel is distributed under MIT license. This will be updated later based on used dependencies.

See [LICENSE](LICENSE) for details.
