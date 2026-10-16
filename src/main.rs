mod lexer;

#[derive(Debug, Clone, PartialEq)] #[allow(unused)]
enum Token{
    If,
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
            _ => None,
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
        }
    }
}

fn main() {
    let mut tokens : Vec<Token> = Vec::new();

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

    fn parse_expression(&mut self, _debug: &str, bp: u8) -> Expression {
        let mut left = self.parse_nud();
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
    fn parse_parenthesis_or_tuple(&mut self, _debug: Option<&str>) -> Expression {
        let mut list = Vec::<Expression>::new();
        loop {
            if self.end() {
                panic!("Expected '{}'", Token::CloseParenthesis.get_name());
            }
            if Token::CloseParenthesis == *self.peek() {
                self.next().consume();
                break
            }
            if list.len() > 0 && Token::Comma == *self.peek() {
                self.next().consume();
                if Token::CloseParenthesis == *self.peek() {
                    self.next().consume();
                    break
                }
            } else if list.len() > 0 && Token::Comma != *self.peek() {
                panic!("Expected Comma got '{}' Instead", self.peek().get_name());
            }
            list.push(self.parse_expression(_debug.unwrap_or("parse_parenthesis_or_tuple"), 1));
        }
        if list.len() == 1 {
            return Expression::Parenthesis(Box::new(list.pop().unwrap()));
        }
        Expression::Tuple(list)
    }
    fn parse_nud(&mut self) -> Expression {
        match self.next().consume() {
            Token::LiteralStr(s) => Expression::LiteralStr(s),
            Token::Identifier(s) => Expression::Identifier(s),
            Token::Not => Expression::Not(Box::new(self.parse_expression(Token::Not.get_name(),Token::Not.get_single_bp().unwrap()))),
            Token::OpenParenthesis => self.parse_parenthesis_or_tuple(Some("parse_nud.parse_parenthesis_or_tuple.OpenParenthesis")),
            Token::LiteralDigit(s) => {
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
             Token::GreaterOrEqual => {
                 Expression::GreaterOrEqual {
                     left: Box::new(left),
                     right: Box::new(self.parse_expression(Token::GreaterOrEqual.get_name(),rbp))
                 }
             }
             Token::LessOrEqual => {
                 Expression::LessOrEqual {
                     left: Box::new(left),
                     right: Box::new(self.parse_expression(Token::LessOrEqual.get_name(),rbp))
                 }
             }
             Token::GreaterThan => {
                 Expression::GreaterThan {
                     left: Box::new(left),
                     right: Box::new(self.parse_expression(Token::GreaterThan.get_name(),rbp))
                 }
             }
             Token::LessThan => {
                 Expression::LessThan {
                     left: Box::new(left),
                     right: Box::new(self.parse_expression(Token::LessThan.get_name(),rbp))
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
             Token::OpenParenthesis => {
                 Expression::MemberCall {
                     left: Box::new(left),
                     right: Box::new(self.parse_parenthesis_or_tuple(Some("parse_led.parse_parenthesis_or_tuple.MemberCall"))),
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
