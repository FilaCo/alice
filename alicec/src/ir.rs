#[salsa::input(debug)]
pub struct SourceFile {
    #[returns(ref)]
    pub text: String,
}

#[salsa::interned(debug)]
pub struct PropId<'db> {
    #[returns(ref)]
    pub value: String,
}

#[salsa::interned(debug)]
pub struct SysId<'db> {
    #[returns(ref)]
    pub value: String,
}

#[salsa::tracked(debug)]
pub struct Span<'db> {
    #[tracked]
    pub lo: usize,
    #[tracked]
    pub hi: usize,
}

#[salsa::tracked(debug)]
pub struct Ast<'db> {
    pub root: Node<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum Node<'db> {
    Decl(Decl<'db>),
    Expr(Expr<'db>),
}

#[salsa::tracked(debug)]
pub struct Decl<'db> {
    #[tracked]
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub enum DeclKind<'db> {
    Prop { id: PropId<'db> },
    Sys { id: SysId<'db> },
}

#[salsa::tracked(debug)]
pub struct Expr<'db> {
    #[tracked]
    pub kind: ExprKind<'db>,
    #[tracked]
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum ExprKind<'db> {
    Binary {
        lhs: Box<Expr<'db>>,
        op: BinaryOp<'db>,
        rhs: Box<Expr<'db>>,
    },
    Unary {
        op: UnaryOp<'db>,
        rhs: Box<Expr<'db>>,
    },
    Grouped {
        expr: Box<Expr<'db>>,
    },
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
