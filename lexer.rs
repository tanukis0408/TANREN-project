use crate::token::Token;

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn current(&self) -> char {
        if self.pos < self.source.len() {
            self.source[self.pos]
        } else {
            '\0'
        }
    }

    fn peek(&self) -> char {
        if self.pos + 1 < self.source.len() {
            self.source[self.pos + 1]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.current();
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.source.len()
            && self.current() != '\n'
            && self.current().is_whitespace()
        {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        if self.current() == '-' && self.peek() == '-' {
            while self.pos < self.source.len() && self.current() != '\n' {
                self.advance();
            }
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance();
        let mut s = String::new();
        while self.pos < self.source.len() && self.current() != '"' {
            if self.current() == '\\' {
                self.advance();
                match self.current() {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    _ => {
                        s.push('\\');
                        s.push(self.current());
                    }
                }
            } else {
                s.push(self.current());
            }
            self.advance();
        }
        self.advance();
        Token::Str(s)
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        let mut is_float = false;

        while self.pos < self.source.len()
            && (self.current().is_ascii_digit()
                || self.current() == '.'
                || self.current() == '_')
        {
            if self.current() == '.' {
                if self.peek() == '.' {
                    break;
                }
                is_float = true;
            }
            if self.current() != '_' {
                num.push(self.current());
            }
            self.advance();
        }

        if is_float {
            Token::Float(num.parse().unwrap_or(0.0))
        } else {
            Token::Int(num.parse().unwrap_or(0))
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut name = String::new();
        while self.pos < self.source.len()
            && (self.current().is_alphanumeric()
                || self.current() == '_'
                || self.current() == '?'
                || self.current() == '!')
        {
            name.push(self.current());
            self.advance();
        }

        match name.as_str() {
            "fn"     => Token::Fn,
            "end"    => Token::End,
            "if"     => Token::If,
            "elif"   => Token::Elif,
            "else"   => Token::Else,
            "for"    => Token::For,
            "in"     => Token::In,
            "while"  => Token::While,
            "loop"   => Token::Loop,
            "return" => Token::Return,
            "let"    => Token::Let,
            "say"    => Token::Say,
            "and"    => Token::And,
            "or"     => Token::Or,
            "not"    => Token::Not,
            "true"   => Token::Bool(true),
            "false"  => Token::Bool(false),
            "nil"    => Token::Nil,
            "use"    => Token::Use,
            "match"  => Token::Match,
            "break"  => Token::Break,
            "next"   => Token::Next,
            "struct" => Token::Struct,
            "class"  => Token::Class,
            _        => Token::Identifier(name),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.source.len() {
            self.skip_whitespace();

            if self.pos >= self.source.len() {
                break;
            }

            // Пропустить комментарий
            if self.current() == '-' && self.peek() == '-' {
                self.skip_comment();
                continue;
            }

            let ch = self.current();

            let token = match ch {
                '\n' => {
                    self.advance();
                    Token::Newline
                }
                '"' => self.read_string(),
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                '*' => {
                    self.advance();
                    if self.current() == '*' {
                        self.advance();
                        Token::Power
                    } else {
                        Token::Star
                    }
                }
                '/' => {
                    self.advance();
                    Token::Slash
                }
                '%' => {
                    self.advance();
                    Token::Percent
                }
                '=' => {
                    self.advance();
                    if self.current() == '=' {
                        self.advance();
                        Token::EqEq
                    } else if self.current() == '>' {
                        self.advance();
                        Token::Arrow
                    } else {
                        Token::Eq
                    }
                }
                '!' => {
                    self.advance();
                    if self.current() == '=' {
                        self.advance();
                        Token::NotEq
                    } else {
                        continue;
                    }
                }
                '<' => {
                    self.advance();
                    if self.current() == '=' {
                        self.advance();
                        Token::LessEq
                    } else {
                        Token::Less
                    }
                }
                '>' => {
                    self.advance();
                    if self.current() == '=' {
                        self.advance();
                        Token::GreaterEq
                    } else {
                        Token::Greater
                    }
                }
                '(' => {
                    self.advance();
                    Token::LParen
                }
                ')' => {
                    self.advance();
                    Token::RParen
                }
                '[' => {
                    self.advance();
                    Token::LBracket
                }
                ']' => {
                    self.advance();
                    Token::RBracket
                }
                '{' => {
                    self.advance();
                    Token::LBrace
                }
                '}' => {
                    self.advance();
                    Token::RBrace
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                ':' => {
                    self.advance();
                    Token::Colon
                }
                '.' => {
                    self.advance();
                    if self.current() == '.' {
                        self.advance();
                        Token::DotDot
                    } else {
                        Token::Dot
                    }
                }
                '|' => {
                    self.advance();
                    if self.current() == '>' {
                        self.advance();
                        Token::Pipe
                    } else {
                        Token::Bar
                    }
                }
                _ if ch.is_ascii_digit() => self.read_number(),
                _ if ch.is_alphabetic() || ch == '_' => self.read_identifier(),
                _ => {
                    self.advance();
                    continue;
                }
            };

            tokens.push(token);
        }

        tokens.push(Token::Eof);
        tokens
    }
}