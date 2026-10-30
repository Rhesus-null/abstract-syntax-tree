use std::cell::Cell;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;
use crate::TokenKind;


#[derive(Debug, Copy, Clone)]
pub struct TokenMetadata {
    pub span: (usize, usize),
    pub line: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    token: TokenKind,
    value: &'a [u8],
    metadata: TokenMetadata,
}

impl<'a> Token<'a> {
    pub fn new(token: TokenKind, metadata: TokenMetadata, value: &'a [u8]) -> Self {
        Self {token, value, metadata}
    }
    pub fn get_token_kind(&self) -> TokenKind {
        self.token
    }
    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata
    }
    pub fn get_value(&self) -> &[u8] {
        self.value
    }
}

struct RSpan {
    start: usize,
    end: Rc<Cell<usize>>,
}
impl RSpan {
    fn new(start: usize, end: Rc<Cell<usize>>) -> Self {
        Self { start, end }
    }
    fn as_range(&self) -> Range<usize> {
        if self.start == self.end.get() {
            return self.start..(self.end.get() + 1)
        }
        self.start..self.end.get()

    }
    fn as_range_inclusive(&self) -> RangeInclusive<usize> {
        self.start..=self.end.get()
    }
    fn as_raw(&self) -> (usize, usize) {
        (self.start, self.end.get())
    }
}


pub struct Lexer<'a> {
    tokens: Vec<Token<'a>>,
    raw: &'a[u8],

    parse_error: Vec<&'a str>,
    position: Rc<Cell<usize>>,
    line: usize,

    is_end: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {tokens: Vec::new(), raw: bytes, parse_error: Vec::new(), position: Rc::new(Cell::new(0)), line: 1, is_end: false}
    }

    pub fn parse(mut self) -> Result<Vec<Token<'a>>, Vec<&'a str>> {
        while !self.end()  {
            if *self.get() == b'\n' { self.line += 1; }
            if self.get().is_ascii_whitespace() {
                self.next();
                continue;
            }
            let result = match self.get() {
                t if t.is_ascii_alphabetic() => self.parse_identifier_or_keyword(),
                t if t.is_ascii_digit() => self.parse_digit(),
                t if *t == b'"' => self.parse_str(),
                t if t.is_ascii_punctuation() =>  self.parse_punctuation(),
                _ => Token::new(TokenKind::NONE, TokenMetadata {span: (0, 0), line: 0}, b"") /*TODO: Add error*/
            };
            self.tokens.push(result);
        }

        if self.parse_error.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.parse_error)
        }
    }
    fn parse_identifier_or_keyword(&mut self) -> Token<'a> {
        let r_span = self.record_span();
        loop {
            if !self.get().is_ascii_alphanumeric() || !*self.get() == b'_' {
                break
            }
            if self.raw_end() || self.get().is_ascii_whitespace() {break}
            self.next();
        }
        let result = &self.raw[r_span.as_range()];
        match result {
            b"if" => Token::new(TokenKind::If, TokenMetadata {span: r_span.as_raw(), line: self.line}, result),
            b"let" => Token::new(TokenKind::Let, TokenMetadata {span: r_span.as_raw(), line: self.line}, result),
            _ => Token::new(TokenKind::Identifier, TokenMetadata {span: r_span.as_raw(), line: self.line}, result),
        }
    }
    fn parse_digit(&mut self) -> Token<'a> {
        let r_span = self.record_span();
        loop {
            if  !self.get().is_ascii_digit() {
                break
            }
            if self.raw_end() || self.get().is_ascii_whitespace() { break }
            self.next();
        }
        let result = &self.raw[r_span.as_range()];
        Token::new(TokenKind::LiteralDigit, TokenMetadata {span: r_span.as_raw(), line: self.line}, result)
    }

    fn parse_str(&mut self) -> Token<'a> {
        let r_span = self.record_span();
        let mut c = *self.next().get();
        loop {
            if self.end() {
                return Token::new(TokenKind::NONE, TokenMetadata {span: (0, 0), line: 0}, b"") /*TODO: Add error*/
            }

            if c == b'"' { self.next(); break; }
            c = *self.next().get();
        }

        let result = &self.raw[r_span.as_range()];
        Token::new(TokenKind::LiteralStr, TokenMetadata {span: r_span.as_raw(), line: self.line}, result)
    }

    fn parse_punctuation(&mut self) -> Token<'a> {
        let r_span = self.record_span();
        let kind = match (*self.get(), *self.peek()) {
            (b',', _) => {self.next(); TokenKind::Comma },
            (b'(', _) =>  {self.next(); TokenKind::OpenParenthesis },
            (b')', _) =>  {self.next(); TokenKind::CloseParenthesis },
            (b'{', _) =>  {self.next(); TokenKind::OpenBrace },
            (b'}', _) =>  {self.next(); TokenKind::CloseBrace },
            (b'.', _) =>  {self.next(); TokenKind::Dot },
            (b'!', b'=') => {
                self.next().next();
                TokenKind::NotEqual
            },
            (b'=', b'=') => {
                self.next().next();
                TokenKind::Equal
            },
            (b'>', b'=') => {
                self.next().next();
                TokenKind::GreaterOrEqual
            },
            (b'<', b'=') => {
                self.next().next();
                TokenKind::LessOrEqual
            },
            (b'|', b'|') => {
                self.next().next();
                TokenKind::Or
            },
            (b'&', b'&') => {
                self.next().next();
                TokenKind::And
            },
            (b'*', _) => {
                self.next().next();
                TokenKind::Multiply
            },
            (b'/', _) => {
                self.next().next();
                TokenKind::Divide
            },
            (b'+', _) => {
                self.next().next();
                TokenKind::Add
            },
            (b'-', _) => {
                self.next().next();
                TokenKind::Minus
            },
            (b'!', _) =>  {self.next(); TokenKind::Not },
            (b'>', _) =>  {self.next(); TokenKind::GreaterThan },
            (b'<', _) =>  {self.next(); TokenKind::LessThan },
            (b'=', _) =>  {self.next(); TokenKind::Assignment },
            _ => { self.next(); TokenKind::NONE /*TODO: Add error*/  }
        };
        let result = &self.raw[r_span.as_range()];
        Token::new(kind, TokenMetadata {span: r_span.as_raw(), line: self.line}, result)
    }

    fn record_span(&self) -> RSpan {
        RSpan::new(self.position.get(), Rc::clone(&self.position))
    }

    fn raw_end(&self) -> bool {
        self.position.get() + 1 == self.raw.len()
    }
    fn end(&mut self) -> bool {
        let end = self.is_end;
        self.is_end = self.position.get() + 1 == self.raw.len();
        end
    }
    fn next(&self) -> & Self {
        if self.position.get() + 1 < self.raw.len() {
            self.position.set(self.position.get() + 1);
        }
        self
    }
    fn get(&self) -> &u8 {
        &self.raw[self.position.get()]
    }
    fn peek(&self) -> &u8 {
        if  self.position.get() + 1 < self.raw.len() {
            &self.raw[self.position.get() + 1 ]
        } else {
            &self.raw[self.position.get()]
        }
    }
}