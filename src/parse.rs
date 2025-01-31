use crate::tokenize::{Position, Token};

#[derive(Clone, Debug)]
pub enum JsonValue {
    Object(Option<Box<Json>>),
    Float(f64),
    Int(i64),
    Str(String),
    Bool(bool),
    Arr(Vec<JsonValue>),
}

#[derive(Clone, Debug)]
pub struct Json {
    key: String,
    value: Box<JsonValue>,
}

impl Json {
    pub fn new(key: String, value: Box<JsonValue>) -> Self {
        Self { key, value }
    }
}

#[derive(Clone, Debug)]
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

    pub fn parse(&mut self) -> Result<Vec<Json>, String> {
        self.remove_whitespace();
        self.assert_current(&[Token::LeftCurly])?;

        while !self.end_of_tokens() {
            if self.last_token() {
                self.assert_current(&[Token::RightCurly])?;
                return Ok(self.json.clone());
            }

            self.next_token()?;
            self.assert_current(&[Token::Quote, Token::RightCurly])?;

            // Empty object
            if self.current_token()?.0 == Token::RightCurly {
                return Ok(self.json.clone());
            } else {
                match self.parse_object() {
                    Ok(obj) => {
                        self.json.push(obj);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }

        unreachable!("well-formed json has returned, erroneous has crashed");
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
        let key = self.parse_key()?;
        self.assert_current(&[Token::Colon])?;
        self.next_token()?;

        let current = self.current_token()?;
        self.next_token()?;
        let value = match current.0 {
            Token::Quote => JsonValue::Str(self.parse_string_literal()?),
            _ => unimplemented!("token not implemented {}", current.0),
        };

        Ok(Json::new(key, Box::new(value)))
    }

    fn parse_string_literal(&mut self) -> Result<String, String> {
        let mut key = String::new();

        while let Some((token, _)) = self.tokens.get(self.idx) {
            match token {
                Token::Char(c) => {
                    key.push(*c);
                    self.idx += 1;
                }
                Token::Quote => break,
                _ => {
                    return Err(format!(
                        "TODO better error: Unexpected token {:?} in key",
                        token
                    ))
                }
            }
        }

        self.next_token()?;

        Ok(key)
    }

    fn parse_key(&mut self) -> Result<String, String> {
        self.assert_current(&[Token::Quote])?;
        self.next_token()?;

        self.parse_string_literal()
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

    fn last_token(&self) -> bool {
        self.tokens.len() - 1 == self.idx
    }
}
