// #[salsa::tracked(debug)]
// pub struct Ast<'db> {
//     #[tracked]
//     #[returns(ref)]
//     pub root: AstNode<'db>,
// }

// #[derive(Debug, PartialEq, Eq, salsa::Update)]
// pub enum AstNode<'db> {
//     Expr,
// }
