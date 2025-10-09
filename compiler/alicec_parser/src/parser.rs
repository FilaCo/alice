use ExprKind::*;
use alicec_db::prelude::AlicecDbTrait;
use alicec_diag::prelude::Diagnostic;
use alicec_ir::prelude::{
    BinaryOpKind, Expr, ExprKind, Span, Spanned, Token, TokenKind, UnaryOpKind,
};
use alicec_lexer::prelude::Lexer;
use salsa::Accumulator;

pub struct Parser<'db> {
    db: &'db dyn AlicecDbTrait,
    lexer: Lexer<'db>,
    token: Token<'db>,
    prev: Token<'db>,
}

impl<'db> Parser<'db> {
    pub fn new(db: &'db dyn AlicecDbTrait, src: &'db str) -> Self {
        let lexer = Lexer::new(db, src);
        let mut parser = Self {
            db,
            lexer,
            token: Token::dummy(db),
            prev: Token::dummy(db),
        };

        parser.swallow();
        parser
    }

    /// ```ebnf
    /// expr = term;
    /// ```
    pub fn parse_expr(&mut self) -> Expr<'db> {
        self.parse_term()
    }

    ///```ebnf
    /// term = factor { ("Minus" | "Plus") factor };
    /// ```
    fn parse_term(&mut self) -> Expr<'db> {
        self.parse_left_assoc_binary_op(
            |this| this.parse_factor(),
            &[TokenKind::Minus, TokenKind::Plus],
        )
    }

    ///```ebnf
    /// factor = unary { ("Slash" | "Star") unary };
    /// ```
    fn parse_factor(&mut self) -> Expr<'db> {
        self.parse_left_assoc_binary_op(
            |this| this.parse_unary(),
            &[TokenKind::Slash, TokenKind::Star],
        )
    }

    /// ```ebnf
    /// unary = "Minus" unary | primary;
    /// ```
    fn parse_unary(&mut self) -> Expr<'db> {
        if self.eat(&TokenKind::Minus) {
            let op = Spanned::new(UnaryOpKind::Neg, self.prev.span);
            let rhs = Box::new(self.parse_unary());
            let span = Span::new(self.db, op.span.lo(self.db), rhs.span.hi(self.db));
            let kind = Unary { op, rhs };

            return Expr { kind, span };
        }
        self.parse_primary()
    }

    /// ```ebnf
    /// primary = "Lit" | grouped;
    /// ```
    fn parse_primary(&mut self) -> Expr<'db> {
        // self.swallow();
        // match self.prev.kind {
        //     TokenKind::Lit(lit) => Expr {
        //         kind: Lit(lit),
        //         span: self.prev.span,
        //     },
        //     TokenKind::LParen => {
        //         let lo = self.prev.span.lo(self.db);
        //         let mut expr = self.parse_expr();
        //         if !self.eat(&TokenKind::RParen) {
        //             Diagnostic::unexpected_token().accumulate(self.db);
        //         }
        //         let span = Span::new(self.db, lo, self.prev.span.hi(self.db));
        //         expr.span = span;
        //         expr
        //     }
        //     _ => Expr {
        //         kind: Err,
        //         span: self.token.span,
        //     },
        // }
    }

    /// ```ebnf
    /// grouped = "LParen" expr "RParen";
    /// ```
    fn parse_grouped(&mut self) -> Expr<'db> {
        todo!()
    }

    fn parse_left_assoc_binary_op(
        &mut self,
        lower: impl Fn(&mut Self) -> Expr<'db>,
        op_kinds: &[TokenKind],
    ) -> Expr<'db> {
        let mut lhs = lower(self);
        while self.eat_one_of(op_kinds) {
            let op = Spanned::new(
                BinaryOpKind::from_token(self.prev).expect("invalid binary op token"), // TODO: change expect to diagnostic and recovery
                self.prev.span,
            );
            let rhs = Box::new(lower(self));
            let span = Span::new(self.db, lhs.span.lo(self.db), rhs.span.hi(self.db));
            let kind = Binary {
                op,
                lhs: Box::new(lhs),
                rhs,
            };
            lhs = Expr { kind, span };
        }

        lhs
    }

    fn eat_one_of(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.eat(kind) {
                return true;
            }
        }
        false
    }

    fn eat(&mut self, kind: &TokenKind) -> bool {
        let is_seeing = self.is_seeing(kind);
        if is_seeing {
            self.swallow();
        }
        is_seeing
    }

    fn is_seeing(&self, kind: &TokenKind) -> bool {
        self.token == *kind
    }

    fn swallow(&mut self) {
        let lexeme = self.lexer.cook();
        self.prev = std::mem::replace(&mut self.token, lexeme);
    }
}
