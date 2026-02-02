use std::ops::Range;

use ac_ir::syntax::value::Symbol;

pub type Span = Range<u32>;
const DUMMY_SPAN: Span = 0..0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind<'db> {
    /* Expression-operator symbols */
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
    /// `&&`
    AndAnd,
    /// `||`
    OrOr,
    /// `!`
    Bang,
    /// `~`
    Tilde,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `&`
    And,
    /// `|`
    Or,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
    /// `+=`
    PlusEq,
    /// `-=`
    MinusEq,
    /// `*=`
    StarEq,
    /// `/=`
    SlashEq,
    /// `%=`
    PercentEq,
    /// `^=`
    CaretEq,
    /// `&=`
    AndEq,
    /// `|=`
    OrEq,
    /// `<<=`
    ShlEq,
    /// `>>=`
    ShrEq,

    /* Structural symbols */
    /// `@`
    At,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `...`
    DotDotDot,
    /// `..=`
    DotDotEq,
    /// `,`
    Comma,
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `::`
    PathSep,
    /// `->`
    RArrow,
    /// `<-`
    LArrow,
    /// `=>`
    FatArrow,
    /// `#`
    Hash,
    /// `$`
    Dollar,
    /// `?`
    Question,
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

    /// An identifier or keyword, e.g. `ident` or `prop`.
    Ident { sym: Symbol<'db> },

    /// A literal, e.g. `123` or `"hello"`.
    Literal { kind: LiteralKind, sym: Symbol<'db> },

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    Eof,
}

impl<'db> Token<'db> {
    pub const fn dummy() -> Self {
        Self {
            kind: TokenKind::Question,
            span: DUMMY_SPAN,
        }
    }

    pub fn glue(&self, joint: &Token) -> Option<Token<'_>> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa::Update)]
pub enum LiteralKind {
    Int,
    Str { terminated: bool },
}
