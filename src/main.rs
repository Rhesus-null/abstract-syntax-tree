use crate::lexer::{Lexer, Token, TokenMetadata};
use crate::static_analyzer::StaticAnalyzer;

mod lexer;
pub mod static_analyzer;
#[derive(Debug, Copy, Clone,PartialEq)] #[allow(unused)]
pub enum TokenKind {
    If,
    Let,
    Not,
    Identifier,
    Assignment,
    LiteralStr,
    LiteralDigit,
    Add,
    Minus,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    And,
    Or,
    Dot,
    OpenParenthesis,
    CloseParenthesis,

    OpenBrace,
    CloseBrace,
    Comma,

    GreaterOrEqual,
    LessOrEqual,
    GreaterThan,
    LessThan,

    NONE
}

#[allow(unused)]
impl TokenKind {

    pub fn get_single_bp(&self) -> Option<u8> {
        match self {
            TokenKind::Not => Some(40),
            TokenKind::OpenParenthesis => Some(80),
            _ => None,
        }
    }
    pub fn get_bp(&self) -> Option<(u8, u8)> {
        match self {
            TokenKind::Assignment => Some((2, 1)),
            TokenKind::Add => Some((20, 20)),
            TokenKind::Minus => Some((20, 20)),
            TokenKind::Multiply => Some((30, 30)),
            TokenKind::Divide => Some((30, 30)),

            TokenKind::Equal => Some((8, 8)),
            TokenKind::NotEqual => Some((8, 8)),

            TokenKind::GreaterOrEqual => Some((10, 10)),
            TokenKind::LessOrEqual => Some((10, 10)),
            TokenKind::GreaterThan => Some((10, 10)),
            TokenKind::LessThan => Some((10, 10)),

            TokenKind::And => Some((5, 5)),
            TokenKind::Or => Some((4, 4)),
            TokenKind::Comma => Some((0, 0)),
            TokenKind::Dot => Some((60, 60)),
            TokenKind::OpenParenthesis => Some((80, 80)),
            TokenKind::CloseParenthesis => Some((0, 0)),
            TokenKind::OpenBrace => Some((0, 0)),
            TokenKind::CloseBrace => Some((0, 0)),
            _ => None,
        }
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            TokenKind::If | TokenKind::Let => true,
            _ => false
        }
    }

    // Debug ONLY
    pub fn get_name(&self) -> &str {
        match self {
            TokenKind::Identifier => "Identifier",
            TokenKind::Assignment => "Assignment",
            TokenKind::LiteralStr=> "LiteralStr",
            TokenKind::LiteralDigit => "LiteralInt",
            TokenKind::Add => "Add",
            TokenKind::Minus => "Minus",
            TokenKind::Multiply => "Multiply",
            TokenKind::Divide => "Divide",
            TokenKind::Equal => "Equal",
            TokenKind::NotEqual => "NotEqual",
            TokenKind::And => "And",
            TokenKind::Or => "Or",
            TokenKind::Dot => "Dot",
            TokenKind::OpenParenthesis => "OpenParenthesis",
            TokenKind::CloseParenthesis => "CloseParenthesis",
            TokenKind::NONE => "None",
            TokenKind::If => "If",
            TokenKind::Not => "Not",
            TokenKind::Comma => "Comma",
            TokenKind::GreaterOrEqual => "GreaterOrEqual",
            TokenKind::LessOrEqual => "LessOrEqual",
            TokenKind::GreaterThan => "GreaterThan",
            TokenKind::LessThan => "LessThan",
            TokenKind::OpenBrace => "OpenBrace",
            TokenKind::CloseBrace => "CloseBrace",
            TokenKind::Let => "Let",
        }
    }
}

fn main() {
    let mut lexer = Lexer::new(b"if !mynameisjeff {}").parse().unwrap();
    println!("v: {:#?}", lexer);
    let ast = Parser::new(lexer).parse();
    println!("r: {:#?}", ast);
    let analyzer_result = StaticAnalyzer::new(ast);
    println!("s: {:#?}", analyzer_result);
}

#[derive(Debug)]
pub enum ErrorKind {
    EOF,
    MissingToken(TokenKind),
    CannotParse,
}

#[derive(Debug)] #[allow(unused)]
pub enum Expression {
    ERROR {
        error: ErrorKind,
        origin: TokenKind
    },
    Identifier(Box<[u8]>),
    LiteralStr(Box<str>),
    LiteralInt(i64),
    Not(Box<Expression>),
    VariableAssignment {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Add {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Minus {
        left: Box<Expression>,
        right: Box<Expression>
    },

    Multiply {
        left: Box<Expression>,
        right: Box<Expression>
    },

    Divide {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Equal {
        left: Box<Expression>,
        right: Box<Expression>
    },
    NotEqual {
        left: Box<Expression>,
        right: Box<Expression>
    },
    GreaterOrEqual {
        left: Box<Expression>,
        right: Box<Expression>
    },
    LessOrEqual {
        left: Box<Expression>,
        right: Box<Expression>
    },
    GreaterThan {
        left: Box<Expression>,
        right: Box<Expression>
    },
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>
    },
    AccessMember {
        left: Box<Expression>,
        right: Box<Expression>
    },
    MemberCall {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Tuple(Vec<Expression>),
    Parenthesis(Box<Expression>),
    And {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Or {
        left: Box<Expression>,
        right: Box<Expression>
    }
}

#[derive(Debug)]
pub enum StatementErrorKind {
    ExpectedExpression(&'static str),
    ExpectDeclaration,
    OtherErr(ErrorKind),
}

#[derive(Debug)]
pub enum Statement {
    If {
        condition: Expression,
        body: Vec<Statement>
    },
    ImmutableVariableDeclaration {
        identifier: Box<[u8]>,
        value: Expression
    },
    Any {t: Expression},
    ERROR { error: StatementErrorKind, origin: TokenKind, origin_value: Option<Expression>},
}

struct Parser<'a> {
    tokens : Vec<Token<'a>>,

    parse_result: Vec<Statement>,
    position : usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens : Vec<Token<'a>>) -> Self {
        Self { tokens, parse_result: Vec::new(), position: 0}
    }


    /// The main Parser
    fn parse(mut self) -> Vec<Statement> {
        while !self.end() {
            let r = self.parse_statement();
            self.parse_result.push(r);
            self.next();
        }
        self.parse_result
    }

    fn parse_statement(&mut self) -> Statement {
        match self.consume().get_token_kind() {
            TokenKind::If => {
                let condition = self.parse_expression("IF_START", 1);
                let result = self.parse_body();
                if let Ok(body) = result {
                    return Statement::If {condition, body};
                };
                Statement::ERROR { error: StatementErrorKind::OtherErr(result.unwrap_err()), origin: TokenKind::If, origin_value: Some(condition) }
            }
            TokenKind::Let => {
                let Expression::VariableAssignment {left, right}= self.parse_expression("LET_START", 1) else {
                    return Statement::ERROR { error: StatementErrorKind::ExpectedExpression("VariableAssignment"), origin: TokenKind::Let, origin_value: None};
                };
                if let Expression::Identifier(name) = *left {
                    return Statement::ImmutableVariableDeclaration {identifier: name, value: *right}
                }
                Statement::ERROR { error: StatementErrorKind::ExpectedExpression("Identifier"), origin: TokenKind::Let, origin_value: None}
            }

            t => Statement::ERROR { error: StatementErrorKind::ExpectDeclaration, origin: t, origin_value: None},
        }
    }

    fn parse_body(&mut self) -> Result<Vec<Statement>, ErrorKind> {
        if self.peek() == TokenKind::OpenBrace {
            let mut statements : Vec<Statement> = Vec::new();
            self.next().consume();
            loop {
                if self.peek() == TokenKind::CloseBrace {
                    self.next().consume();
                    break
                }
                if self.peek().is_keyword() {
                    self.next();
                    statements.push(self.parse_statement());
                } else {
                    statements.push(Statement::Any {t:
                    self.parse_expression("parse_body", 1)});
                }

                if self.end() {
                    return Err(ErrorKind::MissingToken(TokenKind::CloseBrace));
                }
            }
            return Ok(statements)
        }
        Err(ErrorKind::MissingToken(TokenKind::CloseBrace))
    }

    fn parse_unclosed_until(&mut self, bp: u8, closed: TokenKind, separator: TokenKind, _debug: Option<&str>) -> Result<Vec<Expression>, ErrorKind> {
        let mut list = Vec::<Expression>::new();
        loop {
            if self.end() {
                return Err(ErrorKind::MissingToken(closed));
            }
            if self.peek() == closed {
                self.next().consume();
                break
            }
            if list.len() > 0 && self.peek() == separator {
                self.next().consume();
                if self.peek() == closed {
                    self.next().consume();
                    break
                }
            } else if list.len() > 0 && self.peek() != separator {
                return Err(ErrorKind::MissingToken(separator))
            }
            let result = self.parse_expression(_debug.unwrap_or("parse_unclosed"), bp);
            list.push(result);
        }
        Ok(list)

    }

    fn parse_parenthesis(&mut self, _debug: Option<&str>) -> Expression {
        let result = self.parse_unclosed_until(2, TokenKind::CloseParenthesis, TokenKind::Comma, _debug);
        if let Ok(mut result) = result {
            if result.len() == 1 {
                return Expression::Parenthesis(Box::new(result.pop().unwrap()));
            }
            return Expression::Tuple(result)
        }
        Expression::ERROR { error: result.unwrap_err(), origin: TokenKind::OpenParenthesis}
    }
    fn parse_expression(&mut self, _debug: &str, bp: u8) -> Expression {
        let mut left= self.parse_nud();
        loop {
            if self.end() {
                break
            }
            if let Some((lbp, rbp)) = self.peek().get_bp() {
                if lbp < bp {
                    break
                }
                left = self.parse_led(left, rbp);
            } else { break }
        }
        left
    }
    fn parse_nud(&mut self) -> Expression {
        let t = self.next().consume();
        match (t.get_token_kind(), t.get_value()) {
            (TokenKind::LiteralStr, v) => {
                let Ok(str) = str::from_utf8(v) else {
                    return Expression::ERROR { error: ErrorKind::CannotParse, origin: TokenKind::LiteralStr }
                };
                Expression::LiteralStr(Box::from(str))
            }
            (TokenKind::Identifier, v) => {
                Expression::Identifier(Box::from(v))
            }
            (TokenKind::Not, _) => {
                 Expression::Not(Box::new(self.parse_expression(TokenKind::Not.get_name(), TokenKind::Not.get_single_bp().unwrap())))
            },
            (TokenKind::OpenParenthesis, _) => {
                self.parse_parenthesis(Some("parse_nud.parse_parenthesis_or_tuple.OpenParenthesis"))
            },
            (TokenKind::LiteralDigit, v) => {
                let Ok(str) = str::from_utf8(v) else {
                    return Expression::ERROR { error: ErrorKind::CannotParse, origin: TokenKind::LiteralDigit }
                };
                let Ok(s) = str.parse::<i64>() else {
                    return Expression::ERROR { error: ErrorKind::CannotParse, origin: TokenKind::LiteralDigit }
                };
                Expression::LiteralInt(s)
            },
            _ => {
                Expression::ERROR { error: ErrorKind::CannotParse, origin: TokenKind::NONE}
            },
        }
    }

    fn parse_led(&mut self, left: Expression, rbp: u8) ->  Expression {
         match self.next().consume().get_token_kind() {
             TokenKind::Assignment => {
                 let result=self.parse_expression(TokenKind::Assignment.get_name(), rbp);
                 Expression::VariableAssignment {
                     left: Box::new(left),
                     right: Box::new(result)
                 }
             }
             TokenKind::Add => {
                 let result=self.parse_expression(TokenKind::Add.get_name(), rbp);
                 Expression::Add {
                     left: Box::new(left),
                     right: Box::new(result)
                 }
             },
             TokenKind::Minus => {
                 let result=self.parse_expression(TokenKind::Minus.get_name(), rbp);
                 Expression::Minus {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::Multiply => {
                 let result=self.parse_expression(TokenKind::Multiply.get_name(), rbp);
                 Expression::Multiply {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::Divide => {
                 let result=self.parse_expression(TokenKind::Divide.get_name(), rbp);
               Expression::Divide {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             },
             TokenKind::Equal => {
                 let result=self.parse_expression(TokenKind::Equal.get_name(), rbp);
                Expression::Equal {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::NotEqual => {
                 let result=self.parse_expression(TokenKind::NotEqual.get_name(), rbp);
                Expression::NotEqual {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::GreaterOrEqual => {
                 let result=self.parse_expression(TokenKind::GreaterOrEqual.get_name(), rbp);
              Expression::GreaterOrEqual {
                         left: Box::new(left),
                         right: Box::new(result)
                     }

             }
             TokenKind::LessOrEqual => {
                 let result=self.parse_expression(TokenKind::LessOrEqual.get_name(), rbp);
                Expression::LessOrEqual {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::GreaterThan => {
                 let result=self.parse_expression(TokenKind::GreaterThan.get_name(), rbp);
                Expression::GreaterThan {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::LessThan => {
                 let result=self.parse_expression(TokenKind::LessThan.get_name(), rbp);
                 Expression::LessThan {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::And => {
                 let result=self.parse_expression(TokenKind::And.get_name(), rbp);
                 Expression::And {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::Or => {
                 let result=self.parse_expression(TokenKind::Or.get_name(), rbp);
                 Expression::Or {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::Dot => {
                 let result=self.parse_expression(TokenKind::Dot.get_name(), rbp);
                 Expression::AccessMember {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             TokenKind::OpenParenthesis => {
                 let result=self.parse_parenthesis(Some("membercall"));
                 Expression::AccessMember {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             t => {
                 Expression::ERROR { error: ErrorKind::CannotParse, origin: TokenKind::NONE}
             }
        }
    }

    fn end(&self) -> bool {
        self.position + 1 == self.tokens.len()
    }
    /// increment the position by one, and stop if position reach the end bounds
    fn next(&mut self) -> &mut Self {
        if self.position + 1 < self.tokens.len() {
            self.position += 1;
        }
        self
    }

    fn consume(&mut self) -> Token {
        let t = std::mem::replace(&mut self.tokens[self.position], Token::new(TokenKind::NONE, TokenMetadata {span: (0, 0), line: 0}, b""));
        let _ = std::mem::replace(&mut self.tokens[self.position], t);
        t
    }

    /// peek next.
    fn peek(&self) -> TokenKind {
        if self.position + 1 < self.tokens.len() {
            self.tokens[self.position + 1].get_token_kind()
        } else {
            self.tokens[self.position].get_token_kind()
        }
    }
}
