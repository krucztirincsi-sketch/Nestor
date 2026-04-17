#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Binary(Box<Expr>, Op, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Closure(Vec<String>, Box<Block>),
    TensorInit(Vec<usize>, String), // Shape, type
    Path(Vec<String>),
    MemberAccess(Box<Expr>, String),
    Grad(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add, Sub, Mul, Div,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Return(Expr),
    Fn(String, Vec<String>, Block),
    Spawn(Expr),
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub enum Decl {
    Fn(String, Vec<String>, Block),
    Struct(String, Vec<(String, String)>),
    Model(String, Vec<(String, String)>, Vec<Decl>),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub decls: Vec<Decl>,
}
