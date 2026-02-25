use super::{ParseError, Token, TokenKind};
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub buffer: String,
    pub strpos: usize,
    pub token_start: usize,
}

impl<'a> Lexer<'a> {
    pub fn lex_token(&mut self) -> Result<Token, ParseError> {
        self.consume_while(char::is_whitespace);
        self.start_token();
        let currchar = self.peek_char();

        if currchar == '\0' {
            return Ok(self.create_token(TokenKind::End));
        }

        if currchar.is_alphabetic() {
            Ok(self.lex_word())
        } else if currchar.is_numeric() || currchar == '.' {
            Ok(self.lex_number())
        } else if currchar == '<' {
            self.lex_string()
        } else {
            self.next_char();
            Ok(self.create_token(TokenKind::Symbol(currchar)))
        }
    }

    fn lex_number(&mut self) -> Token {
        self.consume_while(|c| c.is_numeric() || c == '_');

        // Buffer will always be populated by at least one numeric char here
        // because of the initial requirement of char::is_numeric for this function to be called
        if self.peek_char() != '.' {
            let clean_str = self
                .get_buffer_str()
                .chars()
                .filter(|&c| c != '_')
                .collect::<String>();
            let result = clean_str.parse::<u32>().unwrap();
            return self.create_token(TokenKind::Integer(result));
        }

        todo!()
    }

    fn lex_word(&mut self) -> Token {
        self.consume_while(|c| c.is_alphanumeric() || c == '_');
        self.create_token(TokenKind::Word(self.get_buffer_str()))
    }

    fn lex_string(&mut self) -> Result<Token, ParseError> {
        self.next_char(); // <

        if !self.consume_while(|c| c != '>') {
            return Err(ParseError {
                reason: "String is unclosed".into(),
                start: self.strpos,
                end: self.strpos,
            });
        }

        let mut word = self.get_buffer_str();
        word.remove(0); // remove leading <
        self.next_char(); // >

        Ok(self.create_token(TokenKind::Word(word)))
    }

    fn peek_char(&mut self) -> char {
        self.chars.peek().cloned().unwrap_or('\0')
    }

    fn next_char(&mut self) -> char {
        let c = match self.chars.next() {
            Some(c) => c,
            None => return '\0',
        };

        self.strpos += 1;
        self.buffer.push(c);
        c
    }

    fn start_token(&mut self) {
        self.buffer.clear();
        self.token_start = self.strpos;
    }

    fn create_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            start: self.token_start,
            end: self.strpos,
        }
    }

    fn get_buffer_str(&self) -> String {
        self.buffer.clone()
    }

    fn consume_while<P>(&mut self, cond: P) -> bool
    where
        P: Fn(char) -> bool,
    {
        loop {
            let c = self.peek_char();
            if !cond(c) {
                return true;
            }
            if c == '\0' {
                return false;
            }
            self.next_char();
        }
    }
}
