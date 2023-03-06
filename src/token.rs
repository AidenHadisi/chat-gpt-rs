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


