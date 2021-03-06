use crate::parser::asp_comparison::AspComparison;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::error::AspParseError;
use crate::parser::asp_expr::AspExpr;

#[derive(Debug)]
pub struct AspNotTest {
    not: bool,
    comparison: AspComparison,
}

impl AspNotTest {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspNotTest, AspParseError> {
        logger.enter_parser("AspNotTest");

        let a = AspNotTest { not: false, comparison: AspComparison::parse(sc, logger)?  };

        logger.leave_parser("AspNotTest");
        Ok(a)
    }
}
