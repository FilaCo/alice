#[salsa::tracked(debug)]
pub struct Ast<'db> {}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
}

#[derive(Debug)]
pub enum NodeKind {
    Decl,
    Expr,
}

pub enum DeclKind {
    Component,
    System,
    Module,
    Var,
}
