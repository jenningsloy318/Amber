use heraclitus_compiler::prelude::*;
use crate::parser::ParserMetadata;
use super::{super::expr::Expr, Binop};

#[derive(Debug)]
pub struct Add {
    left: Box<Expr>,
    right: Box<Expr>
}

impl SyntaxModule<ParserMetadata> for Add {
    fn new() -> Self {
        Add {
            left: Box::new(Expr::new()),
            right: Box::new(Expr::new())
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        Binop::parse_left_expr(meta, &mut *self.left, "+")?;
        token(meta, "+")?;
        syntax(meta, &mut *self.right)?;
        Ok(())
    }
}