use crate::Token;

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    raw: &'a[u8],

    parse_error: Vec<&'a str>,
    position: usize
}

impl<'a> Lexer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {tokens: Vec::new(), raw: bytes, parse_error: Vec::new(), position: 0}
    }

    pub fn parse(mut self) -> Result<Vec<Token>, Vec<&'a str>> {
        while !self.end() {
            if self.get().is_ascii_whitespace() {
                self.next();
                continue;
            }
            let result = match self.get() {
                t if t.is_ascii_alphabetic() => self.parse_identifier_or_keyword(),
                t if t.is_ascii_digit() => self.parse_digit(),
                t if *t == b'"' => self.parse_str(),
                t if t.is_ascii_punctuation() =>  self.parse_punctuation(),
                _ => Token::NONE /*TODO: Add error*/
            };
            self.tokens.push(result);
        }
        if self.parse_error.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.parse_error)
        }
    }

    fn cast_keyword(&mut self, string: &str) -> Option<Token> {
        match string {
            "if" => Some(Token::If),
            _ => None
        }
    }
    fn parse_identifier_or_keyword(&mut self) -> Token {
        let mut buffer = Vec::new();
        loop {
            if !self.get().is_ascii_whitespace() && (self.get().is_ascii_alphanumeric() || *self.get() == b'_') {
                let c = *self.get();
                buffer.push(c);
            } else {
                return self.cast_keyword(unsafe { str::from_utf8_unchecked(&*buffer) }).unwrap_or(Token::Identifier(unsafe { String::from_utf8_unchecked(buffer) }))
            }
            if self.end() {
                return self.cast_keyword(unsafe { str::from_utf8_unchecked(&*buffer) }).unwrap_or(Token::Identifier(unsafe { String::from_utf8_unchecked(buffer) }))
            }
            self.next();
        }
    }
    fn parse_digit(&mut self) -> Token {
        let mut buffer = Vec::new();
        loop {
            if !self.get().is_ascii_whitespace() && self.get().is_ascii_digit() {
                let c = *self.get();
                buffer.push(c);
            } else {
                return Token::LiteralDigit(unsafe { String::from_utf8_unchecked(buffer) })
            }
            if self.end() {
                return Token::LiteralDigit(unsafe { String::from_utf8_unchecked(buffer) })
            }
            self.next();
        }
    }

    fn parse_str(&mut self) -> Token {
        let mut buffer = Vec::new();
        let mut c = *self.next().get();
        loop {
            if c == b'"' { self.next(); break; }
            buffer.push(c);
            c = *self.next().get();
            if self.end() {
                return Token::NONE /*TODO: Add error*/
            }
        }
        Token::LiteralStr(unsafe { String::from_utf8_unchecked(buffer) } /*TODO: This may cause safety problem*/)
    }

    fn parse_punctuation(&mut self) -> Token {
        match (*self.get(), *self.peek()) {
            (b',', _) => {self.next(); Token::Comma },
            (b'(', _) =>  {self.next(); Token::OpenParenthesis },
            (b')', _) =>  {self.next(); Token::CloseParenthesis },
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

    fn end(&self) -> bool {
        self.position + 1 == self.raw.len()
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