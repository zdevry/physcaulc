mod lex;
mod parse;

#[derive(Debug, Clone)]
pub enum TokenKind {
    Integer(i32),
    Float(f64),
    Word(String),
    Symbol(char),
    End,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub reason: String,
    pub start: usize,
    pub end: usize,
}

pub fn lex(s: &str) -> Result<Vec<Token>, ParseError> {
    let mut l = lex::Lexer {
        chars: s.chars().peekable(),
        buffer: String::new(),
        strpos: 0,
        token_start: 0,
    };

    let mut result = Vec::new();
    loop {
        let token = l.lex_token()?;
        let is_end = matches!(token.kind, TokenKind::End);
        result.push(token);
        if is_end {
            break;
        }
    }

    Ok(result)
}
