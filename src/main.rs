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
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: aion-compiler <file.ai>");
        return;
    }
    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let mut program = parser.parse_program();

    let optimizer = LLMOptimizer;
    optimizer.optimize(&mut program);

    let mut generator = MLIRGenerator::new();
    generator.generate(&program);

    println!("--- Generated MLIR ---\n{}", generator.output);
}
