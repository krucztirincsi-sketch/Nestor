use crate::ast::*;

pub struct MLIRGenerator {
    pub output: String,
}

impl MLIRGenerator {
    pub fn new() -> Self {
        Self { output: String::new() }
    }

    pub fn generate(&mut self, program: &Program) {
        self.output.push_str("module {\n");
        for decl in &program.decls {
            self.generate_decl(decl);
        }
        self.output.push_str("}\n");
    }

    fn generate_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Fn(name, params, block) => {
                let params_str = params.join(", ");
                self.output.push_str(&format!("  func.func @{}( {} ) {{\n", name, params_str));
                self.generate_block(block);
                self.output.push_str("    return\n");
                self.output.push_str("  }\n");
            }
            Decl::Struct(name, _) => {
                self.output.push_str(&format!("  // struct {}\n", name));
            }
            Decl::Model(name, _, inner_decls) => {
                self.output.push_str(&format!("  // model {}\n", name));
                for d in inner_decls {
                    self.generate_decl(d);
                }
            }
        }
    }

    fn generate_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.generate_stmt(stmt);
        }
    }

    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let(name, expr) => {
                self.output.push_str(&format!("    // let {} = {:?}\n", name, expr));
            }
            Stmt::Return(expr) => {
                self.output.push_str(&format!("    // return {:?}\n", expr));
            }
            Stmt::Spawn(expr) => {
                self.output.push_str(&format!("    // spawn {:?}\n", expr));
            }
            Stmt::Expr(expr) => {
                self.output.push_str(&format!("    // {:?}\n", expr));
            }
            _ => {}
        }
    }
}
