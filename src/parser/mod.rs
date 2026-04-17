use crate::lexer::{Lexer, Token};
use crate::ast::*;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self { lexer, cur_token, peek_token }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut decls = Vec::new();
        while self.cur_token != Token::EOF {
            if let Some(decl) = self.parse_decl() {
                decls.push(decl);
            }
            self.next_token();
        }
        Program { decls }
    }

    fn parse_decl(&mut self) -> Option<Decl> {
        match self.cur_token {
            Token::Fn => self.parse_fn_decl(),
            Token::Struct => self.parse_struct_decl(),
            Token::Model => self.parse_model_decl(),
            _ => None,
        }
    }

    fn parse_fn_decl(&mut self) -> Option<Decl> {
        self.next_token(); // skip fn
        if let Token::Ident(name) = &self.cur_token {
            let name = name.clone();
            self.next_token(); // skip name
            let params = self.parse_params();
            let block = self.parse_block();
            Some(Decl::Fn(name, params, block))
        } else {
            None
        }
    }

    fn parse_params(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        if self.cur_token == Token::LParen {
            self.next_token();
            while self.cur_token != Token::RParen && self.cur_token != Token::EOF {
                if let Token::Ident(name) = &self.cur_token {
                    params.push(name.clone());
                    self.next_token();
                    if self.cur_token == Token::Comma { self.next_token(); }
                } else {
                    self.next_token();
                }
            }
            self.next_token(); // skip )
        }
        params
    }

    fn parse_model_decl(&mut self) -> Option<Decl> {
        self.next_token(); // skip model
        if let Token::Ident(name) = &self.cur_token {
            let name = name.clone();
            self.next_token(); // skip name
            self.next_token(); // skip {
            let mut decls = Vec::new();
            while self.cur_token != Token::RBrace && self.cur_token != Token::EOF {
                if let Some(decl) = self.parse_decl() {
                    decls.push(decl);
                } else {
                    self.next_token();
                }
            }
            Some(Decl::Model(name, vec![], decls))
        } else {
            None
        }
    }

    fn parse_struct_decl(&mut self) -> Option<Decl> {
        self.next_token(); // skip struct
        if let Token::Ident(name) = &self.cur_token {
            let name = name.clone();
            self.next_token(); // skip name
            self.next_token(); // skip {
            // Placeholder for fields
            while self.cur_token != Token::RBrace && self.cur_token != Token::EOF {
                self.next_token();
            }
            Some(Decl::Struct(name, vec![]))
        } else {
            None
        }
    }

    fn parse_block(&mut self) -> Block {
        let mut stmts = Vec::new();
        if self.cur_token == Token::LBrace {
            self.next_token();
            while self.cur_token != Token::RBrace && self.cur_token != Token::EOF {
                if let Some(stmt) = self.parse_stmt() {
                    stmts.push(stmt);
                }
                self.next_token();
            }
        }
        Block { stmts }
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.cur_token {
            Token::Let => {
                self.next_token();
                if let Token::Ident(name) = &self.cur_token {
                    let name = name.clone();
                    self.next_token();
                    if self.cur_token == Token::Assign {
                        self.next_token();
                        let expr = self.parse_expr();
                        Some(Stmt::Let(name, expr))
                    } else { None }
                } else { None }
            }
            Token::Return => {
                self.next_token();
                Some(Stmt::Return(self.parse_expr()))
            }
            Token::Spawn => {
                self.next_token();
                Some(Stmt::Spawn(self.parse_expr()))
            }
            _ => {
                let expr = self.parse_expr();
                if expr != Expr::Literal(Literal::Int(0)) {
                    Some(Stmt::Expr(expr))
                } else {
                    None
                }
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = match &self.cur_token {
            Token::Int(i) => {
                let val = *i;
                self.next_token();
                Expr::Literal(Literal::Int(val))
            }
            Token::Float(f) => {
                let val = *f;
                self.next_token();
                Expr::Literal(Literal::Float(val))
            }
            Token::Ident(s) => {
                let s = s.clone();
                self.next_token();
                if self.cur_token == Token::DoubleColon {
                    let mut path = vec![s];
                    while self.cur_token == Token::DoubleColon {
                        self.next_token();
                        if let Token::Ident(name) = &self.cur_token {
                            path.push(name.clone());
                            self.next_token();
                        }
                    }
                    return Expr::Path(path);
                } else {
                    // Don't next_token() here, it's done below if needed or we keep it for now
                    // Actually, let's just return Ident(s) and let the loop handle it
                    Expr::Ident(s)
                }
            }
            Token::Pipe => return self.parse_closure(),
            Token::Grad => {
                self.next_token();
                self.next_token(); // (
                let expr = self.parse_expr();
                if self.cur_token == Token::RParen { self.next_token(); }
                return Expr::Grad(Box::new(expr));
            }
            Token::LBracket => {
                // Simplified tensor shape parsing
                while self.cur_token != Token::RBracket && self.cur_token != Token::EOF {
                    self.next_token();
                }
                if self.cur_token == Token::RBracket { self.next_token(); }
                Expr::Literal(Literal::Int(0)) // Placeholder
            }
            _ => {
                let tok = self.cur_token.clone();
                self.next_token();
                return Expr::Literal(Literal::Int(0));
            }
        };

        // This is a very simplified expression parser
        loop {
            match &self.cur_token {
                Token::LParen => {
                    self.next_token();
                    let mut args = Vec::new();
                    while self.cur_token != Token::RParen && self.cur_token != Token::EOF {
                        args.push(self.parse_expr());
                        if self.cur_token == Token::Comma { self.next_token(); }
                    }
                    if self.cur_token == Token::RParen {
                        self.next_token();
                    }
                    left = Expr::Call(Box::new(left), args);
                }
                Token::Dot => {
                    self.next_token();
                    if let Token::Ident(name) = &self.cur_token {
                        left = Expr::MemberAccess(Box::new(left), name.clone());
                        self.next_token();
                    }
                }
                Token::Star | Token::Plus | Token::Minus | Token::Slash => {
                    let op = match self.cur_token {
                        Token::Star => Op::Mul,
                        Token::Plus => Op::Add,
                        Token::Minus => Op::Sub,
                        _ => Op::Div,
                    };
                    self.next_token();
                    let right = self.parse_expr();
                    left = Expr::Binary(Box::new(left), op, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_closure(&mut self) -> Expr {
        self.next_token(); // skip |
        let mut params = Vec::new();
        while self.cur_token != Token::Pipe && self.cur_token != Token::EOF {
            if let Token::Ident(name) = &self.cur_token {
                params.push(name.clone());
            }
            self.next_token();
        }
        self.next_token(); // skip |
        let block = self.parse_block();
        Expr::Closure(params, Box::new(block))
    }
}
