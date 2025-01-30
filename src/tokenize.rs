use core::{fmt, str};
use std::fmt::Display;

pub struct Tokenizer {
    pos: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Quote,
    Digit(char),
    Dot,
    Comma,
    Colon,
    RightCurly,
    LeftCurly,
    RightBracket,
    LeftBracket,
    Char(char),
    NewLine,
    Whitespace,
    NotSupported,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match *self {
            Self::Quote => r#"""#,
            Self::Digit(d) => &format!("'{}'", d),
            Self::Dot => ".",
            Self::Comma => ",",
            Self::Colon => ":",
            Self::RightCurly => "{",
            Self::LeftCurly => "{",
            Self::RightBracket => "]",
            Self::LeftBracket => "[",
            Self::Char(c) => &format!("'{}'", c),
            Self::NewLine => "a newline",
            Self::Whitespace => "a whitespace",
            Self::NotSupported => unreachable!(),
        };

        write!(f, "{}", msg)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    line: i32,
    col: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {} column {}", self.line, self.col)
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            pos: Position { line: 1, col: 0 },
        }
    }

    /// Map the characters in `file_contents` to JSON tokens
    pub fn tokenize(&mut self, file_contents: &str) -> Result<Vec<(Token, Position)>, String> {
        // FIXME: why is this a result if it never fails
        let mut in_string = false;
        Ok(file_contents
            .chars()
            .map(|c| {
                self.next_char();
                let token: Token = match c {
                    '"' => { 
                        in_string = !in_string;
                        Token::Quote
                    },
                    ':' => Token::Colon,
                    '{' => Token::LeftCurly,
                    '}' => Token::RightCurly,
                    '[' => Token::LeftBracket,
                    ']' => Token::RightBracket,
                    ',' => Token::Comma,
                    '.' => Token::Dot,
                    ' ' | '\t' => {
                        if in_string {
                            Token::Char(c)
                        } else {
                            Token::Whitespace
                        }
                    }
                    '\n' => {
                        self.new_line();
                        Token::NewLine
                    }
                    '1'..='9' => Token::Digit(c),
                    'a'..='z' | 'A'..='Z' => Token::Char(c),
                    _ => Token::NotSupported,
                };

                (token, self.pos)
            })
            .collect())
    }

    fn new_line(&mut self) {
        self.pos.line += 1;
        self.pos.col = 1;
    }

    fn next_char(&mut self) {
        self.pos.col += 1;
    }
}
