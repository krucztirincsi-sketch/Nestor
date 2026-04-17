#[cfg(test)]
mod tests {
    use super::super::Token;
    use super::super::Lexer;

    #[test]
    fn test_lexer_basic() {
        let mut lexer = Lexer::new("fn main() {}");
        assert_eq!(lexer.next_token(), Token::Fn);
        assert_eq!(lexer.next_token(), Token::Ident("main".to_string()));
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::RBrace);
    }
}
