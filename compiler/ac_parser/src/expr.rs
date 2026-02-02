use ac_ir::syntax::ast::Expr;

use crate::parser::Parser;

impl<'db> Parser<'db> {
    /// ```ebnf
    /// Expr = Equality .
    /// ```
    fn expr(&mut self) -> Expr<'db> {
        self.equality()
    }

    /// ```ebnf
    /// Equality = Comparison { ( "==" | "!=" ) Comparison } .
    /// ```
    fn equality(&mut self) -> Expr<'db> {
        let lhs = self.comparison();
        todo!()
    }

    /// ```ebnf
    /// Comparison = Term { ( "<" | "<=" | ">" | ">=" ) Term } .
    /// ```
    fn comparison(&mut self) -> Expr<'db> {
        todo!()
    }

    /// ```ebnf
    /// Term = Factor { ( "+" | "-" ) Factor } .
    /// ```
    fn term(&mut self) -> Expr<'db> {
        todo!()
    }

    /// ```ebnf
    /// Factor = Unary { ( "*" | "/" ) Unary } .
    /// ```
    fn factor(&mut self) -> Expr<'db> {
        todo!()
    }

    /// ```ebnf
    /// Unary = ( "-" | "not" ) Unary | Primary .
    /// ```
    fn unary(&mut self) -> Expr<'db> {
        todo!()
    }

    /// ```ebnf
    /// Primary = Literal | Grouped .
    /// ```
    fn primary(&mut self) -> Expr<'db> {
        todo!()
    }

    /// ```ebnf
    /// Grouped = "(" Expr ")" .
    /// ```
    fn grouped(&mut self) -> Expr<'db> {
        todo!()
    }

    fn left_assoc(&mut self) {}
}
