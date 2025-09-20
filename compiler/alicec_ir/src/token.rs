use alicec_db::prelude::Symbol;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'db> {
    Lit {
        kind: LitKind,
        symbol: Symbol<'db>,
    },

    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,

    /// `(`
    LParen,
    /// `)`
    RParen,

    /// An end of input.
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LitKind {
    Int,
    Float,
    Err,
}
