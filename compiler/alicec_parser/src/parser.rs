use alicec_db::prelude::AlicecDbTrait;
use alicec_lexer::prelude::Lexer;

pub struct Parser<'db> {
    db: &'db dyn AlicecDbTrait,
    lexer: Lexer<'db>,
}

impl<'db> Parser<'db> {
    pub fn new(db: &'db dyn AlicecDbTrait, src: &'db str) -> Self {
        let lexer = Lexer::new(db, src);
        Self { db, lexer }
    }

    /// ```ebnf
    /// expr = term;
    /// ```
    fn parse_expr(&mut self) {
        self.parse_term();
    }

    ///```ebnf
    /// term = factor { ("Minus" | "Plus") factor };
    /// ```
    fn parse_term(&mut self) {}

    ///```ebnf
    /// factor = unary { ("Slash" | "Star") unary };
    /// ```
    fn parse_factor(&mut self) {}

    /// ```ebnf
    /// unary = "Minus" unary | primary;
    /// ```
    fn parse_unary(&mut self) {}

    /// ```ebnf
    /// primary = "Lit" | "LParen" expr "RParen";
    /// ```
    fn parse_primary(&mut self) {}
}
