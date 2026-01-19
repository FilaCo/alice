// use std::io::Read;

// use logos::{Lexer, Logos};

// use crate::{
//     db::AcDbTrait,
//     ir::{
//         ast::Ast,
//         source::{SourceFile, SourceFileId},
//         token::Token,
//     },
// };

// #[salsa::tracked]
// pub fn parse_file<'db>(db: &'db dyn AcDbTrait, src: SourceFile) -> Ast<'db> {
//     let handle = src.handle(db);
//     let contents = std::str::from_utf8(&handle).expect("invalid file encoding");
//     let mut parser = Parser::new(db, Token::lexer(contents));

//     todo!()
// }

// struct Parser<'db> {
//     db: &'db dyn AcDbTrait,
//     lexer: Lexer<'db, Token<'db>>,
//     token: Token<'db>,
//     prev: Token<'db>,
//     num_bump_calls: u32,
// }

// impl<'db> Parser<'db> {
//     fn new(db: &'db dyn AcDbTrait, lexer: Lexer<'db, Token<'db>>) -> Self {
//         let mut parser = Self {
//             db,
//             lexer,
//             token: Token::Unknown,
//             prev: Token::Unknown,
//             num_bump_calls: 0,
//         };

//         parser.bump();
//         parser.num_bump_calls -= 1;

//         parser
//     }

//     fn bump(&mut self) {
//         let lexeme = self.lexer.next();
//     }
// }
