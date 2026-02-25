use super::{ParseError, Token, TokenKind, curr_token, optional, step_token};
use crate::{eval::UnitTerm, rational::Rational};

pub fn parse_term_exponent(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Rational, ParseError> {
    if optional(TokenKind::Symbol('^'), tokens, position).is_none() {
        return Ok(Rational::new(1, 1));
    }

    let negative = optional(TokenKind::Symbol('-'), tokens, position).is_some();

    let numerator_token = curr_token(tokens, position);
    step_token(tokens, position);
    let numerator = match numerator_token.kind {
        TokenKind::Integer(n) => n,
        _ => {
            return Err(ParseError {
                reason: "expected exponent".into(),
                start: numerator_token.start,
                end: numerator_token.end,
            });
        }
    };
    let numerator = if negative {
        -(numerator as i32)
    } else {
        numerator as i32
    };

    // Following '/' not proceeded by integer is the denominator of unit
    // Thus backtrack to here otherwise
    let after_numerator_pos = *position;
    if optional(TokenKind::Symbol('/'), tokens, position).is_none() {
        return Ok(Rational::new(numerator as i32, 1));
    }

    let denom_token = curr_token(tokens, position);
    match denom_token.kind {
        TokenKind::Integer(denom) => {
            step_token(tokens, position);
            Ok(Rational::new(numerator as i32, denom))
        }
        _ => {
            *position = after_numerator_pos;
            Ok(Rational::new(numerator as i32, 1))
        }
    }
}

pub fn parse_units(tokens: &Vec<Token>, position: &mut usize) -> Result<Vec<UnitTerm>, ParseError> {
    step_token(tokens, position); // [

    let mut result = Vec::new();
    let mut in_denom = false;

    loop {
        let curr = curr_token(tokens, position);
        step_token(tokens, position);

        let unit = match &curr.kind {
            TokenKind::Word(s) => s.clone(),
            TokenKind::Symbol(']') => return Ok(result),
            TokenKind::Symbol('/') => {
                if in_denom {
                    return Err(ParseError {
                        reason: "second usage of '/' in units not allowed".into(),
                        start: curr.start,
                        end: curr.end,
                    });
                }
                in_denom = true;
                continue;
            }
            _ => {
                return Err(ParseError {
                    reason: "expected unit".into(),
                    start: curr.start,
                    end: curr.end,
                });
            }
        };

        let exp = parse_term_exponent(tokens, position)?;

        result.push(UnitTerm {
            unit,
            power: if in_denom { exp.negative() } else { exp },
            start: curr.start,
            end: curr.end,
        })
    }
}
