# Aion Programming Language

Aion is a high-performance, AI-native programming language designed for Large Language Models and Heterogeneous Computing. It combines the safety of Rust, the concurrency of Go, and the mathematical expressiveness of Python/Mojo.

## Features

- **AI-First**: First-class support for Tensors and Automatic Differentiation (`grad`).
- **Concurrency**: Go-style lightweight threads (`spawn`) and channels.
- **Modern Compiler**: Built on Rust and designed to target MLIR/LLVM for CPU/GPU acceleration.
- **Nested Logic**: Optimized support for deeply nested closures and modular structures.

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd aion-compiler

# Build the compiler
cargo build --release
```

### Running the Compiler

You can use the Aion compiler to parse and generate MLIR from an Aion source file:

```bash
# Run on the demo file
cargo run -- demo.ai
```

## Language Overview

### AI Inference
```aion
let input = tensor::randn([1, 512]);
let weights = tensor::randn([512, 512]);
let output = grad(model.forward)(input, weights);
```

### Concurrency
```aion
spawn fn() {
    let result = compute();
    println("Done");
}
```

## Project Structure

- `src/lexer`: Tokenizes Aion source code.
- `src/parser`: Generates an Abstract Syntax Tree (AST).
- `src/mlir`: Lowers AST to MLIR representation.
- `src/codegen`: LLM-specific optimizations (Operator Fusion).
- `Aion_Spec.md`: Full language specification.
