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

pub struct SyntaxError(pub String);

#[derive(Clone, Debug)]
pub struct Parser {
    tokens: Vec<(Token, Position)>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Position)>) -> Self {
        Parser { tokens, idx: 0 }
    }

    /// Parse a JSON document
    pub fn parse(&mut self) -> Result<JsonValue, SyntaxError> {
        self.remove_whitespace();
        let (first_token, _) = self.current_token()?;
        match first_token {
            Token::LeftBracket => self.parse_array(),
            Token::LeftCurly => self.parse_object(),
            _ => Err(self.err("invalid JSON document")),
        }
    }

    /// Parse a literal object
    fn parse_object(&mut self) -> Result<JsonValue, SyntaxError> {
        // Assume {
        self.assert_current(&[Token::LeftCurly])?;
        self.next_token()?;

        if self.assert_current(&[Token::RightCurly]).is_ok() {
            self.next_token()?;
            return Ok(JsonValue::Empty);
        }

        let mut objs: Vec<JsonValue> = vec![];
        while self
            .assert_current(&[Token::RightCurly, Token::RightBracket])
            .is_err()
        {
            // Expect a key or an empty object
            self.assert_current(&[Token::Quote, Token::Comma])?;
            let (next, _) = self.current_token()?;
            let json = match next {
                Token::Quote => self.parse_keyed_object(),
                Token::Comma => break,
                _ => Err(self.err("unterminated object")),
            };

            if !self.last_token() {
                self.next_token()?;
            }

            objs.push(json?);
        }

        if objs.is_empty() {
            Ok(JsonValue::Empty)
        } else {
            Ok(JsonValue::Object(objs))
        }
    }

    /// Parse a keyed object
    /// e.g., "key": {}
    fn parse_keyed_object(&mut self) -> Result<JsonValue, SyntaxError> {
        let key = self.parse_key()?;
        self.assert_current(&[Token::Colon])?;
        self.next_token()?;
        let (next, _) = self.current_token()?;
        let json = match next {
            Token::LeftCurly => self.parse_object(),
            Token::Quote => self.parse_string_literal(),
            Token::Char('t') | Token::Char('f') => self.parse_bool(),
            Token::Digit(_) | Token::Minus => self.parse_number(),
            Token::LeftBracket => self.parse_array(),
            _ => Err(self.err("unexpected token while parsing object")),
        };

        Ok(JsonValue::KeyedObject(key, Box::new(json?)))
    }

    /// Parse an array of json values
    fn parse_array(&mut self) -> Result<JsonValue, SyntaxError> {
        let mut arr: Vec<JsonValue> = vec![];
        while self.current_token()?.0 != Token::RightBracket {
            self.next_token()?;
            let (next, _) = self.current_token()?;
            let json = match next {
                Token::LeftCurly => self.parse_object(),
                Token::Quote => self.parse_string_literal(),
                Token::Char('t') | Token::Char('f') => self.parse_bool(),
                Token::Digit(_) | Token::Minus => self.parse_number(),
                Token::LeftBracket => self.parse_array(),
                Token::RightBracket => break,
                _ => Err(self.err("unexpected token while parsing array")),
            };

            arr.push(json?);
        }

        self.next_token()?;
        Ok(JsonValue::Arr(arr))
    }

    /// Parse a number, resulting in either a float or an integer
    fn parse_number(&mut self) -> Result<JsonValue, SyntaxError> {
        let num = self.digits_to_string();
        if num.contains('.') {
            match num.parse::<f64>() {
                Ok(f) => Ok(JsonValue::Float(f)),
                Err(_) => Err(self.err("failed to parse float")),
            }
        } else {
            match num.parse::<i64>() {
                Ok(i) => Ok(JsonValue::Int(i)),
                Err(_) => Err(self.err("failed to parse integer")),
            }
        }
    }

    /// Parse a string literal
    /// e.g., "foo": "bar"
    fn parse_string_literal(&mut self) -> Result<JsonValue, SyntaxError> {
        self.assert_current(&[Token::Quote])?;
        self.next_token()?;

        let str = self.chars_to_string();

        self.assert_current(&[Token::Quote])?;
        self.next_token()?;

        Ok(JsonValue::Str(str))
    }

    /// Parse a bool
    /// e.g. "field": true
    fn parse_bool(&mut self) -> Result<JsonValue, SyntaxError> {
        let str = self.chars_to_string();
        if str == "true" {
            Ok(JsonValue::Bool(true))
        } else if str == "false" {
            Ok(JsonValue::Bool(false))
        } else {
            Err(self.err("failed to parse boolean"))
        }
    }

    /// Parse a key (property name)
    /// Consumes: `"key" :`, leaves next token as e.g., `{`
    fn parse_key(&mut self) -> Result<String, SyntaxError> {
        if self.assert_current(&[Token::Comma]).is_ok() {
            self.next_token()?;
        }
        self.assert_current(&[Token::Quote])?;
        self.next_token()?;

        let key = self.chars_to_string();

        self.assert_current(&[Token::Quote])?;
        self.next_token()?;

        Ok(key)
    }

    /// Assert that the current token is one of the expected ones
    fn assert_current(&self, expected: &[Token]) -> Result<(), SyntaxError> {
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

        Err(self.err(format!("expected {} but got {}", expected_list, curr.0).as_str()))
    }

    /// Consumes char tokens from the current position.
    /// Important: no assertions made here
    fn chars_to_string(&mut self) -> String {
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
        key
    }

    /// Convert the expected incoming characters to a string representing a digit
    fn digits_to_string(&mut self) -> String {
        let digits = self
            .tokens
            .iter()
            .skip(self.idx)
            .map_while(|(t, _)| match t {
                Token::Digit(c) => {
                    self.idx += 1;
                    Some(c)
                }
                Token::Minus => {
                    self.idx += 1;
                    Some(&'-')
                }
                Token::Dot => {
                    self.idx += 1;
                    Some(&'.')
                }
                _ => None,
            })
            .collect::<String>();
        digits
    }

    /// Trim all of the whitespace since the parser does not care for it
    fn remove_whitespace(&mut self) {
        self.tokens = self
            .tokens
            .clone()
            .into_iter()
            .filter(|(x, _)| *x != Token::Whitespace && *x != Token::NewLine)
            .collect();
    }

    /// Consume the next token if it exists
    fn next_token(&mut self) -> Result<(), SyntaxError> {
        if self.end_of_tokens() {
            Err(self.err("unterminated"))
        } else {
            self.idx += 1;
            Ok(())
        }
    }

    /// Get the current token if it exists
    fn current_token(&self) -> Result<(Token, Position), SyntaxError> {
        if self.end_of_tokens() {
            Err(self.err("unexpected end of file"))
        } else {
            Ok(self.tokens[self.idx])
        }
    }

    fn err(&self, msg: &str) -> SyntaxError {
        if self.end_of_tokens() {
            // A bit ugly, but allows current_token to crash
            SyntaxError("Syntax error: unexpected end of file".to_string())
        } else {
            let (_, pos) = self.tokens[self.idx];
            SyntaxError(format!("Syntax error: {} at {}", msg, pos))
        }
    }

    fn end_of_tokens(&self) -> bool {
        self.tokens.len() <= self.idx
    }

    fn last_token(&self) -> bool {
        self.tokens.len() - 1 == self.idx
    }
}
