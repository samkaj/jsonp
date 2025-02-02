use crate::tokenize::{Position, Token};

#[derive(Clone, Debug)]
pub enum JsonValue {
    Object(Vec<JsonValue>),
    KeyedObject(String, Box<JsonValue>),
    Float(f64),
    Int(i64),
    Str(String),
    Bool(bool),
    Arr(Vec<JsonValue>),
    Empty,
}

#[derive(Clone, Debug)]
pub struct Parser {
    tokens: Vec<(Token, Position)>,
    idx: usize,
    json: Vec<JsonValue>,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Position)>) -> Self {
        Parser {
            tokens,
            idx: 0,
            json: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<Vec<JsonValue>, String> {
        self.remove_whitespace();
        self.assert_current(&[Token::LeftCurly])?;
        self.next_token()?;

        self.parse_json()
    }

    fn remove_whitespace(&mut self) {
        self.tokens = self
            .tokens
            .clone()
            .into_iter()
            .filter(|(x, _)| *x != Token::Whitespace && *x != Token::NewLine)
            .collect();
    }

    fn parse_json(&mut self) -> Result<Vec<JsonValue>, String> {
        while !self.end_of_tokens() {
            if self.last_token() {
                break;
            }

            // Always expect a key.
            let key = self.parse_key()?;
            self.assert_current(&[Token::Colon])?;
            self.next_token()?;

            let (next, _) = self.current_token()?;

            // Next is always an object, number, string, boolean, or array
            let json = match next {
                Token::LeftCurly => self.parse_object(),
                _ => unimplemented!(),
            };

            dbg!(&json);
            self.json.push(JsonValue::KeyedObject(key, Box::new(json?)));
        }

        Ok(self.json.clone())
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        // Assume {
        self.assert_current(&[Token::LeftCurly])?;
        self.next_token()?;

        // Expect a key or an empty object
        self.assert_current(&[Token::RightCurly, Token::Quote])?;
        let (next, pos) = self.current_token()?;
        let json = match next {
            Token::RightCurly => Ok(JsonValue::Empty),
            Token::Quote => self.parse_keyed_object(),
            _ => Err(format!("unexpected token `{next}` at {}", pos)),
        };

        self.next_token()?;
        json
    }

    fn parse_keyed_object(&mut self) -> Result<JsonValue, String> {
        let key = self.parse_key()?;
        self.assert_current(&[Token::Colon])?;
        self.next_token()?;
        let (next, _) = self.current_token()?;
        let json = match next {
            Token::LeftCurly => self.parse_object(),
            _ => unimplemented!(),
        };

        Ok(JsonValue::KeyedObject(key, Box::new(json?)))
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        unimplemented!("numbers")
    }

    fn parse_string_literal(&mut self) -> Result<JsonValue, String> {
        unimplemented!("string literal")
    }

    /// Parse a key (property name)
    /// Consumes: `"key" :`, leaves next token as e.g., `{`
    fn parse_key(&mut self) -> Result<String, String> {
        (self.assert_current(&[Token::Quote])?);
        self.next_token()?;

        let key = self
            .tokens
            .iter()
            .skip(self.idx)
            .map_while(|(t, _)| match t {
                Token::Char(c) => {
                    self.idx += 1;
                    Some(c)
                }
                _ => None,
            })
            .collect::<String>();

        (self.assert_current(&[Token::Quote])?);
        self.next_token()?;

        Ok(key)
    }

    /// Assert that the current token is one of the expected ones
    fn assert_current(&self, expected: &[Token]) -> Result<(), String> {
        let curr = self.current_token()?;

        for ex in expected {
            let mat = match (ex, curr.0) {
                (Token::Char(_), Token::Char(_)) | (Token::Digit(_), Token::Digit(_)) => true,
                (a, b) => *a == b,
            };

            if mat {
                return Ok(());
            }
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
        if self.end_of_tokens() {
            Err("reached end of tokens".to_string())
        } else {
            self.idx += 1;
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
