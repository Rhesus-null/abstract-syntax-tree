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
    pub fn get_lbp(&self) -> Option<u8> {
        if let Some((l, _)) = self.get_bp() { return Some(l) }
        None
    }
    pub fn get_bp(&self) -> Option<(u8, u8)> {
        match self {
            Token::Assignment => Some((2, 1)),
            Token::Add => Some((3, 4)),
            Token::Minus => Some((3, 4)),
            Token::Multiply => Some((5, 6)),
            Token::Divide => Some((5, 6)),
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
    // tokens.push(Token::LiteralInt("34".to_string()));
    // tokens.push(Token::Add);
    // tokens.push(Token::LiteralInt("34".to_string()));
    // tokens.push(Token::Minus);
    // tokens.push(Token::LiteralInt("11".to_string()));
    // tokens.push(Token::Multiply);
    // tokens.push(Token::LiteralInt("5".to_string()));
    // tokens.push(Token::Divide);
    // tokens.push(Token::LiteralInt("5".to_string()));
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
    position : usize,
    stack_trace: Option<Vec<String>>,
}

impl Parser {
    pub fn new(tokens : Vec<Token>) -> Self {
        Self { tokens, position: 0, stack_trace: None}
    }

    /// turn the containing Token to ast
    pub fn to_ast(mut self) {
        self.parse();
    }
    /// enable trace, useful for debugging
    pub fn enable_trace(&mut self) {
        self.stack_trace = Some(Vec::new());
    }
    /// get the trace.
    pub fn get_trace(&mut self) -> Option<Vec<String>> {
        self.stack_trace.take()
    }

    pub fn print_trace(&mut self) {
        if let Some(t) =self.get_trace() {
            println!("[TRACE]\n");
            for v in t {
                println!("{}", v);
            }
        }
    }


    /// The main Parser
    fn parse(&mut self) {
        self.enable_trace();
        println!("{:#?}", self.parse_expression(0));
        self.print_trace();
    }

    fn parse_expression(&mut self, bp: u8) -> Expression {
        let mut left = self.parse_nud();
        self.trace(|c| c.push(format!("{} -> NUD: '{:?}'",format!("[parse_expression](bp: {})", bp).bright_magenta(), left)));
        loop {
            if self.end() {
                self.trace(|c| c.push(format!("{} -> EOF: return '{:?}'",format!("[parse_expression](bp: {})", bp).bright_magenta(), left)));
                break;
            }
            let peek = self.peek();
            if let Some((lbp, rbp)) = peek.get_bp() {
                if lbp < bp {
                    break;
                }
                left = self.parse_led(left, rbp);
            } else { break }
        }
        left
    }

    fn parse_nud(&mut self) -> Expression {
        self.trace(|c| c.push(format!("{} -> Called", "[parse_nud]".bright_magenta())));
        match self.next().consume() {
            Token::LiteralStr(s) => Expression::LiteralStr(s),
            Token::Identifier(s) => Expression::Identifier(s),
            Token::Not => Expression::Not(Box::new(self.parse_expression(Token::Not.get_single_bp().unwrap()))),
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
        self.trace(|c| c.push(format!("{} -> Called",format!("[parse_led](rbp: {})", rbp).bright_magenta())));
        match self.next().consume() {
            Token::Assignment => {
                Expression::VariableAssignment {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(rbp))
                }
            }
            Token::Add => {
                Expression::Add {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(rbp))
                }
            },
            Token::Minus => {
                Expression::Minus {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(rbp))
                }
            }
            Token::Multiply => {
                Expression::Multiply {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(rbp))
                }
            }
            Token::Divide => {
                Expression::Divide {
                    left: Box::new(left),
                    right: Box::new(self.parse_expression(rbp))
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

            self.trace(|t| t.push(format!("{} -> increment position by one", "[next]".bright_black())));
        }
        self
    }

    /// consume the current item, replacing [Token::NONE] in its place
    fn consume(&mut self) -> Token {
        let t = std::mem::replace(&mut self.tokens[self.position], Token::NONE);
        self.trace_mut(|v, s| {
            v.push(format!("{}\n| consumed '{:?}'", "[consume]".bright_black(), t));
            v.push(format!("| Vec: {:?}", s.tokens))
        });
        t
    }

    /// peek next.
    fn peek(&self) -> &Token {
        if self.position + 1 < self.tokens.len() {
            &self.tokens[self.position + 1]
        } else {
            &self.tokens[self.position]
        }
    }
    fn trace(&mut self, mut t: impl FnMut(&mut Vec<String>)) {
        if let Some(vec) = self.stack_trace.as_mut() {
            t(vec);
        }
    }
    fn trace_mut(&mut self, mut t: impl FnMut(&mut Vec<String>,&mut Self)) {
        if let Some(mut vec) = self.stack_trace.take() {
            t(&mut vec, self);
            self.stack_trace = Some(vec);
        }
    }
}
