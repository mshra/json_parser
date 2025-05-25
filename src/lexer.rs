use std::{
    fmt::{self, Formatter},
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug)]
pub enum Token {
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftSquareBrace,
    RightSquareBrace,
    Colon,
    Comma,
    StringToken(String),
    FloatToken(f64),
    IntegerToken(i64),
    Null,
    Bool(bool),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftCurlyBrace => write!(f, "{{"),
            Token::RightCurlyBrace => write!(f, "}}"),
            Token::LeftSquareBrace => write!(f, "["),
            Token::RightSquareBrace => write!(f, "]"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::StringToken(s) => write!(f, "\"{}\"", s),
            Token::FloatToken(num) => write!(f, "{}", num),
            Token::IntegerToken(num) => write!(f, "{}", num),
            Token::Null => write!(f, "null"),
            Token::Bool(bool_value) => write!(f, "{}", bool_value),
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnterminatedString,
    InvalidFloat(ParseFloatError),
    InvalidInteger(ParseIntError),
}

impl std::error::Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LexerError::UnterminatedString => write!(f, "Unterminated string literal"),
            LexerError::InvalidFloat(err) => write!(f, "Error parsing float literal: {}", err),
            LexerError::InvalidInteger(err) => write!(f, "Error parsing integer literal: {}", err),
        }
    }
}

impl From<ParseIntError> for LexerError {
    fn from(err: ParseIntError) -> Self {
        LexerError::InvalidInteger(err)
    }
}

impl From<ParseFloatError> for LexerError {
    fn from(err: ParseFloatError) -> Self {
        LexerError::InvalidFloat(err)
    }
}

pub fn lexer(json_string: String) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = vec![];
    let mut characters = json_string.chars().peekable();

    while let Some(char) = characters.next() {
        match char {
            '{' => tokens.push(Token::LeftCurlyBrace),
            '}' => tokens.push(Token::RightCurlyBrace),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '"' => {
                let mut string = String::new();
                let mut found_string_end = false;

                while let Some(ch) = characters.next() {
                    if ch == '"' {
                        found_string_end = true;
                        break;
                    }

                    string.push(ch)
                }

                if !found_string_end {
                    return Err(LexerError::UnterminatedString);
                }

                tokens.push(Token::StringToken(string));
            }
            '[' => tokens.push(Token::LeftSquareBrace),
            ']' => tokens.push(Token::RightSquareBrace),
            't' => {
                let next_three: String = characters.by_ref().take(3).collect();
                if next_three == "rue" {
                    tokens.push(Token::Bool(true));
                }
            }
            'f' => {
                let next_three: String = characters.by_ref().take(4).collect();
                if next_three == "alse" {
                    tokens.push(Token::Bool(false));
                }
            }
            'n' => {
                let next_three: String = characters.by_ref().take(3).collect();
                if next_three == "ull" {
                    tokens.push(Token::Null);
                }
            }
            '0'..='9' => {
                let mut number = String::from(char);

                while let Some(ch) = characters.next() {
                    if ch.is_ascii_digit() || ch == '.' {
                        number.push(ch);
                    } else {
                        break;
                    }
                }

                if number.contains('.') {
                    tokens.push(Token::FloatToken(number.parse::<f64>()?));
                } else {
                    tokens.push(Token::IntegerToken(number.parse::<i64>()?));
                }
            }
            _ => (),
        }
    }

    Ok(tokens)
}
