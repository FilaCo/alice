# Lexical elements

## Comments

Comments serve as program documentation. There are two forms:

- Line comments start with the character sequence `//` and stop at the end of the line.
- Block comments start with the character sequence `/*` and stop with the first subsequent character sequence `*/`.

A comment cannot start inside a rune or string literal, or inside a comment. A block comment containing no newlines acts like a space. Any other comment acts like a newline. 

:arrow_right: Next: [Constants](constants.md)

:blue_book: Back: [Table of Contents](table_of_contents.md)
