use std::fmt::Pointer;
use colored::Colorize;

#[derive(Debug, Clone)] #[allow(unused)]
enum Token {
    If,
    Not,
    Identifier(String),
    Assignment,
    LiteralStr(String),
    LiteralInt(String),
    Add,
    Minus,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    And,
    Or,
    Dot,
    OpenBrace,
    CloseBrace,

    NONE
}

#[allow(unused)]
impl Token {

    pub fn get_single_bp(&self) -> Option<u8> {
        match self {
            Token::Not => Some(40),
            _ => None,
        }
    }
    pub fn get_bp(&self) -> Option<(u8, u8)> {
        match self {
            Token::Assignment => Some((2, 1)),
            Token::Add => Some((10, 10)),
            Token::Minus => Some((10, 10)),
            Token::Multiply => Some((20, 20)),
            Token::Divide => Some((20, 20)),

            Token::Equal => Some((8, 8)),
            Token::NotEqual => Some((8, 8)),

            Token::And => Some((5, 5)),
            Token::Or => Some((4, 4)),
            Token::Dot => Some((60,60)),
            _ => None,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Token::Identifier(name) => "identifier",
            Token::Assignment => "assignment",
            Token::LiteralStr(name) => "string literal",
            Token::LiteralInt(name) => "integer literal",
            Token::Add => "Add",
            Token::Minus => "Minus",
            Token::Multiply => "Multiply",
            Token::Divide => "Divide",
            Token::Equal => "Equal",
            Token::NotEqual => "NotEqual",
            Token::And => "And",
            Token::Or => "Or",
            Token::Dot => "Dot",
            Token::OpenBrace => "OpenBrace",
            Token::CloseBrace => "CloseBrace",
            Token::NONE => "None",
            Token::If => "If",
            Token::Not => "Not",


        }
    }
}

fn main() {
    let mut tokens : Vec<Token> = Vec::new();

    if 001 != 002 && 003 * 004 == 005 || true {

    }
    tokens.push(Token::If);
    // tokens.push(Token::Identifier("ab".to_string()));
    // tokens.push(Token::Assignment);
    // tokens.push(Token::LiteralInt("12".to_string()));
    // tokens.push(Token::Add);
    // tokens.push(Token::LiteralInt("34".to_string()));
    // tokens.push(Token::Add);
    // tokens.push(Token::LiteralInt("341".to_string()));
    // tokens.push(Token::Minus);
    tokens.push(Token::LiteralInt("001".to_string()));
    tokens.push(Token::NotEqual);
    tokens.push(Token::LiteralInt("002".to_string()));
    tokens.push(Token::And);
    tokens.push(Token::LiteralInt("003".to_string()));
    tokens.push(Token::Multiply);
    tokens.push(Token::LiteralInt("004".to_string()));
    tokens.push(Token::Equal);
    tokens.push(Token::LiteralInt("005".to_string()));
    tokens.push(Token::Or);
    tokens.push(Token::LiteralInt("101".to_string()));
    tokens.push(Token::Multiply);
    tokens.push(Token::LiteralInt("102".to_string()));
    tokens.push(Token::Equal);
    tokens.push(Token::LiteralInt("103".to_string()));
    tokens.push(Token::Or);
    tokens.push(Token::Identifier("Object".to_string()));
    tokens.push(Token::Dot);
    tokens.push(Token::Identifier("String".to_string()));
    tokens.push(Token::Dot);
    tokens.push(Token::Identifier("Len".to_string()));
    let parser = Parser::new(tokens);
    parser.to_ast();
}

#[derive(Debug)] #[allow(unused)]
enum Expression {
    None,
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
    AccessMember {
        left: Box<Expression>,
        right: Box<Expression>
    },
    And {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Or {
        left: Box<Expression>,
        right: Box<Expression>
    }

}

struct Parser {
    tokens : Vec<Token>,
    position : usize,
}

impl Parser {
    pub fn new(tokens : Vec<Token>) -> Self {
        Self { tokens, position: 0}
    }

    /// turn the containing Token to ast
    pub fn to_ast(mut self) {
        self.parse();
    }


    /// The main Parser
    fn parse(&mut self) {
        println!("{:#?}", self.parse_expression("START",0));
    }

    fn parse_expression(&mut self, _d: &str, bp: u8) -> Expression {
        let mut left = self.parse_nud();
        print!("{:#?}", left);
        loop {
            if self.end() {
                break;
            }
            if let Some((lbp, rbp)) = self.peek().get_bp() {
                if lbp < bp {
                    break;
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
            Token::Not => Expression::Not(Box::new(self.parse_expression(Token::Not.get_name(),Token::Not.get_single_bp().unwrap()))),
            Token::LiteralInt(s) => {
                let Ok(s) = s.parse::<i64>() else {
                    panic!("Could not parse number");
                };
                Expression::LiteralInt(s)
            },
            _ => Expression::None,
        }
    }

    fn parse_led(&mut self, left: Expression, rbp: u8) -> Expression {
         match self.next().consume() {
            Token::Assignment => {
                Expression::VariableAssignment {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(Token::Assignment.get_name(),rbp))
                }
            }
            Token::Add => {
                Expression::Add {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(Token::Add.get_name(),rbp))
                }
            },
            Token::Minus => {
                Expression::Minus {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(Token::Minus.get_name(),rbp))
                }
            }
            Token::Multiply => {
                Expression::Multiply {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(Token::Multiply.get_name(),rbp))
                }
            }
            Token::Divide => {
                Expression::Divide {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(Token::Divide.get_name(),rbp))
                }
            },
             Token::Equal => {
                Expression::Equal {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(Token::Equal.get_name(),rbp))
                }
             }
             Token::NotEqual => {
                 Expression::NotEqual {
                     left: Box::new(left),
                     right: Box::new(self.parse_expression(Token::NotEqual.get_name(),rbp))
                 }
             }
             Token::And => {
                 Expression::And {
                     left: Box::new(left),
                     right: Box::new(self.parse_expression(Token::And.get_name(),rbp))
                 }
             }
             Token::Or => {
                 Expression::Or {
                     left: Box::new(left),
                     right: Box::new(self.parse_expression(Token::Or.get_name(),rbp))
                 }
             }
             Token::Dot => {
                 Expression::AccessMember {
                     left: Box::new(left),
                     right: Box::new(self.parse_expression(Token::Dot.get_name(),rbp))
                 }
             }
            _ => {
                Expression::None
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
