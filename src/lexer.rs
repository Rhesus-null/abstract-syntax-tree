use std::ops::{Deref, DerefMut};
use crate::Token;


#[derive(Debug)]
pub struct TokenMetadata {
    pub(crate) span: (usize, usize),
    pub(crate) line: usize,
}

#[derive(Debug)]
pub struct MToken {
    token: Token,
    value: Option<String>,
    metadata: TokenMetadata,
}

impl MToken {
    pub fn new(token: Token, metadata: TokenMetadata, value: Option<String>) -> Self {
        Self {token, value, metadata}
    }
    
    pub fn deconstruct(self) -> (Token, TokenMetadata, Option<String>) {
        (self.token, self.metadata, self.value)
    }
    
    pub fn get_metadata(self) -> TokenMetadata {
        self.metadata
    }
    
    pub fn get_value(self) -> Option<String> {
        self.value
    }
}
impl Deref for MToken {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}


pub struct Lexer<'a> {
    tokens: Vec<MToken>,
    raw: &'a[u8],

    parse_error: Vec<&'a str>,
    position: usize,
    is_end: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {tokens: Vec::new(), raw: bytes, parse_error: Vec::new(), position: 0, is_end: false}
    }

    pub fn parse(mut self) -> Result<Vec<MToken>, Vec<&'a str>> {
        while !self.end()  {
            if self.get().is_ascii_whitespace() {
                self.next();
                continue;
            }
            let result = match self.get() {
                t if t.is_ascii_alphabetic() => self.parse_identifier_or_keyword(),
                t if t.is_ascii_digit() => self.parse_digit(),
                t if *t == b'"' => self.parse_str(),
                t if t.is_ascii_punctuation() =>  MToken::new(self.parse_punctuation(), TokenMetadata {span: (0, 0), line: 0}, None),
                _ => MToken::new(Token::NONE, TokenMetadata {span: (0, 0), line: 0}, None) /*TODO: Add error*/
            };
            self.tokens.push(result);
        }

        if self.parse_error.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.parse_error)
        }
    }

    fn cast_keyword(&mut self, string: &[u8]) -> Option<MToken> {
        match string {
            b"if" => Some(MToken::new(Token::If, TokenMetadata {span: (0, 0), line: 0}, None)),
            b"let" => Some(MToken::new(Token::Let, TokenMetadata {span: (0, 0), line: 0}, None)),
            _ => None
        }
    }
    fn parse_identifier_or_keyword(&mut self) -> MToken {
        let mut str_b = Vec::new();
        loop {
            if !self.get().is_ascii_whitespace() && (self.get().is_ascii_alphanumeric() || *self.get() == b'_') {
                str_b.push(*self.get());
            } else { break }

            if self.raw_end() {break}
            self.next();
        }
        self.cast_keyword(&str_b)
            .unwrap_or(MToken::new(Token::Identifier, TokenMetadata {span: (0, 0), line: 0}, Some(unsafe {String::from_utf8_unchecked(str_b)})))
    }
    fn parse_digit(&mut self) -> MToken{
        let mut str_b = Vec::new();
        loop {
            if !self.get().is_ascii_whitespace() && self.get().is_ascii_digit() {
                str_b.push(*self.get());
            } else { break }
            if self.raw_end() { break }
            self.next();
        }
        
        MToken::new(Token::LiteralDigit, TokenMetadata {span: (0, 0), line: 0}, Some(unsafe {String::from_utf8_unchecked(str_b)}))
    }

    fn parse_str(&mut self) -> MToken {

        let mut str_b = Vec::new();
        let mut c = *self.next().get();
        loop {
            if self.end() {
                return MToken::new(Token::NONE, TokenMetadata {span: (0, 0), line: 0}, None) /*TODO: Add error*/
            }

            if c == b'"' { self.next(); break; }
            str_b.push(c);
            c = *self.next().get();
        }

        MToken::new(Token::LiteralStr, TokenMetadata {span: (0, 0), line: 0}, Some(unsafe {String::from_utf8_unchecked(str_b)}))
    }

    fn parse_punctuation(&mut self) -> Token {
        match (*self.get(), *self.peek()) {
            (b',', _) => {self.next(); Token::Comma },
            (b'(', _) =>  {self.next(); Token::OpenParenthesis },
            (b')', _) =>  {self.next(); Token::CloseParenthesis },
            (b'{', _) =>  {self.next(); Token::OpenBrace },
            (b'}', _) =>  {self.next(); Token::CloseBrace },
            (b'.', _) =>  {self.next(); Token::Dot },
            (b'!', b'=') => {
                self.next().next();
                Token::NotEqual
            },
            (b'=', b'=') => {
                self.next().next();
                Token::Equal
            },
            (b'>', b'=') => {
                self.next().next();
                Token::GreaterOrEqual
            },
            (b'<', b'=') => {
                self.next().next();
                Token::LessOrEqual
            },
            (b'|', b'|') => {
                self.next().next();
                Token::Or
            },
            (b'&', b'&') => {
                self.next().next();
                Token::And
            },
            (b'*', _) => {
                self.next().next();
                Token::Multiply
            },
            (b'/', _) => {
                self.next().next();
                Token::Divide
            },
            (b'+', _) => {
                self.next().next();
                Token::Add
            },
            (b'-', _) => {
                self.next().next();
                Token::Minus
            },
            (b'!', _) =>  {self.next(); Token::Not },
            (b'>', _) =>  {self.next(); Token::GreaterThan },
            (b'<', _) =>  {self.next(); Token::LessThan },
            (b'=', _) =>  {self.next(); Token::Assignment },
            _ => { self.next(); Token::NONE /*TODO: Add error*/  }
        }
    }

    fn raw_end(&self) -> bool {
        self.position + 1 == self.raw.len()
    }
    fn end(&mut self) -> bool {
        let end = self.is_end;
        self.is_end = self.position + 1 == self.raw.len();
        end
    }
    fn next(&mut self) -> &mut Self {
        if self.position + 1 < self.raw.len() {
            self.position += 1;
        }
        self
    }
    fn get(&mut self) -> &u8 {
        &self.raw[self.position]
    }
    fn peek(&self) -> &u8 {
        if self.position + 1 < self.raw.len() {
            &self.raw[self.position + 1]
        } else {
            &self.raw[self.position]
        }
    }
}