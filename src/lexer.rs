use std::{
    fmt::{self, Formatter},
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(num) => write!(f, "{}", num),
            Self::Float(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug)]
pub enum Token {
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftSquareBrace,
    RightSquareBrace,
    Colon,
    Comma,
    StringToken(String),
    Number(Number),
    Null,
    Bool(bool),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftCurlyBrace => write!(f, "{{"),
            Token::RightCurlyBrace => write!(f, "}}"),
            Token::RightSquareBrace => write!(f, "]"),
            Token::LeftSquareBrace => write!(f, "["),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::StringToken(s) => write!(f, "\"{}\"", s),
            Token::Number(num) => write!(f, "{}", num),
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
            '[' => tokens.push(Token::LeftSquareBrace),
            ']' => tokens.push(Token::RightSquareBrace),
            '"' => {
                let mut string = String::new();
                let mut found_string_end = false;
                let mut preceding_character = ' ';

                while let Some(ch) = characters.next() {
                    if ch == '"' && preceding_character != '\\' {
                        found_string_end = true;
                        break;
                    }

                    string.push(ch);
                    preceding_character = ch;
                }

                if !found_string_end {
                    return Err(LexerError::UnterminatedString);
                }

                tokens.push(Token::StringToken(string));
            }
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
            '-' | '0'..='9' => {
                let mut number = String::from(char);

                while let Some(ch) = characters.next() {
                    if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == '-' {
                        number.push(ch);
                    } else {
                        break;
                    }
                }

                if number.contains('.') {
                    tokens.push(Token::Number(Number::Float(number.parse::<f64>()?)));
                } else {
                    tokens.push(Token::Number(Number::Integer(number.parse::<i64>()?)))
                }
            }
            _ => (),
        }
    }

    Ok(tokens)
}
