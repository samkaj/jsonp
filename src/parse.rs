use crate::tokenize::{Position, Token};

pub enum JsonValue {
    Object(Option<Box<Json>>),
    Float(f64),
    Int(i64),
    Str(String),
    Bool(bool),
    Arr(Vec<JsonValue>),
}

pub struct Json {
    key: String,
    value: Box<Json>,
}

pub struct Parser {
    tokens: Vec<(Token, Position)>,
    idx: usize,
    json: Vec<Json>,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Position)>) -> Self {
        Parser {
            tokens,
            idx: 0,
            json: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        self.remove_whitespace();
        self.parse_object()?;

        Err("".to_string())
    }

    fn remove_whitespace(&mut self) {
        self.tokens = self
            .tokens
            .clone()
            .into_iter()
            .filter(|x| x.0 != Token::Whitespace && x.0 != Token::NewLine)
            .collect();
    }

    fn parse_object(&mut self) -> Result<Json, String> {
        self.assert_current(&[Token::LeftCurly])?;

        self.next_token()?;
        self.assert_current(&[Token::Quote, Token::RightCurly])?;

        unimplemented!("parse object")
    }

    /// Assert that the current token is one of the expected ones
    fn assert_current(&self, expected: &[Token]) -> Result<(), String> {
        let curr = self.current_token()?;

        if expected.contains(&curr.0) {
            return Ok(());
        }

        let expected_list = expected
            .iter()
            .map(Token::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        Err(format!(
            "Expected {} but got {} at {}",
            expected_list, curr.0, curr.1
        ))
    }
    /// Consume whitespaces and newlines until a new token is reached
    fn next_token(&mut self) -> Result<(), String> {
        self.idx += 1;
        if self.end_of_tokens() {
            Err("no next token".to_string())
        } else {
            Ok(())
        }
    }

    /// Get the current token if it exists
    fn current_token(&self) -> Result<(Token, Position), String> {
        if self.end_of_tokens() {
            Err("unexpected end of file".to_string())
        } else {
            Ok(self.tokens[self.idx])
        }
    }

    /// Get the previous token if it exists
    fn prev_token(&self) -> Result<(Token, Position), String> {
        if self.idx == 0 {
            Err("no previous token".to_string())
        } else {
            Ok(self.tokens[self.idx - 1])
        }
    }

    /// Peek at the next token if it exists
    fn peek(&self) -> Result<(Token, Position), String> {
        if self.idx == self.tokens.len() - 1 {
            Err("no next token".to_string())
        } else {
            Ok(self.tokens[self.idx + 1])
        }
    }

    fn end_of_tokens(&self) -> bool {
        self.tokens.len() <= self.idx
    }
}
