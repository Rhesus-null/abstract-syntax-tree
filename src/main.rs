use crate::lexer::Lexer;

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
    let mut lexer = Lexer::new(b"if A.B.C(32 * 4) / 3 ").parse().unwrap();
    println!("{:#?}", lexer);
    let ast = Parser::new(lexer).to_ast();
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

struct Parser {
    tokens : Vec<Token>,
    parse_error: Vec<String>,
    position : usize,
}

impl Parser {
    pub fn new(tokens : Vec<Token>) -> Self {
        Self { tokens, parse_error: Vec::new(), position: 0}
    }

    /// turn the containing Token to ast
    pub fn to_ast(mut self) {
        self.parse();
    }


    /// The main Parser
    fn parse(&mut self) {
        println!("{:#?}", self.parse_expression("START",0));
    }

    fn parse_expression(&mut self, _debug: &str, bp: u8) -> Result<Expression, &'static str> {
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
    fn parse_parenthesis_or_tuple(&mut self, _debug: Option<&str>) -> Result<Expression, &'static str> {
        let mut list = Vec::<Expression>::new();
        loop {
            if self.end() {
                return Err("Unclosed 'OpenParenthesis' when reached EOF.");
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
                return Err("Expected Comma For Tuple.");
            }
            let result = self.parse_expression(_debug.unwrap_or("parse_parenthesis_or_tuple"), 1);
            if let Ok(result) = result {
                list.push(result);
            } else {
                return Err(result.unwrap_err())
            }
        }
        if list.len() == 1 {
            return Ok(Expression::Parenthesis(Box::new(list.pop().unwrap())));
        }
        Ok(Expression::Tuple(list))
    }
    fn parse_nud(&mut self) -> Result<Expression, &'static str> {
        match self.next().consume() {
            Token::LiteralStr(s) => Ok(Expression::LiteralStr(s)),
            Token::Identifier(s) => Ok(Expression::Identifier(s)),
            Token::Not => {
                let result = self.parse_expression(Token::Not.get_name(),Token::Not.get_single_bp().unwrap());
                if let Ok(result) = result {
                    return Ok(Expression::Not(Box::new(result)))
                }
                Err(result.unwrap_err())
            },
            Token::OpenParenthesis => {
                let result =self.parse_parenthesis_or_tuple(Some("parse_nud.parse_parenthesis_or_tuple.OpenParenthesis"));
                if let Ok(result) = result {
                    return Ok(result);
                }
                Err(result.unwrap_err())
            },
            Token::LiteralDigit(s) => {
                let Ok(s) = s.parse::<i64>() else {
                    return Err("Cannot Parse To Number");
                };
                Ok(Expression::LiteralInt(s))
            },
            _ => Err("Error"),
        }
    }

    fn parse_led(&mut self, left: Expression, rbp: u8) ->  Result<Expression, &'static str> {
         match self.next().consume() {
             Token::Assignment => {
                 let Ok(result)=self.parse_expression(Token::Assignment.get_name(),rbp)else {
                     return Err("Could not parse Assignment");
                 };
                 Ok(Expression::VariableAssignment {
                    left: Box::new(left),
                    right: Box::new(result)
                 })
             }
             Token::Add => {
                 let Ok(result) =self.parse_expression(Token::Add.get_name(),rbp) else {
                     return Err("Could not parse Add");
                 };
                 Ok(Expression::Add {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             },
             Token::Minus => {
                 let Ok(result) =self.parse_expression(Token::Minus.get_name(),rbp) else {
                     return Err("Could not parse Minus");
                 };
                 Ok(Expression::Minus {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::Multiply => {
                 let Ok(result) =self.parse_expression(Token::Multiply.get_name(),rbp) else {
                     return Err("Could not parse Multiply");
                 };
                 Ok(Expression::Multiply {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::Divide => {
                 let Ok(result) =self.parse_expression(Token::Divide.get_name(),rbp) else {
                     return Err("Could not parse Divide");
                 };
                 Ok(Expression::Divide {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             },
             Token::Equal => {
                 let Ok(result) =self.parse_expression(Token::Equal.get_name(),rbp) else {
                     return Err("Could not parse Equal");
                 };
                 Ok(Expression::Equal {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::NotEqual => {
                 let Ok(result) =self.parse_expression(Token::NotEqual.get_name(),rbp) else {
                     return Err("Could not parse NotEqual");
                 };
                 Ok(Expression::NotEqual {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::GreaterOrEqual => {
                 let Ok(result) =self.parse_expression(Token::GreaterOrEqual.get_name(),rbp) else {
                     return Err("Could not parse GreaterOrEqual");
                 };
                 Ok(Expression::GreaterOrEqual {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::LessOrEqual => {
                 let Ok(result) =self.parse_expression(Token::LessOrEqual.get_name(),rbp) else {
                     return Err("Could not parse LessOrEqual");
                 };
                 Ok(Expression::LessOrEqual {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::GreaterThan => {
                 let Ok(result) =self.parse_expression(Token::GreaterThan.get_name(),rbp) else {
                     return Err("Could not parse GreaterThan");
                 };
                 Ok(Expression::GreaterThan {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::LessThan => {
                 let Ok(result) =self.parse_expression(Token::LessThan.get_name(),rbp) else {
                     return Err("Could not parse LessThan");
                 };
                 Ok(Expression::LessThan {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::And => {
                 let Ok(result) =self.parse_expression(Token::And.get_name(),rbp) else {
                     return Err("Could not parse And");
                 };
                 Ok(Expression::And {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::Or => {
                 let Ok(result) =self.parse_expression(Token::Or.get_name(),rbp) else {
                     return Err("Could not parse Or");
                 };
                 Ok(Expression::Or {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::Dot => {
                 let Ok(result) =self.parse_expression(Token::Dot.get_name(),rbp) else {
                     return Err("Could not parse AccessMember");
                 };
                 Ok(Expression::AccessMember {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             Token::OpenParenthesis => {
                 let Ok(result) =self.parse_parenthesis_or_tuple(Some("membercall")) else {
                     return Err("Could not parse MemberCall");
                 };
                 Ok(Expression::MemberCall {
                     left: Box::new(left),
                     right: Box::new(result)
                 })
             }
             T => {Err("")}
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
