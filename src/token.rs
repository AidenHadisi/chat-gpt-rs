use reqwest::header::HeaderValue;
use std::fmt::Display;

pub struct Token(String);

impl Token {
    pub fn new(token: impl ToString) -> Self {
        Self(token.to_string())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bearer {}", self.0)
    }
}

impl From<Token> for HeaderValue {
    fn from(token: Token) -> HeaderValue {
        HeaderValue::from_str(&token.to_string()).unwrap()
    }
}
