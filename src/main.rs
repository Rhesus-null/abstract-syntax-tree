#[derive(Debug, Clone)] #[allow(unused)]
enum Token {
    If,
    Identifier(String),
    Assignment,
    Literal(String),
    Add,
}

#[allow(unused)]
impl Token {
    pub fn get_bp(&self) -> Option<(u8, u8)> {
        match self {
            Token::Assignment => Some((2, 1)),
            Token::Add => Some((3, 4)),
            _ => None,
        }
    }
    pub fn is_keyword(&self) -> bool {
        match self {
            Token::If => true,
            _ => false,
        }
    }
    pub fn is_nud(&self) -> bool {
        match self {
            Token::Identifier(_) => true,
            Token::Literal(_) => true,
            _ => false,
        }
    }
    pub fn is_led(&self) -> bool {
        match self {
            Token::Assignment => true,
            Token::Add => true,
            _ => false,
        }
    }
}

fn main() {
    let mut tokens : Vec<Token> = Vec::new();

    tokens.push(Token::If); //start
    tokens.push(Token::Identifier("ab".to_string())); // nud, bp = 0
    tokens.push(Token::Assignment); // peek and get bp, lbp = 2, rbp = 1. 2 < 1 = false. next().get() (move into these pos), call P_ASSIGNMENT.
    tokens.push(Token::Literal(" nice".to_string())); //nud, P_ASSIGNMENT have LHS = Identifier("ab") with RHS Literal(" nice").
    tokens.push(Token::Identifier("ab2".to_string())); // peek and get bp. but this return none because this isn't infix (syntax error)
    // syntax error
    tokens.push(Token::Assignment);
    tokens.push(Token::Literal("nice".to_string()));
    // syntax error
    /* result:
        VariableAssignment {
        left: Identifier(
            "ab",
        ),
        right: Literal(
            " nice",
        ),
    */ // total recursive level = 2, due to syntax error.
    let parser = Parser::new(tokens);
    parser.to_ast();
}

#[derive(Debug)] #[allow(unused)]
enum Expression {
    None,
    Identifier(String),
    Literal(String),
    VariableAssignment {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Add {
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

    fn parse(&mut self) {
        let a = self.parse_expression(0);
        println!("{:#?}", a)
    }

    fn parse_expression(&mut self, bp: u8) -> Expression {
        let mut left = match self.next().get() {
            Token::Identifier(s) => Expression::Identifier(s.clone()),
            Token::Literal(s) => Expression::Literal(s.clone()),
            _ => Expression::None,
        };
        if let Expression::None = left {
            println!("[PARSER ERR OF CASTING ERR]: casting return 'Expression::None' for Token of '{:?}', syntax may be malformed", self.get());
            panic!("Parser panic due to invalid expression");
        }

        if self.get().is_nud() && self.peek().is_nud() {
            println!("[PARSER ERR OF SYNTAX ERR]: Null Detonation appear twice for '{:?}'. The following Null Detonation are '{:?}'", self.get(), self.peek());
        }

        while let Some((lbp, rbp)) = self.peek().get_bp() {
            if lbp < bp {
                break;
            }
            match self.next().get() {
                Token::Assignment => {
                    left = Self::P_ASSIGMENT(left, rbp, self);
                }
                Token::Add => {
                    left = Self::P_ADD(left, rbp, self);
                }
                _ => {
                    println!("unexpected")
                }
            }
        }

        left
    }

    fn next(&mut self) -> &mut Self {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
        self
    }
    fn get(&self) -> &Token {
        &self.tokens[self.position]
    }
    fn peek(&self) -> &Token {
        if self.position + 1 < self.tokens.len() {
            &self.tokens[self.position + 1]
        } else {
            &self.tokens[self.position]
        }
    }
}