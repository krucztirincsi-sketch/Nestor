mod lexer;
mod ast;
mod parser;
mod codegen;
mod mlir;

use lexer::Lexer;
use parser::Parser;
use mlir::MLIRGenerator;
use codegen::LLMOptimizer;

use std::fs;

fn main() {
    let input = fs::read_to_string("demo.ai").expect("Failed to read demo.ai");
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let mut program = parser.parse_program();

    let optimizer = LLMOptimizer;
    optimizer.optimize(&mut program);

    let mut generator = MLIRGenerator::new();
    generator.generate(&program);

    println!("--- Generated MLIR ---\n{}", generator.output);
}
