use crate::span::Span;

#[salsa::tracked(debug)]
pub struct Cake<'db> {
    #[tracked]
    #[returns(ref)]
    pub stmts: Vec<Stmt<'db>>,
}

#[salsa::tracked(debug)]
pub struct Stmt<'db> {
    #[tracked]
    pub kind: StmtKind<'db>,
    #[tracked]
    pub span: Span<'db>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum StmtKind<'db> {
    Def(Def<'db>),
    Expr(Expr<'db>),
    Include,
}

#[salsa::tracked(debug)]
pub struct Def<'db> {}

#[salsa::tracked(debug)]
pub struct Expr<'db> {
    #[tracked]
    pub kind: ExprKind<'db>,
    #[tracked]
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum ExprKind<'db> {
    Binary(BinaryExpr<'db>),
    Unary(UnaryExpr<'db>),
    Grouped(GroupedExpr<'db>),
}

#[salsa::tracked(debug)]
pub struct BinaryExpr<'db> {
    #[tracked]
    pub lhs: Box<Expr<'db>>,
    #[tracked]
    pub op: BinaryOp<'db>,
    #[tracked]
    pub rhs: Box<Expr<'db>>,
}

#[salsa::tracked(debug)]
pub struct BinaryOp<'db> {
    #[tracked]
    pub kind: BinaryOpKind,
    #[tracked]
    pub span: Span<'db>,
}

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

#[salsa::tracked(debug)]
pub struct UnaryExpr<'db> {
    #[tracked]
    pub op: UnaryOp<'db>,
    #[tracked]
    pub expr: Box<Expr<'db>>,
}

#[salsa::tracked(debug)]
pub struct UnaryOp<'db> {
    #[tracked]
    pub kind: UnaryOpKind,
    #[tracked]
    pub span: Span<'db>,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, salsa::Update)]
pub enum UnaryOpKind {
    /// The `-` operator for negation
    Neg,
}

#[salsa::tracked(debug)]
pub struct GroupedExpr<'db> {
    #[tracked]
    pub expr: Box<Expr<'db>>,
}
