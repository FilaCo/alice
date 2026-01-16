use crate::ir::Span;

#[salsa::tracked(debug)]
pub struct Ast<'db> {
    pub statements: Vec<Statement<'db>>,
}

#[salsa::tracked(debug)]
pub struct Statement<'db> {
    pub kind: StmtKind<'db>,
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum StmtKind<'db> {
    Definition(Definition<'db>),
    Import,
}

#[salsa::tracked(debug)]
pub struct Definition<'db> {
    id: DefinitionId<'db>,
    kind: DefKind,
}

#[salsa::interned(debug)]
pub struct DefinitionId<'db> {
    #[returns(ref)]
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum DefKind {
    Property,
    System,
}

// #[salsa::tracked(debug)]
// pub struct Expr<'db> {
//     #[tracked]
//     pub kind: ExprKind<'db>,
//     #[tracked]
//     pub span: Span<'db>,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
// pub enum ExprKind<'db> {
//     Binary {
//         lhs: Box<Expr<'db>>,
//         op: BinaryOp<'db>,
//         rhs: Box<Expr<'db>>,
//     },
//     Unary {
//         op: UnaryOp<'db>,
//         rhs: Box<Expr<'db>>,
//     },
//     Grouped {
//         expr: Box<Expr<'db>>,
//     },
// }

// #[salsa::tracked(debug)]
// pub struct BinaryOp<'db> {
//     #[tracked]
//     pub kind: BinaryOpKind,
//     #[tracked]
//     pub span: Span<'db>,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq, salsa::Update)]
// pub enum BinaryOpKind {
//     /// The `+` operator (addition)
//     Add,
//     /// The `-` operator (subtraction)
//     Sub,
//     /// The `*` operator (multiplication)
//     Mul,
//     /// The `/` operator (division)
//     Div,
// }

// #[salsa::tracked(debug)]
// pub struct UnaryOp<'db> {
//     #[tracked]
//     pub kind: UnaryOpKind,
//     #[tracked]
//     pub span: Span<'db>,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, salsa::Update)]
// pub enum UnaryOpKind {
//     /// The `-` operator for negation
//     Neg,
// }
