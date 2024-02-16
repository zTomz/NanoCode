#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    Number,
    Operator,
    Identifier,
    Equal,
    EOF,
}
