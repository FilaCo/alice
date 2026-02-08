# Alice Language Specification <!-- omit from toc -->

- [Introduction](#introduction)
- [1. Syntax and grammar](#1-syntax-and-grammar)
  - [1.1 Notation](#11-notation)
  - [1.2 Lexical grammar](#12-lexical-grammar)
    - [1.2.1 Whitespace and comments](#121-whitespace-and-comments)
    - [1.2.2 Keywords and operators](#122-keywords-and-operators)
    - [1.2.3 Literals](#123-literals)
    - [1.2.4 Identifiers](#124-identifiers)
    - [1.2.5 Tokens](#125-tokens)
  - [1.3 Syntax grammar](#13-syntax-grammar)

## Introduction

## 1. Syntax and grammar

### 1.1 Notation

TBD

### 1.2 Lexical grammar

#### 1.2.1 Whitespace and comments

```ebnf
LF           = /* the Unicode code point Line Feed U+000A */
CR           = /* the Unicode code point Carriage Return U+000D */
NL           = LF | (CR [LF])
BlockComment = "/*" { BlockComment | <any character> } "*/"
LineComment  = "//" { <any character except NL> }
WS           = <one of the following characters: SPACE U+0020, TAB U+0009, Form Feed U+000C>
Hidden       = BlockComment | LineComment | WS
```

#### 1.2.2 Keywords and operators

```ebnf
Eq       = "="
Lt       = "<"
Gt       = ">"
Excl     = "!"
Plus     = "+"
Minus    = "-"
Star     = "*"
Slash    = "/"
Dot      = "."
Comma    = ","
Semi     = ";"
Colon    = ":"
Hash     = "#"
Quest    = "?"
LParen   = "("
RParen   = ")"
LBrace   = "{"
RBrace   = "}"
LBracket = "["
RBracket = "]"

EqEq       = "=="
Ne         = "!="
Le         = "<="
Ge         = ">="
ColonColon = "::"

Id        = "id"
Prop      = "prop"
Query     = "query"
Include   = "include"
Namespace = "namespace"
Const     = "const"
And       = "and"
Or        = "or"
Using     = "using"
Not       = "not"
```

#### 1.2.3 Literals

TBD

#### 1.2.4 Identifiers

TBD

#### 1.2.5 Tokens

These are all the valid tokens in one rule. Note that syntax grammar ignores tokens [BlockComment](#121-whitespace-and-comments), [LineComment](#121-whitespace-and-comments) and [Whitespace](#121-whitespace-and-comments).

```ebnf
AliceToken = Whitespace   |
             BlockComment |
             LineComment  |
             NewLine      |
             Eq           |
             Lt           |
             Gt           |
             Excl         |
             Plus         |
             Minus        |
             Star         |
             Slash        |
             Dot          |
             Comma        |
             Semi         |
             Colon        |
             Hash         |
             Quest        |
             LParen       |
             RParen       |
             LBrace       |
             RBrace       |
             LBracket     |
             RBracket     |
             EqEq         |
             Ne           |
             Le           |
             Ge           |
             ColonColon   |
             Id           |
             Prop         |
             Query        |
             Include      |
             Namespace    |
             Const        |
             And          |
             Or           |
             Using        |
             Ident
Eof        = <end of input>
```

### 1.3 Syntax grammar

The grammar below replaces some lexical grammar rules with explicit literals (where such replacement in trivial and always correct, for example, for keywords) for better readability.

```ebnf
alice_file = { top_level_object } Eof

top_level_object = (top_level_stmt | top_level_decl) semis

top_level_stmt   = using_namespace | query_stmt
top_level_decl   = namespace_decl | prop_decl

using_namespace  = "using" "namespace" ident_path

query_stmt = "query" prop_list 
prop_list = ident_path { "," ident_path }

namespace_decl = "namespace" ident namespace_body
namespace_body = "{" { top_level_object } "}"

prop_decl  = "prop" ident [prop_body]
prop_body  = "{" field_decl { semis field_decl } "}"
field_decl = Ident ":" ident_path

ident_path = ["::"] Ident { "::" Ident }
semis      = ";" | NL { ";" | NL }
```
