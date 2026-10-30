use crate::{ErrorKind, Expression, Statement, StatementErrorKind, TokenKind};


pub enum MustBe {
    Condition,
}

pub struct StaticAnalyzer;
impl StaticAnalyzer
{

    pub fn new(list: Vec<Statement>) -> Result<(), Vec<String>> {
        let mut err_list = Vec::<String>::new();
        for statement in list {
            if let Some(t) = Self::evaluate(statement) {
                err_list.push(t);
            }
        }
        if err_list.is_empty() {
            return Ok(());
        }
        Err(err_list)
    }

    pub fn evaluate(stmt: Statement) -> Option<String> {
        match stmt {
            Statement::ERROR {error, origin, origin_value } => {
                match (error, origin, origin_value) {
                    (StatementErrorKind::OtherErr(T), TokenKind::If, Any) => {
                        match (T, Any) {
                            (T, Some(f)) => {
                                let result = Self::evaluate_expression(f, MustBe::Condition);
                                if result.is_some() {
                                    return result;
                                }
                                Self::evaluate(Statement::ERROR {error: StatementErrorKind::OtherErr(T), origin: TokenKind::If, origin_value: None})
                            },
                            (ErrorKind::MissingToken(T), _) => match T {
                                TokenKind::CloseBrace => Some("If Statement missing '}', believe it or not you may want to add '}' after the '{'".to_string()),
                                T => Some(format!("If Statement Missing Token '{}'", T.get_name())),
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Statement::If {condition, body } => {
                if let Some(t) = Self::evaluate_expression(condition, MustBe::Condition) {
                    return Some(t);
                }
                for statement in body {
                    if let Some(t) = Self::evaluate(statement) {
                        return Some(t);
                    }
                }
                None
            }
            Statement::ImmutableVariableDeclaration { .. } => { None }
            Statement::Any { .. } => {None}
        }
    }

    pub fn evaluate_expression(expression: Expression, must_be: MustBe) -> Option<String>  {
        match must_be {
            MustBe::Condition => match expression {
                Expression::ERROR {error, origin} => {
                    match error {
                        ErrorKind::EOF => {
                            Some(format!("End Of File while processing '{}'", origin.get_name()))
                        }
                        ErrorKind::MissingToken(t) => {
                            Some(format!("Expect Condition but token '{}' missing token '{}', you may want to add the missing token.", origin.get_name(), t.get_name()))
                        }
                        ErrorKind::CannotParse => {
                            Some(format!("'{}' is not a valid expression for a condition", origin.get_name()))
                        }
                    }
                }
                Expression::Not(t) => Self::evaluate_expression(*t, MustBe::Condition),
                Expression::VariableAssignment { .. } => Some("Expect Condition got VariableAssignment instead.".to_string()),
                Expression::Add { .. } => Some("Expect Condition got Additions instead.".to_string()),
                Expression::Minus { .. } => Some("Expect Condition got Subtraction instead.".to_string()),
                Expression::Multiply { .. } => Some("Expect Condition got Multiplication instead.".to_string()),
                Expression::Divide { .. } => Some("Expect Condition got Division instead.".to_string()),
                Expression::AccessMember { .. } => None, /* these can return bool, must implement type inference in order to static analyzer */
                Expression::MemberCall { .. } => None, /* these can return bool, must implement type inference in order to static analyzer */
                Expression::Tuple(_) => Some("Expect Condition got Tuple instead.".to_string()),
                Expression::Parenthesis(t) => Self::evaluate_expression(*t, MustBe::Condition),

                Expression::Identifier(_) => None,  /* this can point bool */
                Expression::LiteralStr(_) => Some("Expect Condition got LiteralStr instead.".to_string()),
                Expression::LiteralInt(_) => Some("Expect Condition got LiteralInt instead.".to_string()),

                _ => None
            }
        }
    }
}