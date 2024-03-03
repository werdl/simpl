use std::default;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Token {
    Number(i32),
    If,
    Else,
    Elif,
    While,
    Fn,
    Return,
    Arguments,
    Struct,
    Int,
    String,
    Float,
    List,

    #[default]
    Null,
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Bang,
    LessThan,
    GreaterThan,
    RightParen,
    LeftParen,
    RightSquareBracket,
    LeftSquareBracket,
    RightCurlyBracket,
    LeftCurlyBracket,

    // Special tokens
    DoubleEqual,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Arrow,

    // structure tokens
    Program,
    Block,
    EOF,
    Line,

    SemiColon,
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            ';' => {

                Token::SemiColon
            },
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftCurlyBracket,
            '}' => Token::RightCurlyBracket,
            '[' => Token::LeftSquareBracket,
            ']' => Token::RightSquareBracket,
            '+' => Token::Plus,
            '-' => {
                if self.peek_char() == '>' {
                    self.read_char();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            
            }
            '*' => Token::Star,
            '/' => Token::Slash,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::DoubleEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::LessThanOrEqual
                } else {
                    Token::LessThan
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::GreaterThanOrEqual
                } else {
                    Token::GreaterThan
                }
            }
            _ => {
                if self.ch == '\0' {
                    return Token::EOF
                }
                if self.ch.is_alphabetic() {
                    let ident = self.read_identifier();

                    match ident.as_str() {
                        "if" => Token::If,
                        "else" => Token::Else,
                        "elif" => Token::Elif,
                        "while" => Token::While,
                        "fn" => Token::Fn,
                        "struct" => Token::Struct,
                        "int" => Token::Int,
                        "string" => Token::String,
                        "float" => Token::Float,
                        "list" => Token::List,
                        "null" => Token::Null,
                        "return" => Token::Return,
                        _ => Token::Identifier(ident),
                    }
                } else if self.ch.is_digit(10) {
                    Token::Number(self.read_number())
                } else {
                    panic!("Unknown token: {}", self.ch);
                }
            }
        };
        self.read_char();
        tok
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }

        self.read_position -= 1;

        self.input[position..self.position].to_string()
    }

    pub fn read_number(&mut self) -> i32 {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[position..self.position].parse().unwrap()
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}