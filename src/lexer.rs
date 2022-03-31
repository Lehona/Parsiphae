use anyhow::{bail, ensure, Result};
use std::collections::HashMap;

type ParserResult = Result<(TokenKind, usize)>;
type Parser = fn(&[u8]) -> ParserResult;

fn printable(data: &[u8]) -> String {
    String::from_utf8_lossy(data).to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Comment
    Comment(String),

    // Values
    Identifier(Vec<u8>),
    Integer(i64),
    Decimal(f64),
    StringLit(Vec<u8>),

    // Punctuation
    BracketOpen,  // {
    BracketClose, // }
    SquareOpen,   // [
    SquareClose,  // ]
    ParenOpen,    // (
    ParenClose,   // )
    Comma,        // ,
    Period,       // .
    Semicolon,    // ;

    // Operators
    Plus,           // +
    PlusAssign,     // +=
    Minus,          // -
    MinusAssign,    // -=
    Multiply,       // *
    MultiplyAssign, // *=
    Divide,         // /
    DivideAssign,   // /=
    Assign,         // =
    Equals,         // ==
    NotEquals,      // !=
    Greater,        // >
    GreaterEquals,  // >=
    Lower,          // <
    LowerEquals,    // <=
    Modulo,         // %
    Or,             // ||
    BitOr,          // |
    And,            // &&
    BitAnd,         // &
    ShiftLeft,      // <<
    ShiftRight,     // >>
    Not,            // !
    BitNot,         // ~

    // Keywords
    Func,
    Class,
    Prototype,
    Instance,
    Var,
    Const,
    Return,
    Else,
    If,

    // End of File
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: (usize, usize), // (Start, End)
}

impl Token {
    pub fn stringified(&self) -> String {
        match &self.kind {
            TokenKind::Comment(c) => format!("// {}", c.replace("\n", "\\n")),
            TokenKind::Identifier(id) => format!("{}", String::from_utf8_lossy(&id).to_uppercase()),
            TokenKind::StringLit(s) => format!("\"{}\"", String::from_utf8_lossy(&s)),
            _ => format!("{:?}", self.kind).to_uppercase(),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({} - {})", self.kind, self.span.0, self.span.1)
    }
}

fn skip_whitespace(data: &[u8]) -> usize {
    const WHITESPACE: &'static [u8] = b" \t\n\r";

    data.iter().take_while(|c| WHITESPACE.contains(*c)).count()
}

fn get_collisions() -> HashMap<u8, Vec<Parser>> {
    let mut hm: HashMap<u8, Vec<Parser>> = HashMap::new();

    // Division and Comments
    hm.insert(
        b'/',
        vec![
            |data| tokenize_literal(data, b"/=", TokenKind::DivideAssign, false),
            |data| tokenize_linecomment(data),
            |data| tokenize_multilinecomment(data),
            |data| tokenize_literal(data, b"/", TokenKind::Divide, false),
        ],
    );

    // Operators except Divide
    hm.insert(
        b'+',
        vec![
            |data| tokenize_literal(data, b"+=", TokenKind::PlusAssign, false),
            |data| tokenize_literal(data, b"+", TokenKind::Plus, false),
        ],
    );
    hm.insert(
        b'-',
        vec![
            |data| tokenize_literal(data, b"-=", TokenKind::MinusAssign, false),
            |data| tokenize_literal(data, b"-", TokenKind::Minus, false),
        ],
    );
    hm.insert(
        b'*',
        vec![
            |data| tokenize_literal(data, b"*=", TokenKind::MultiplyAssign, false),
            |data| tokenize_literal(data, b"*", TokenKind::Multiply, false),
        ],
    );
    hm.insert(
        b'=',
        vec![
            |data| tokenize_literal(data, b"==", TokenKind::Equals, false),
            |data| tokenize_literal(data, b"=", TokenKind::Assign, false),
        ],
    );
    hm.insert(
        b'!',
        vec![
            |data| tokenize_literal(data, b"!=", TokenKind::NotEquals, false),
            |data| tokenize_literal(data, b"!", TokenKind::Not, false),
        ],
    );
    hm.insert(
        b'&',
        vec![
            |data| tokenize_literal(data, b"&&", TokenKind::And, false),
            |data| tokenize_literal(data, b"&", TokenKind::BitAnd, false),
        ],
    );
    hm.insert(
        b'>',
        vec![
            |data| tokenize_literal(data, b">=", TokenKind::GreaterEquals, false),
            |data| tokenize_literal(data, b">>", TokenKind::ShiftRight, false),
            |data| tokenize_literal(data, b">", TokenKind::Greater, false),
        ],
    );
    hm.insert(
        b'<',
        vec![
            |data| tokenize_literal(data, b"<=", TokenKind::LowerEquals, false),
            |data| tokenize_literal(data, b"<<", TokenKind::ShiftLeft, false),
            |data| tokenize_literal(data, b"<", TokenKind::Lower, false),
        ],
    );
    hm.insert(
        b'|',
        vec![
            |data| tokenize_literal(data, b"||", TokenKind::Or, false),
            |data| tokenize_literal(data, b"|", TokenKind::BitOr, false),
        ],
    );

    hm
}

fn tokenize_linecomment(data: &[u8]) -> ParserResult {
    ensure!(!data.is_empty(), "Input is empty!");
    ensure!(data.len() > 1, "Input is not long enough.");
    ensure!(&data[..2] == b"//", "Comment does not start with //");

    const COMMENT_END: &'static [u8] = b"\r\n";

    let comment_length = data[2..]
        .iter()
        .take_while(|c| !COMMENT_END.contains(c))
        .count();

    Ok((
        TokenKind::Comment(printable(&data[2..2 + comment_length])),
        2 + comment_length,
    ))
}

fn tokenize_multilinecomment(data: &[u8]) -> ParserResult {
    ensure!(!data.is_empty(), "Input is empty!");
    ensure!(data.len() > 3, "Input is not long enough.");
    ensure!(&data[..2] == b"/*", "Comment does not start with /*");

    const MULTILINE_COMMENT_END: &'static [u8] = b"*/";

    // TODO FIX THIS FN
    let comment_length = data[2..]
        .windows(2)
        .take_while(|w| w != &MULTILINE_COMMENT_END)
        .count();

    Ok((
        TokenKind::Comment(printable(&data[2..2 + comment_length])),
        4 + comment_length,
    ))
}

fn tokenize_identifier(data: &[u8]) -> ParserResult {
    const IDENTIFIER_BEGIN: &'static [u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_1234567890";
    const IDENTIFIER_END: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_^@1234567890\xC4\xE4\xD6\xF6\xFC\xDC\xDF";

    ensure!(!data.is_empty(), "Input is empty!");
    let first = data[0];

    ensure!(
        IDENTIFIER_BEGIN.contains(&first),
        "Encountered illegal character: 0x{:x}",
        first
    );

    let tail = data[1..].iter().take_while(|b| IDENTIFIER_END.contains(b));

    let mut name = vec![first];
    name.extend(tail);

    let length = name.len();

    let keywords: Vec<(&'static [u8], TokenKind)> = vec![
        (b"Func", TokenKind::Func),
        (b"Class", TokenKind::Class),
        (b"Prototype", TokenKind::Prototype),
        (b"Instance", TokenKind::Instance),
        (b"Var", TokenKind::Var),
        (b"Const", TokenKind::Const),
        (b"Return", TokenKind::Return),
        (b"Else", TokenKind::Else),
        (b"If", TokenKind::If),
    ];

    for (kw, t) in keywords {
        if kw.eq_ignore_ascii_case(&name) {
            return Ok((t, name.len()));
        }
    }

    Ok((TokenKind::Identifier(name), length))
}

fn tokenize_literal(data: &[u8], literal: &[u8], t: TokenKind, ignore_case: bool) -> ParserResult {
    ensure!(!data.is_empty(), "Input is empty!");
    ensure!(data.len() >= literal.len(), "Input is not long enough.");

    let is_equal = if ignore_case {
        <[u8]>::eq_ignore_ascii_case
    } else {
        (|a: &[u8], b: &[u8]| a == b) as fn(&[u8], &[u8]) -> bool
    };

    if is_equal(&data[..literal.len()], literal) {
        Ok((t, literal.len()))
    } else {
        bail!(
            "Unable to match literal. Expected {} but found {}",
            printable(literal),
            printable(&data[..literal.len()])
        )
    }
}

fn tokenize_number(data: &[u8]) -> ParserResult {
    // Parse as many digits as possible
    let mut has_dot = false;
    let parsed: Vec<u8> = data
        .iter()
        .take_while(|c| {
            if c.is_ascii_digit() {
                true
            } else if **c == b'.' && !has_dot {
                has_dot = true;
                true
            } else {
                false
            }
        })
        .copied()
        .collect();

    ensure!(
        parsed.iter().any(|b| b.is_ascii_digit()),
        "Expected at least one digit."
    );
    let parsed =
        String::from_utf8(parsed).expect("Guaranteed to be valid utf8 due to parsing above.");

    let token = if has_dot {
        TokenKind::Decimal(parsed.parse::<f64>()?)
    } else {
        TokenKind::Integer(parsed.parse::<i64>()?)
    };

    Ok((token, parsed.len()))
}

fn tokenize_one_of(data: &[u8], parsers: &[Parser]) -> ParserResult {
    for parser in parsers {
        match parser(data) {
            Err(_) => {
                continue;
            }
            Ok(r) => return Ok(r),
        }
    }

    bail!("No parsers matched!")
}

fn tokenize_string_literal(data: &[u8]) -> ParserResult {
    ensure!(!data.is_empty(), "Input is empty!");
    ensure!(data.len() > 1, "Input is not long enough.");

    ensure!(
        data[0] == b'"',
        "Found illegal character: 0x{:x} (Expected \")",
        data[0]
    );

    let lit: Vec<u8> = data[1..]
        .iter()
        .take_while(|b| **b != b'"')
        .copied()
        .collect();
    ensure!(
        data[1..].len() > lit.len(),
        "String literal did not end with \" (EOF)"
    );

    let length = lit.len() + 2; // Literal length + Quotation marks
    Ok((TokenKind::StringLit(lit), length))
}

fn extract_token(data: &[u8]) -> ParserResult {
    // Length of token
    let next = match data.iter().next() {
        Some(c) => c,
        None => bail!("Empty Data"),
    };

    let collisions = get_collisions();

    let token = match next {
        b'{' => (TokenKind::BracketOpen, 1),
        b'}' => (TokenKind::BracketClose, 1),
        b'[' => (TokenKind::SquareOpen, 1),
        b']' => (TokenKind::SquareClose, 1),
        b'(' => (TokenKind::ParenOpen, 1),
        b')' => (TokenKind::ParenClose, 1),
        b',' => (TokenKind::Comma, 1),
        b'.' => (TokenKind::Period, 1),
        b';' => (TokenKind::Semicolon, 1),
        b'%' => (TokenKind::Modulo, 1),
        b'~' => (TokenKind::BitNot, 1),
        b'0'..=b'9' => tokenize_number(data)?,
        b'+' | b'-' | b'*' | b'/' | b'=' | b'!' | b'>' | b'<' | b'|' | b'&' => {
            tokenize_one_of(data, &collisions[next])?
        }
        b'"' => tokenize_string_literal(data)?,
        _ => tokenize_identifier(data)?,
    };

    Ok(token)
}

pub fn lex(mut input: &[u8]) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut offset = 0;

    loop {
        let ws = skip_whitespace(input);
        offset += ws;
        input = &input[ws..];

        if input.is_empty() {
            break;
        }

        let (kind, length) = extract_token(input)?;
        let token = Token {
            kind,
            span: (offset, offset + length),
        };
        tokens.push(token);

        input = &input[std::cmp::min(length, input.len())..];

        offset += length;
    }

    tokens.push(Token {
        kind: TokenKind::EOF,
        span: (offset, offset),
    });
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use TokenKind::*;

    #[test]
    fn no_ws() {
        let input = b"1+2=3";
        let lexed = lex(input).unwrap();

        let expected = vec![
            Token {
                kind: Integer(1),
                span: (0, 1),
            },
            Token {
                kind: Plus,
                span: (1, 2),
            },
            Token {
                kind: Integer(2),
                span: (2, 3),
            },
            Token {
                kind: Assign,
                span: (3, 4),
            },
            Token {
                kind: Integer(3),
                span: (4, 5),
            },
            Token {
                kind: EOF,
                span: (5, 5),
            },
        ];

        assert_eq!(expected, lexed);
    }

    #[test]
    fn with_ws() {
        let input = b"1 + 2 = 3";
        let lexed = lex(input).unwrap();

        let expected = vec![
            Token {
                kind: Integer(1),
                span: (0, 1),
            },
            Token {
                kind: Plus,
                span: (2, 3),
            },
            Token {
                kind: Integer(2),
                span: (4, 5),
            },
            Token {
                kind: Assign,
                span: (6, 7),
            },
            Token {
                kind: Integer(3),
                span: (8, 9),
            },
            Token {
                kind: EOF,
                span: (9, 9),
            },
        ];

        assert_eq!(expected, lexed);
    }

    #[test]
    fn with_newline() {
        let input = b"foo + 2\r\n= 3";
        let lexed = lex(input).unwrap();

        let expected = vec![
            Token {
                kind: Identifier(b"foo".to_vec()),
                span: (0, 3),
            },
            Token {
                kind: Plus,
                span: (4, 5),
            },
            Token {
                kind: Integer(2),
                span: (6, 7),
            },
            Token {
                kind: Assign,
                span: (9, 10),
            },
            Token {
                kind: Integer(3),
                span: (11, 12),
            },
            Token {
                kind: EOF,
                span: (12, 12),
            },
        ];

        assert_eq!(expected, lexed);
    }
}
