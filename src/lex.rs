pub struct Tokenizer {
    pos: Position,
}

#[derive(Debug)]
pub enum Token {
    Quote,
    Digit,
    Dot,
    Comma,
    Colon,
    RightCurly,
    LeftCurly,
    RightBracket,
    LeftBracket,
    Char,
    NewLine,
    Whitespace,
    NotSupported,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    line: i32,
    col: i32,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            pos: Position { line: 1, col: 1 },
        }
    }

    /// Map the characters in `file_contents` to JSON tokens
    pub fn tokenize(&mut self, file_contents: &str) -> Result<Vec<(Token, Position)>, String> {
        Ok(file_contents
            .chars()
            .map(|c| {
                self.next_char();
                let token: Token = match c {
                    '"' => Token::Quote,
                    ':' => Token::Colon,
                    '{' => Token::LeftCurly,
                    '}' => Token::RightCurly,
                    '[' => Token::LeftBracket,
                    ']' => Token::RightBracket,
                    ',' => Token::Comma,
                    '.' => Token::Dot,
                    ' ' | '\t' => Token::Whitespace,
                    '\n' => {
                        self.new_line();
                        Token::NewLine
                    }
                    '1'..='9' => Token::Digit,
                    'a'..='z' | 'A'..='Z' => Token::Char,
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
