use crate::lexer::Lexer;

mod lexer;
pub mod i;
#[derive(Debug, Clone, PartialEq)] #[allow(unused)]
enum Token{
    If,
    Let,
    Not,
    Identifier(String),
    Assignment,
    LiteralStr(String),
    LiteralDigit(String),
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
impl Token {

    pub fn get_single_bp(&self) -> Option<u8> {
        match self {
            Token::Not => Some(40),
            Token::OpenParenthesis => Some(80),
            _ => None,
        }
    }
    pub fn get_bp(&self) -> Option<(u8, u8)> {
        match self {
            Token::Assignment => Some((2, 1)),
            Token::Add => Some((20, 20)),
            Token::Minus => Some((20, 20)),
            Token::Multiply => Some((30, 30)),
            Token::Divide => Some((30, 30)),

            Token::Equal => Some((8, 8)),
            Token::NotEqual => Some((8, 8)),

            Token::GreaterOrEqual => Some((10, 10)),
            Token::LessOrEqual => Some((10, 10)),
            Token::GreaterThan => Some((10, 10)),
            Token::LessThan => Some((10, 10)),

            Token::And => Some((5, 5)),
            Token::Or => Some((4, 4)),
            Token::Comma => Some((0, 0)),
            Token::Dot => Some((60,60)),
            Token::OpenParenthesis => Some((80, 80)),
            Token::CloseParenthesis => Some((0, 0)),
            Token::OpenBrace => Some((0, 0)),
            Token::CloseBrace => Some((0, 0)),
            _ => None,
        }
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            Token::If | Token::Let => true,
            _ => false
        }
    }

    // Debug ONLY
    pub fn get_name(&self) -> &str {
        match self {
            Token::Identifier(name) => "Identifier",
            Token::Assignment => "Assignment",
            Token::LiteralStr(name) => "LiteralStr",
            Token::LiteralDigit(name) => "LiteralInt",
            Token::Add => "Add",
            Token::Minus => "Minus",
            Token::Multiply => "Multiply",
            Token::Divide => "Divide",
            Token::Equal => "Equal",
            Token::NotEqual => "NotEqual",
            Token::And => "And",
            Token::Or => "Or",
            Token::Dot => "Dot",
            Token::OpenParenthesis => "OpenParenthesis",
            Token::CloseParenthesis => "CloseParenthesis",
            Token::NONE => "None",
            Token::If => "If",
            Token::Not => "Not",
            Token::Comma => "Comma",
            Token::GreaterOrEqual => "GreaterOrEqual",
            Token::LessOrEqual => "LessOrEqual",
            Token::GreaterThan => "GreaterThan",
            Token::LessThan => "LessThan",
            Token::OpenBrace => "OpenBrace",
            Token::CloseBrace => "CloseBrace",
            Token::Let => "Let",
        }
    }
}

fn main() {
    let mut lexer = Lexer::new(b"if a + (3 3 4 5)").parse().unwrap();
    println!("{:#?}", lexer);
    let ast = Parser::new(lexer).to_ast();
    println!("{:#?}", ast)
}

#[derive(Debug)] #[allow(unused)]
enum Expression {
    Identifier(String),
    LiteralStr(String),
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
enum Statement {
    If {
        condition: Expression,
        body: Vec<Statement>
    },
    ImmutableVariableDeclaration {
        identifier: String,
        value: Expression
    },
    Any {t: Expression}
}

#[derive(Debug)]
enum ErrorKind {
    EOF,
    Unclosed,
    EOFandUnclosed,
    MissingToken(Token),
    CannotParse(Token)
}

struct ParseError {
    kind: ErrorKind,
    message: String
}

impl ParseError {
    fn new(kind: ErrorKind, message: Option<String>) -> Self {
        Self {kind, message: message.unwrap_or("".to_string())}
    }
    fn unwarp(self) -> (ErrorKind, String) {
        (self.kind, self.message)
    }

    fn summary(self) -> String {
        match self.kind {
            ErrorKind::EOF => format!("End of File: '{}'", self.message),
            ErrorKind::MissingToken(t) => format!("Missing token of '{:?}': '{}'", t, self.message),
            ErrorKind::CannotParse(t) => format!("Cannot Parse token of '{:?}': '{}'", t, self.message),
            _ => "".to_string()
        }
    }
}
struct Parser {
    tokens : Vec<Token>,

    parse_result: Vec<Statement>,
    parse_error: Vec<String>,
    position : usize,
}

impl Parser {
    pub fn new(tokens : Vec<Token>) -> Self {
        Self { tokens, parse_error: Vec::new(), parse_result: Vec::new(), position: 0}
    }

    /// turn the containing Token to ast
    pub fn to_ast(mut self) -> Result<Vec<Statement>, Vec<String>> {
        let result = self.parse();
        if result.is_some() {
            Ok(self.parse_result)
        } else {
            Err(self.parse_error)
        }
    }


    /// The main Parser
    fn parse(&mut self) -> Option<()> {
        while !self.end() {
            if let Some(statement) = self.parse_statement() {
                self.parse_result.push(statement);
            } else {
                return None;
            }
            self.next();
        }
        Some(())
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.consume() {
            Token::If => {
                let condition = self.parse_expression("IF_START", 1);

                if let Ok(condition) = condition {
                    let Some(body) = self.parse_body() else {
                        self.parse_error.push("error when parsing body for if statement".to_string());
                        return None
                    };
                    return Some(Statement::If {condition, body})
                }
                println!("{}",condition.unwrap_err().summary());
                None
            }
            Token::Let => {
                let Ok(Expression::VariableAssignment {left, right}) = self.parse_expression("LET_START", 1) else {
                    return  None
                };
                if let Expression::Identifier(name) = *left {
                    return Some(Statement::ImmutableVariableDeclaration {identifier: name, value: *right})
                }
                None
            }

            _ => None
        }
    }

    fn parse_body(&mut self) -> Option<Vec<Statement>> {
        if self.peek() == &Token::OpenBrace {
            let mut statements : Vec<Statement> = Vec::new();
            self.next().consume();
            loop {
                if self.peek() == &Token::CloseBrace {
                    self.next().consume();
                    break
                }
                if self.peek().is_keyword() {
                    self.next();
                    let Some(result) = self.parse_statement() else {
                        return None
                    };
                    statements.push(result);
                } else {
                    let Ok(result) = self.parse_expression("parse_body", 1) else {
                        return None
                    };
                    statements.push(Statement::Any {t: result});
                }

                if self.end() {
                    self.parse_error.push("Unexpected EOF while parsing body".to_string());
                    return None
                }
            }
            return Some(statements)
        }
        self.parse_error.push("Expected OpenBrace and CloseBrace. Got EOF instead".to_string());
        None
    }

    fn parse_unclosed_until(&mut self, bp: u8, closed: Token, separator: Token, _debug: Option<&str>) -> Result<Vec<Expression>, ParseError> {
        let mut list = Vec::<Expression>::new();
        loop {
            if self.end() {
                return Err(ParseError::new(ErrorKind::EOFandUnclosed, None));
            }
            if *self.peek() == closed {
                self.next().consume();
                break
            }
            if list.len() > 0 && *self.peek() == separator {
                self.next().consume();
                if *self.peek() == closed {
                    self.next().consume();
                    break
                }
            } else if list.len() > 0 && *self.peek() != separator {
                return Err(ParseError::new(ErrorKind::MissingToken(separator), None));
            }
            let result = self.parse_expression(_debug.unwrap_or("parse_unclosed"), bp);
            if let Ok(result) = result {
                list.push(result);
            } else {
                return Err(result.unwrap_err())
            }
        }
        Ok(list)
    }

    fn parse_parenthesis(&mut self, _debug: Option<&str>) -> Result<Expression, ParseError> {
        let result = self.parse_unclosed_until(2, Token::CloseParenthesis, Token::Comma, _debug);
        if let Ok(mut result) = result {
            if result.len() == 1 {
                return Ok(Expression::Parenthesis(Box::new(result.pop().unwrap())));
            }
            return Ok(Expression::Tuple(result))
        }
        match result.unwrap_err().unwarp() {
            (ErrorKind::EOFandUnclosed, _) => Err(ParseError::new(ErrorKind::MissingToken(Token::CloseParenthesis), Some("Unclosed '(' with Unexpectedly EOF.".to_string()))),
            (T, A) => Err(ParseError::new(T, Some(A))),
        }
    }
    fn parse_expression(&mut self, _debug: &str, bp: u8) -> Result<Expression, ParseError> {
        let mut rawleft= self.parse_nud();
        loop {
            if let Ok(left) = rawleft {
                if self.end() {
                    return Ok(left);
                }
                if let Some((lbp, rbp)) = self.peek().get_bp() {
                    if lbp < bp {
                        return  Ok(left);
                    }
                    rawleft = self.parse_led(left, rbp);
                } else { return  Ok(left); }
            } else {
                return Err(rawleft.unwrap_err())
            }
        }
    }
    fn parse_nud(&mut self) -> Result<Expression, ParseError> {
        match self.next().consume() {
            Token::LiteralStr(s) => Ok(Expression::LiteralStr(s)),
            Token::Identifier(s) => Ok(Expression::Identifier(s)),
            Token::Not => {
                let result = self.parse_expression(Token::Not.get_name(),Token::Not.get_single_bp().unwrap());
                if let Ok(result) = result {
                    return Ok(Expression::Not(Box::new(result)))
                }
                Err(ParseError::new(ErrorKind::CannotParse(Token::Not), Some(result.unwrap_err().summary())))
            },
            Token::OpenParenthesis => {
                let result =self.parse_parenthesis(Some("parse_nud.parse_parenthesis_or_tuple.OpenParenthesis"));
                if let Ok(result) = result {
                    return Ok(result)
                }
                Err(ParseError::new(ErrorKind::CannotParse(Token::OpenParenthesis), Some(result.unwrap_err().summary())))
            },
            Token::LiteralDigit(s) => {
                let Ok(s) = s.parse::<i64>() else {
                    return Err(ParseError::new(ErrorKind::CannotParse(Token::LiteralDigit(s)), Some("Cannot parse to number".to_string())));
                };
                Ok(Expression::LiteralInt(s))
            },
            t => Err(ParseError::new(ErrorKind::CannotParse(t), Some("cannot parse token to expression".to_string()))),
        }
    }

    fn parse_led(&mut self, left: Expression, rbp: u8) ->  Result<Expression, ParseError> {
         match self.next().consume() {
             Token::Assignment => {
                 let result=self.parse_expression(Token::Assignment.get_name(),rbp);
                 if let Ok(result) = result {
                    return Ok(Expression::VariableAssignment {
                            left: Box::new(left),
                            right: Box::new(result)
                    })
                 }
                 Err(result.unwrap_err())
             }
             Token::Add => {
                 let result=self.parse_expression(Token::Add.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::Add {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             },
             Token::Minus => {
                 let result=self.parse_expression(Token::Minus.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::Minus {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::Multiply => {
                 let result=self.parse_expression(Token::Multiply.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::Multiply {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::Divide => {
                 let result=self.parse_expression(Token::Divide.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::Divide {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             },
             Token::Equal => {
                 let result=self.parse_expression(Token::Equal.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::Equal {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::NotEqual => {
                 let result=self.parse_expression(Token::NotEqual.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::NotEqual {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::GreaterOrEqual => {
                 let result=self.parse_expression(Token::GreaterOrEqual.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::GreaterOrEqual {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::LessOrEqual => {
                 let result=self.parse_expression(Token::LessOrEqual.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::LessOrEqual {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::GreaterThan => {
                 let result=self.parse_expression(Token::GreaterThan.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::GreaterThan {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::LessThan => {
                 let result=self.parse_expression(Token::LessThan.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::LessThan {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::And => {
                 let result=self.parse_expression(Token::And.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::And {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::Or => {
                 let result=self.parse_expression(Token::Or.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::Or {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::Dot => {
                 let result=self.parse_expression(Token::Dot.get_name(),rbp);
                 if let Ok(result) = result {
                     return Ok(Expression::AccessMember {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             Token::OpenParenthesis => {
                 let result=self.parse_parenthesis(Some("membercall"));
                 if let Ok(result) = result {
                     return Ok(Expression::AccessMember {
                         left: Box::new(left),
                         right: Box::new(result)
                     })
                 }
                 Err(result.unwrap_err())
             }
             t => {Err(ParseError::new(ErrorKind::CannotParse(t), Some("Token isn't Expression that need right detonation".to_string())))}
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

    /// consume the current item, replacing [Token::NONE] in its place
    fn consume(&mut self) -> Token {
        std::mem::replace(&mut self.tokens[self.position], Token::NONE)
    }

    /// peek next.
    fn peek(&self) -> &Token {
        if self.position + 1 < self.tokens.len() {
            &self.tokens[self.position + 1]
        } else {
            &self.tokens[self.position]
        }
    }
}
