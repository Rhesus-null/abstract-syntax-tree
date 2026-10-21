use crate::lexer::Lexer;

mod lexer;
pub mod static_analyzer;
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
    let mut lexer = Lexer::new(b"if a + v && t >= f || c + { c + ( }").parse().unwrap();
    println!("{:#?}", lexer);
    let ast = Parser::new(lexer).to_ast();
    println!("{:#?}", ast)
}

#[derive(Debug)]
enum ErrorKind {
    EOF,
    Unclosed,
    EOFandUnclosed,
    MissingToken(Token),
    CannotParse,
}

#[derive(Debug)] #[allow(unused)]
enum Expression {
    ERROR {
        e: ErrorKind,
        origin: Token
    },
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
enum StatementErrorKind {
    ExpectedExpression(&'static str),
    ExpectDeclaration,
    OtherErr(ErrorKind),
    SelfErr {
        e: Box<StatementErrorKind>, origin: Token, m_value: Option<Box<Vec<Statement>>>, s_value: Option<Box<Statement>>
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
    Any {t: Expression},
    ERROR {e: StatementErrorKind, origin: Token, origin_value: Option<Expression>},
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
            let r = self.parse_statement();
            self.parse_result.push(r);
            self.next();
        }
        Some(())
    }

    fn parse_statement(&mut self) -> Statement {
        match self.consume() {
            Token::If => {
                let condition = self.parse_expression("IF_START", 1);
                let result = self.parse_body();
                if let Ok(body) = result {
                    return Statement::If {condition, body};
                };
                Statement::ERROR { e: result.unwrap_err(), origin: Token::If, origin_value: Some(condition) }
            }
            Token::Let => {
                let Expression::VariableAssignment {left, right}= self.parse_expression("LET_START", 1) else {
                    return Statement::ERROR {e: StatementErrorKind::ExpectedExpression("VariableAssignment"), origin: Token::Let, origin_value: None};
                };
                if let Expression::Identifier(name) = *left {
                    return Statement::ImmutableVariableDeclaration {identifier: name, value: *right}
                }
                Statement::ERROR {e: StatementErrorKind::ExpectedExpression("Identifier"), origin: Token::Let, origin_value: None}
            }

            t => Statement::ERROR {e: StatementErrorKind::ExpectDeclaration, origin: t, origin_value: None},
        }
    }

    fn parse_body(&mut self) -> Result<Vec<Statement>, StatementErrorKind> {
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
                    statements.push(self.parse_statement());
                } else {
                    statements.push(Statement::Any {t:
                    self.parse_expression("parse_body", 1)});
                }

                if self.end() {
                    return Err(StatementErrorKind::SelfErr{e: Box::new(StatementErrorKind::OtherErr(ErrorKind::EOFandUnclosed)), origin: Token::OpenBrace, m_value:Some(Box::new(statements)), s_value:None});
                }
            }
            return Ok(statements)
        }
        Err(StatementErrorKind::OtherErr(ErrorKind::MissingToken(Token::CloseBrace)))
    }

    fn parse_unclosed_until(&mut self, bp: u8, closed: Token, separator: Token, _debug: Option<&str>) -> Result<Vec<Expression>, ErrorKind> {
        let mut list = Vec::<Expression>::new();
        loop {
            if self.end() {
                return Err(ErrorKind::EOFandUnclosed)
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
                return Err(ErrorKind::MissingToken(separator))
            }
            let result = self.parse_expression(_debug.unwrap_or("parse_unclosed"), bp);
            list.push(result);
        }
        Ok(list)

    }

    fn parse_parenthesis(&mut self, _debug: Option<&str>) -> Expression {
        let result = self.parse_unclosed_until(2, Token::CloseParenthesis, Token::Comma, _debug);
        if let Ok(mut result) = result {
            if result.len() == 1 {
                return Expression::Parenthesis(Box::new(result.pop().unwrap()));
            }
            return Expression::Tuple(result)
        }
        Expression::ERROR {e: result.unwrap_err(), origin: Token::OpenParenthesis}
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
        match self.next().consume() {
            Token::LiteralStr(s) => Expression::LiteralStr(s),
            Token::Identifier(s) => Expression::Identifier(s),
            Token::Not => {
                 self.parse_expression(Token::Not.get_name(),Token::Not.get_single_bp().unwrap())
            },
            Token::OpenParenthesis => {
                self.parse_parenthesis(Some("parse_nud.parse_parenthesis_or_tuple.OpenParenthesis"))
            },
            Token::LiteralDigit(rs) => {
                let Ok(s) = rs.parse::<i64>() else {
                    return Expression::ERROR {e: ErrorKind::CannotParse, origin: Token::LiteralDigit(rs) }
                };
                Expression::LiteralInt(s)
            },
            t => {
                if self.end() {
                    return Expression::ERROR {e: ErrorKind::EOF, origin: Token::NONE}
                };
                Expression::ERROR {e: ErrorKind::CannotParse, origin: t}
            },
        }
    }

    fn parse_led(&mut self, left: Expression, rbp: u8) ->  Expression {
         match self.next().consume() {
             Token::Assignment => {
                 let result=self.parse_expression(Token::Assignment.get_name(),rbp);
                 Expression::VariableAssignment {
                     left: Box::new(left),
                     right: Box::new(result)
                 }
             }
             Token::Add => {
                 let result=self.parse_expression(Token::Add.get_name(),rbp);
                 Expression::Add {
                     left: Box::new(left),
                     right: Box::new(result)
                 }
             },
             Token::Minus => {
                 let result=self.parse_expression(Token::Minus.get_name(),rbp);
                 Expression::Minus {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::Multiply => {
                 let result=self.parse_expression(Token::Multiply.get_name(),rbp);
                 Expression::Multiply {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::Divide => {
                 let result=self.parse_expression(Token::Divide.get_name(),rbp);
               Expression::Divide {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             },
             Token::Equal => {
                 let result=self.parse_expression(Token::Equal.get_name(),rbp);
                Expression::Equal {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::NotEqual => {
                 let result=self.parse_expression(Token::NotEqual.get_name(),rbp);
                Expression::NotEqual {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::GreaterOrEqual => {
                 let result=self.parse_expression(Token::GreaterOrEqual.get_name(),rbp);
              Expression::GreaterOrEqual {
                         left: Box::new(left),
                         right: Box::new(result)
                     }

             }
             Token::LessOrEqual => {
                 let result=self.parse_expression(Token::LessOrEqual.get_name(),rbp);
                Expression::LessOrEqual {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::GreaterThan => {
                 let result=self.parse_expression(Token::GreaterThan.get_name(),rbp);
                Expression::GreaterThan {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::LessThan => {
                 let result=self.parse_expression(Token::LessThan.get_name(),rbp);
                 Expression::LessThan {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::And => {
                 let result=self.parse_expression(Token::And.get_name(),rbp);
                 Expression::And {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::Or => {
                 let result=self.parse_expression(Token::Or.get_name(),rbp);
                 Expression::Or {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::Dot => {
                 let result=self.parse_expression(Token::Dot.get_name(),rbp);
                 Expression::AccessMember {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             Token::OpenParenthesis => {
                 let result=self.parse_parenthesis(Some("membercall"));
                 Expression::AccessMember {
                         left: Box::new(left),
                         right: Box::new(result)
                     }
             }
             t => {
                 if self.end() {
                     return Expression::ERROR {e: ErrorKind::EOF, origin: Token::NONE}
                 }
                 Expression::ERROR {e: ErrorKind::CannotParse, origin: t}
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
