use super::{ParseError, Token, TokenKind, curr_token, expect, step_token};
use crate::{
    eval::{BinaryOp, Node, NodeContent},
    rational::Rational,
};

pub fn expr(tokens: &Vec<Token>, position: &mut usize) -> Result<Node, ParseError> {
    series(tokens, position, 3)
}

fn get_binary_op_info(symbol: char) -> Option<(BinaryOp, usize)> {
    match symbol {
        '*' => Some((BinaryOp::Mul, 1)),
        ':' => Some((BinaryOp::Div, 1)),
        '/' => Some((BinaryOp::Div, 2)),
        '+' => Some((BinaryOp::Add, 3)),
        '-' => Some((BinaryOp::Sub, 3)),
        _ => None,
    }
}

pub fn series(
    tokens: &Vec<Token>,
    position: &mut usize,
    prec_level: usize,
) -> Result<Node, ParseError> {
    if prec_level == 0 {
        return term(tokens, position);
    }

    let mut result = series(tokens, position, prec_level - 1)?;

    loop {
        let op_token = curr_token(tokens, position);

        let (op, level) = match op_token.kind {
            TokenKind::Symbol(c) => match get_binary_op_info(c) {
                Some(info) => info,
                None => return Ok(result),
            },
            _ => return Ok(result),
        };
        if level != prec_level {
            return Ok(result);
        }

        step_token(tokens, position);

        let rhs = series(tokens, position, prec_level - 1)?;

        result = Node {
            content: NodeContent::Binary(Box::new(result), op, Box::new(rhs)),
            start: op_token.start,
            end: op_token.end,
        }
    }
}

pub fn term(tokens: &Vec<Token>, position: &mut usize) -> Result<Node, ParseError> {
    let inner = atom(tokens, position)?;

    let first_suffix_token = curr_token(tokens, position);
    match first_suffix_token.kind {
        TokenKind::Symbol('[') => {
            let unit_terms = super::units::parse_units(tokens, position)?;
            Ok(Node {
                content: NodeContent::Unary(
                    crate::eval::UnaryOp::Units(unit_terms),
                    Box::new(inner),
                ),
                start: first_suffix_token.start,
                end: first_suffix_token.end,
            })
        }
        _ => Ok(inner),
    }
}

pub fn atom(tokens: &Vec<Token>, position: &mut usize) -> Result<Node, ParseError> {
    let curr = curr_token(tokens, position);
    step_token(tokens, position);

    match &curr.kind {
        &TokenKind::Integer(n) => Ok(Node {
            content: NodeContent::Value(Rational::new(n as i32, 1).into()),
            start: curr.start,
            end: curr.end,
        }),
        TokenKind::Word(s) => Ok(Node {
            content: NodeContent::Variable(s.clone()),
            start: curr.start,
            end: curr.end,
        }),
        TokenKind::Symbol('(') => {
            let inner = expr(tokens, position);
            expect(TokenKind::Symbol(')'), tokens, position)?;
            inner
        }
        _ => Err(ParseError {
            reason: "unexpected token".into(),
            start: curr.start,
            end: curr.end,
        }),
    }
}
