#[salsa::tracked(debug)]
pub struct Ast<'db> {}

#[derive(Debug)]
pub enum Node {
    Decl,
    Expr,
}

pub enum DeclKind {
    Component,
    System,
    Module,
    Var,
}
