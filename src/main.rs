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
}

#[allow(unused)]
impl Token {
    pub fn get_single_bp(&self) -> u8 {
        self.get_bp().unwrap_or((0, 0)).0
    }
    pub fn get_bp(&self) -> Option<(u8, u8)> {
        match self {
            Token::Assignment => Some((2, 1)),
            Token::Add => Some((3, 4)),
            Token::Minus => Some((3, 4)),
            Token::Multiply => Some((5, 6)),
            Token::Divide => Some((5, 6)),
            Token::Not => Some((40, 0)),
            _ => None,
        }
    }
}

fn main() {
    let mut tokens : Vec<Token> = Vec::new();

    tokens.push(Token::If);
    tokens.push(Token::Identifier("ab".to_string()));
    tokens.push(Token::Assignment);
    tokens.push(Token::LiteralInt("12".to_string()));
    tokens.push(Token::Add);
    tokens.push(Token::LiteralInt("34".to_string()));
    tokens.push(Token::Add);
    tokens.push(Token::LiteralInt("34".to_string()));
    tokens.push(Token::Minus);
    tokens.push(Token::LiteralInt("11".to_string()));
    tokens.push(Token::Multiply);
    tokens.push(Token::LiteralInt("5".to_string()));
    tokens.push(Token::Divide);
    tokens.push(Token::LiteralInt("5".to_string()));
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
    }

}

struct Parser {
    tokens : Vec<Token>,
    position : usize
}

impl Parser {
    pub fn new(tokens : Vec<Token>) -> Parser {
        Self { tokens, position: 0 }
    }

    pub fn to_ast(mut self) {
        self.parse();
    }


    const P_ASSIGMENT: fn(Expression, u8,&mut Parser) -> Expression = |expr, rbp, s| {
        let left = expr;
        let right = s.parse_expression(rbp);
        Expression::VariableAssignment {
            left: Box::new(left),
            right: Box::new(right)
        }
    };
    const P_ADD: fn(Expression, u8, &mut Parser) -> Expression = |expr, rbp, s| {
        let left = expr;
        let right = s.parse_expression(rbp);
        Expression::Add {
            left: Box::new(left),
            right: Box::new(right)
        }
    };
    const P_MINUS: fn(Expression, u8, &mut Parser) -> Expression = |expr, rbp, s| {
        let left = expr;
        let right = s.parse_expression(rbp);
        Expression::Minus {
            left: Box::new(left),
            right: Box::new(right)
        }
    };
    const P_MULTIPLY: fn(Expression, u8, &mut Parser) -> Expression = |expr, rbp, s| {
        let left = expr;
        let right = s.parse_expression(rbp);
        Expression::Multiply {
            left: Box::new(left),
            right: Box::new(right)
        }
    };
    const P_DIVIDE: fn(Expression, u8, &mut Parser) -> Expression = |expr, rbp, s| {
        let left = expr;
        let right = s.parse_expression(rbp);
        Expression::Divide {
            left: Box::new(left),
            right: Box::new(right)
        }
    };
    fn parse(&mut self) {
        let a = self.parse_expression(0);
        println!("{:#?}", a)
    }


    fn parse_consumed(&mut self) -> Expression {
        match self.consume() {
            Token::LiteralStr(s) => Expression::LiteralStr(s),
            Token::Identifier(s) => Expression::Identifier(s),
            Token::Not => Expression::Not(Box::new(self.parse_expression(Token::Not.get_single_bp()))),
            Token::LiteralInt(s) => {
              let Ok(s) = s.parse::<i64>() else {
                  panic!("Could not parse number");
              };
                Expression::LiteralInt(s)
            },
            _ => Expression::None,
        }
    }
    fn parse_nud(&mut self) -> Expression {
        self.next();
        self.parse_consumed()
    }

    fn parse_expression(&mut self, bp: u8) -> Expression {
        let mut left = self.parse_nud();
        if let Expression::None = left {
            println!("[PARSER ERR OF CASTING ERR]: casting return 'Expression::None' for Token of '{:?}', syntax may be malformed", self.get());
            panic!("Parser panic due to invalid expression");
        }

        while let Some((lbp, rbp)) = self.peek().get_bp() {
            if lbp < bp {
                break;
            }
            match self.next().consume() {
                Token::Assignment => {
                    left = Self::P_ASSIGMENT(left, rbp, self);
                }
                Token::Add => {
                    left = Self::P_ADD(left, rbp, self);
                },
                Token::Minus => {
                    left = Self::P_MINUS(left, rbp, self);
                }
                Token::Multiply => {
                    left = Self::P_MULTIPLY(left, rbp, self);
                }
                Token::Divide => {
                    left = Self::P_DIVIDE(left, rbp, self);
                }
                _ => {}
            }
        }

        left
    }

    fn next(&mut self) -> &mut Self {
        if self.position + 1 < self.tokens.len() {
            self.position += 1;
        }
        self
    }

    fn get(&self) -> &Token {
        &self.tokens[self.position]
    }
    fn consume(&mut self) -> Token {
        let t = self.tokens.remove(self.position);
        self.position = self.position.saturating_sub(1);
        t
    }
    fn peek(&self) -> &Token {
        if self.position + 1 < self.tokens.len() {
            &self.tokens[self.position + 1]
        } else {
            &self.tokens[self.position]
        }
    }
}
