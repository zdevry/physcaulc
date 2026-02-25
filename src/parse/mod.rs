use crate::eval::Node;

mod expr;
mod lex;
pub mod units;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Integer(u32),
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

pub fn parse(tokens: Vec<Token>) -> Result<Node, ParseError> {
    let mut position = 0;
    let result = expr::expr(&tokens, &mut position)?;

    let final_token = &tokens[position];

    match final_token.kind {
        TokenKind::End => Ok(result),
        _ => Err(ParseError {
            reason: "not end".into(),
            start: final_token.start,
            end: final_token.end,
        }),
    }
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

pub fn curr_token<'a>(tokens: &'a Vec<Token>, position: &mut usize) -> &'a Token {
    &tokens[*position]
}

pub fn step_token(tokens: &Vec<Token>, position: &mut usize) {
    if *position < tokens.len() - 1 {
        *position += 1
    }
}

pub fn expect(
    kind: TokenKind,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<(usize, usize), ParseError> {
    let curr = curr_token(tokens, position);
    if curr.kind != kind {
        return Err(ParseError {
            reason: "unexpected".into(),
            start: curr.start,
            end: curr.end,
        });
    }

    step_token(tokens, position);
    Ok((curr.start, curr.end))
}

pub fn optional(
    kind: TokenKind,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Option<(usize, usize)> {
    let curr = curr_token(tokens, position);
    if curr.kind != kind {
        None
    } else {
        step_token(tokens, position);
        Some((curr.start, curr.end))
    }
}
