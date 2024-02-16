use crate::token::*;

pub struct Lexer {
    pub input: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let l = Lexer {
            input: input.split("").to_owned().collect(),
        };
        l
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut src: Vec<char> = self.input.chars().collect();

        while !src.is_empty() {
            // Skip comments
            if src[0] == '/' && src[1] == '/' {
                while src[0] != '\n' {
                    src.remove(0);
                }
            }

            if src[0] == '=' {
                tokens.push(Token {
                    value: String::from("="),
                    token_type: TokenType::Equal,
                });
                src.remove(0);
            } else if src[0] == '+' || src[0] == '-' || src[0] == '*' || src[0] == '/' {
                tokens.push(Token {
                    value: String::from(src[0].to_string()),
                    token_type: TokenType::Operator,
                });
                src.remove(0);
            } else {
                if src[0].is_alphabetic() {
                    let mut value = String::from("");

                    while !src.is_empty() && src[0].is_alphabetic() {
                        value.push(src[0]);
                        src.remove(0);
                    }

                    tokens.push(Token {
                        value,
                        token_type: TokenType::Identifier,
                    });
                } else if src[0].is_numeric() {
                    let mut value = String::from("");

                    while !src.is_empty() && (src[0].is_numeric() || (src[0] == '.' && !value.contains("."))) {
                        value.push(src[0]);
                        src.remove(0);
                    }

                    tokens.push(Token {
                        value,
                        token_type: TokenType::Number,
                    });
                } else if self.is_skippable(src[0]) {
                    src.remove(0);
                } else {
                    panic!("Unknown token: {}", src[0]);
                }
            }
        }

        tokens.push(Token {
            value: String::from(""),
            token_type: TokenType::EOF,
        });

        tokens
    }

    fn is_skippable(&self, c: char) -> bool {
        c.is_whitespace() || c == '\n' || c == '\t' || c == '\r'
    }
}
