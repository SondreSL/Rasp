use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::RuntimeValue;
use crate::runtime::runtime::Scope;
use crate::log::logger::Logger;
use std::io;

#[derive(Debug)]
pub struct AspAssignment {
    name: AspName,
    expr: AspExpr
}

impl AspAssignment {
    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspAssignment, AspParseError> {

        logger.enter_parser("AspAssignment")?;

        let asp_assignment = match sc.cur_token() {
            Token::Name(_) => {
                let name = AspName::parse(sc, logger)?;
                sc.skip(Token::Equal)?;
                let expr = AspExpr::parse(sc, logger)?;
                sc.skip(Token::Newline)?;
                Ok(AspAssignment {name,expr} )
            },
            token => return Err(AspParseError::Expected {
                expected: Token::Name(String::new()),
                found: token.clone(),
                line_number: sc.cur_line() as usize
            })
        };

        logger.leave_parser("AspAssignment")?;
        asp_assignment
    }

    pub fn eval(&self, cur_scope: &mut Scope) -> RuntimeValue {
        // TODO
        // Need to make the scopes that something other than String, or
        // get a string from AspName. 
        // cur_scope.insert(self.name.name, self.expr.eval(&mut cur_scope));
        RuntimeValue::RuntimeNone
    }

    pub fn test_parser(&self, logger: &mut Logger) -> io::Result<()> {
        logger.enter_parser("AspAssignment")?;

//        self.name.test_parser(logger)?;
//        self.expr.test_parser(logger)?;

        logger.leave_parser("AspAssignment")
    }
}
