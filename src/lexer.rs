use std::fmt::{Display, Error};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct Location {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Keyword {
    Select,
    From,
    As,
    Table,
    Create,
    Insert,
    Into,
    Values,
    Int,
    Text,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Symbol {
    Semicolon,
    Asterisk,
    Comma,
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenValue<'value> {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(&'value str),
    String(&'value str),
    Number(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'token_value> {
    value: TokenValue<'token_value>,
    location: Location,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LexerResult<'token_value> {
    token_value: TokenValue<'token_value>,
    chars: usize,
    lines: u32,
    columns: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ParseError<'token_value> {
    location: Location,
    last: Option<Token<'token_value>>,
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hint = self.last.as_ref().map_or(String::default(), |token| {
            format!(" after {:?}", token.value)
        });
        write!(
            f,
            "Unable to lex token{}, at {}:{}",
            hint, self.location.line, self.location.column
        )
    }
}

impl std::error::Error for ParseError<'_> {}

type Lexer = fn(&str) -> Option<LexerResult>;

pub fn lex(source: &str) -> Result<Vec<Token>, ParseError> {
    const LEXERS: [Lexer; 0] = [];

    let mut tokens = Vec::<Token>::new();
    let mut location = Location::default();

    let char_indices = source
        .char_indices()
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let num_chars = char_indices.len();
    let mut char_index = 0;
    while char_index < num_chars {
        let slice = &source[char_index..];
        let Some(result) = LEXERS
            .iter()
            .find_map(|lexer| lexer(slice)) else {
        return Err(ParseError {
            location,
            last: tokens.pop(),
        })};

        let (token_value, chars, lines, columns) = (
            result.token_value,
            result.chars,
            result.lines,
            result.columns,
        );
        tokens.push(Token {
            value: token_value,
            location,
        });
        char_index += chars;
        location.line += lines;
        if lines > 0 {
            location.column = columns;
        } else {
            location.column += columns;
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let tokens = lex("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn nonsense() {
        let tokens = lex("deadbeef");
        assert!(tokens.is_err());
    }
}
