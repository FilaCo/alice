use crate::{
    source::{Span, Spanned},
    token::{Lit, LitKind, Token, TokenKind},
};

// #[salsa::tracked(debug)]
// pub struct Ast<'db> {
//     #[tracked]
//     #[returns(ref)]
//     pub root: AstNode<'db>,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq, salsa::Update)]
// pub struct AstNode<'db> {
//     pub kind: AstNodeKind,
//     pub span: Span<'db>,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq, salsa::Update)]
// pub enum AstNodeKind {
//     Decl,
//     Expr { kind: ExprKind },
// }

#[derive(Clone, Debug, PartialEq, salsa::Update)]
pub struct Expr<'db> {
    pub kind: ExprKind<'db>,
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, salsa::Update)]
pub enum ExprKind<'db> {
    Binary {
        op: BinaryOp<'db>,
        lhs: Box<Expr<'db>>,
        rhs: Box<Expr<'db>>,
    },
    Unary {
        op: UnaryOp<'db>,
        rhs: Box<Expr<'db>>,
    },
    Grouped {
        expr: Box<Expr<'db>>,
    },
    Lit(Lit<'db>),
    Err,
}

pub type BinaryOp<'db> = Spanned<'db, BinaryOpKind>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, salsa::Update)]
pub enum BinaryOpKind {
    /// The `+` operator (addition)
    Add,
    /// The `-` operator (subtraction)
    Sub,
    /// The `*` operator (multiplication)
    Mul,
    /// The `/` operator (division)
    Div,
}

impl BinaryOpKind {
    pub fn from_token(tok: Token<'_>) -> Option<Self> {
        use BinaryOpKind::*;
        match tok.kind {
            TokenKind::Minus => Some(Sub),
            TokenKind::Plus => Some(Add),
            TokenKind::Slash => Some(Div),
            TokenKind::Star => Some(Mul),
            _ => None,
        }
    }
}

pub type UnaryOp<'db> = Spanned<'db, UnaryOpKind>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, salsa::Update)]
pub enum UnaryOpKind {
    /// The `-` operator for negation
    Neg,
}
