# Orion's Lexer
This is the folder where the [lexer](https://en.wikipedia.org/wiki/Lexical_analysis)
for Orion. Where I can 

## Tokens
Each token is stored with it's kind, and position.

```rust
pub struct Token {
    kind: TokenKind,
    span: Span
}
```

Where `TokenKind` is an enum of all of the kinds of tokens we can have.

When creating a token, you will need 
```rust
impl Token {
    fn new(kind: TokenKind, span: Span) -> Self {
        Self {
            kind,
            span
        }
    }
}
```

## Spans
A `Span` type takes note of the it's **file**, **span**, where span holds the 
start line and column and ending line and column of a region. This region might 
be for singular tokens, expressions, or functions.

```rust
pub struct Position {
    file: PathBuf,
    span: Span,
}

type Coordinate = (usize, usize);

pub struct Span {
    start: Coordinate,
    end: Coordinate,
}
```

## The Lexer
The Lexer should be able to take in a single file, and operate on that single 
file. The result of the lexer should be a 
[VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) of 
Tokens. 

### General Notes
Came across something called the [lexer hack](https://en.wikipedia.org/wiki/Lexer_hack),
which is the problem where given following snippet of code:
```
(A)*B
```
the lexical class of A cannot be determined without further contextual information.

This could either be interpreted as `A * B`, simply the multiplication of `A`
and `B`, or alternatively, it could be interpreted as `(A) (*B)`, which would 
be casting the dereferenced value of `B` to the type `A`.

The fixes for this is called [the hack solution](https://en.wikipedia.org/wiki/Lexer_hack#The_hack_solution)
in which information from the semantic symbol table is fed back into the Lexer.
Not exactly sure how I would implement that, but is an interesting solution.

An interesting rabbit hole of lexical analysis issues happen when reading this
article and following into [dangling else's](https://en.wikipedia.org/wiki/Dangling_else)
and [Most vexing parse](https://en.wikipedia.org/wiki/Most_vexing_parse).
