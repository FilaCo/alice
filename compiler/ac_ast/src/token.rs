use ac_db::{span::Span, symbol::Symbol};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span<'db>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind<'db> {
    /* Expression-operator symbols. */
    /// `=`
    Eq,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `==`
    EqEq,
    /// `!=`
    Ne,
    /// `>=`
    Ge,
    /// `>`
    Gt,
    // `+`
    Plus,
    // `-`
    Minus,
    // `*`
    Star,
    // `/`
    Slash,

    /* Structural symbols */
    /// `.`
    Dot,
    /// `,`
    Comma,
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `::`
    ColonColon,
    /// `#`
    Pound,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,

    /* Literals */
    Literal {
        value: Literal<'db>,
    },

    Ident {
        sym: Symbol<'db>,
    },

    /// An end of input.
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Literal<'db> {
    pub kind: LiteralKind,
    pub sym: Symbol<'db>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Bool, // AST only, never apper in a `[Token]`
    Int,
    Str,
    Err,
}
