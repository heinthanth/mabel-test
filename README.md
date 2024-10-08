# The Mabel Compiler

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
            SemCheck[✅ Semantic Checker] --> AnnotatedAST{{✅ Annotated AST}}
            AnnotatedAST --> Compiler[🚧 Compiler]
        end
        subgraph LLVMCodeGenGraph[ ]
            direction TB
            LLVMIR{{LLVM IR}} --> JITExecutionEngine[JIT Execution Engine]
            LLVMIR --> TargetExecutable[Target Executable]
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

As of now, I can't write test cases for all modules since working with AST is very inconvenient given that AST contains kind, token, etc.

Those files will be later tested with [mabel source codes](tests) when, at least, the JIT execution engine is finished.

## License

Mabel is distributed under MIT license. This will be updated later based on used dependencies.

See [LICENSE](LICENSE) for details.
