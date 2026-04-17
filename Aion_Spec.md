# Aion Language Specification (v0.1.0-alpha)

Aion is a high-performance, AI-native programming language designed for the era of Large Language Models and Heterogeneous Computing. It combines the safety of Rust, the concurrency of Go, and the mathematical expressiveness of Python/Mojo, all powered by an MLIR-based compiler.

## 1. Core Philosophy
- **Data-Oriented**: Focus on data layout and transformation rather than object hierarchies.
- **AI-First**: Tensors and Gradients are first-class citizens.
- **Safety without Friction**: Simplified ownership model to ensure memory safety without the steep learning curve of traditional borrow checking.
- **Highly Nested**: Native support for deeply nested closures and modular logic.

## 2. Syntax & Type System
Aion uses a rigorous, curly-brace syntax.

### 2.1 Primitive Types
- `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`
- `f16`, `f32`, `f64`, `bf16` (AI-focused floating points)
- `bool`, `str`

### 2.2 First-Class Tensors
Tensors are native types with shape information.
```aion
let a: Tensor<f32, [128, 512]> = Tensor::zeros();
let b = a * 2.0; // Element-wise operation
```

### 2.3 Nested Closures
Deeply nested logic is supported with efficient capture.
```aion
fn factory(x: f32) -> (fn(f32) -> f32) {
    return |y| {
        return |z| {
            x + y + z
        };
    };
}
```

## 3. AI & Automatic Differentiation
### 3.1 Grad Keyword
Built-in reverse-mode automatic differentiation.
```aion
fn loss(w: Tensor<f32, [N]>, x: Tensor<f32, [N]>) -> f32 {
    return sum(w * x);
}

let w = Tensor::randn([10]);
let x = Tensor::randn([10]);
let gradients = grad(loss)(w, x); // Returns gradients with respect to weights
```

### 3.2 LLM Native Support
High-level `model` construct that the compiler understands for optimization (e.g., operator fusion).
```aion
model Transformer {
    layers: Vec<Layer>,

    fn forward(self, input: Tensor) -> Tensor {
        // Compiler automatically optimizes this path
        return self.layers.reduce(|acc, l| l.forward(acc), input);
    }
}
```

## 4. Concurrency (Go-style)
Uses `spawn` for lightweight threads and `chan` for communication.
```aion
fn worker(c: chan<i32>) {
    c.send(42);
}

fn main() {
    let c = chan<i32>::new();
    spawn worker(c);
    let val = c.recv();
}
```

## 5. Memory Safety
Aion uses **"Value Ownership"** with a "Scope-Based Borrowing" model.
- Variables own their data by default.
- Passing to functions defaults to "move" unless marked as `ref`.
- The compiler uses escape analysis to determine if data needs to live on the heap.

## 6. Compiler Architecture (MLIR)
1. **Source** -> **AST**
2. **AST** -> **Aion Dialect (MLIR)**: High-level language constructs.
3. **Aion Dialect** -> **Linalg/TOSA Dialects**: Tensor operations optimization.
4. **Lowering** -> **LLVM IR** -> **Machine Code (CPU/GPU)**.
