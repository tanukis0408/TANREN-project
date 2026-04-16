use crate::token::Token;
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            &Token::Eof
        }
    }

    fn peek(&self) -> &Token {
        if self.pos + 1 < self.tokens.len() {
            &self.tokens[self.pos + 1]
        } else {
            &Token::Eof
        }
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: Token) {
        let got = self.advance();
        if got != expected {
            panic!(
                "Expected {:?}, got {:?} at pos {}",
                expected, got, self.pos
            );
        }
    }

    fn skip_newlines(&mut self) {
        while *self.current() == Token::Newline {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut program = Vec::new();
        self.skip_newlines();

        while *self.current() != Token::Eof {
            let expr = self.parse_statement();
            program.push(expr);
            self.skip_newlines();
        }

        program
    }

    fn parse_statement(&mut self) -> Expr {
        self.skip_newlines();
        match self.current().clone() {
            Token::Say    => self.parse_say(),
            Token::Fn     => self.parse_fn_decl(),
            Token::If     => self.parse_if(),
            Token::For    => self.parse_for(),
            Token::While  => self.parse_while(),
            Token::Loop   => self.parse_loop(),
            Token::Let    => self.parse_let(),
            Token::Return => self.parse_return(),
            Token::Break  => {
                self.advance();
                Expr::Break
            }
            Token::Next => {
                self.advance();
                Expr::Next
            }
            _ => self.parse_assignment_or_expr(),
        }
    }

    fn parse_say(&mut self) -> Expr {
        self.advance(); // съедаем 'say'
        let value = self.parse_expression();
        Expr::Say(Box::new(value))
    }

    fn parse_let(&mut self) -> Expr {
        self.advance(); // съедаем 'let'
        let name = match self.advance() {
            Token::Identifier(n) => n,
            t => panic!("Expected name after let, got {:?}", t),
        };
        self.expect(Token::Eq);
        let value = self.parse_expression();
        Expr::Let {
            name,
            value: Box::new(value),
        }
    }

    fn parse_fn_decl(&mut self) -> Expr {
        self.advance(); // съедаем 'fn'

        // Имя функции
        let name = match self.current().clone() {
            Token::Identifier(n) => {
                self.advance();
                n
            }
            t => panic!("Expected function name after 'fn', got {:?}", t),
        };

        // Параметры
        self.expect(Token::LParen);
        let params = self.parse_params();
        self.expect(Token::RParen);

        self.skip_newlines();

        // Тело функции
        let body = self.parse_block_until(&[Token::End]);

        self.expect(Token::End);

        Expr::FnDecl { name, params, body }
    }

    fn parse_params(&mut self) -> Vec<String> {
        let mut params = Vec::new();

        while *self.current() != Token::RParen
            && *self.current() != Token::Eof
        {
            match self.advance() {
                Token::Identifier(name) => params.push(name),
                t => panic!("Expected param name, got {:?}", t),
            }
            if *self.current() == Token::Comma {
                self.advance();
            }
        }

        params
    }

    fn parse_if(&mut self) -> Expr {
        self.advance(); // съедаем 'if'
        let condition = self.parse_expression();
        self.skip_newlines();

        let then_body = self.parse_block_until(&[
            Token::Elif,
            Token::Else,
            Token::End,
        ]);

        let mut elif_clauses = Vec::new();
        while *self.current() == Token::Elif {
            self.advance(); // съедаем 'elif'
            let elif_cond = self.parse_expression();
            self.skip_newlines();
            let elif_body = self.parse_block_until(&[
                Token::Elif,
                Token::Else,
                Token::End,
            ]);
            elif_clauses.push((elif_cond, elif_body));
        }

        let else_body = if *self.current() == Token::Else {
            self.advance(); // съедаем 'else'
            self.skip_newlines();
            Some(self.parse_block_until(&[Token::End]))
        } else {
            None
        };

        self.expect(Token::End);

        Expr::If {
            condition: Box::new(condition),
            then_body,
            elif_clauses,
            else_body,
        }
    }

    fn parse_for(&mut self) -> Expr {
        self.advance(); // съедаем 'for'

        let var = match self.advance() {
            Token::Identifier(n) => n,
            t => panic!("Expected variable name in for, got {:?}", t),
        };

        self.expect(Token::In);

        let iterable = self.parse_expression();
        self.skip_newlines();

        let body = self.parse_block_until(&[Token::End]);
        self.expect(Token::End);

        Expr::For {
            var,
            iterable: Box::new(iterable),
            body,
        }
    }

    fn parse_while(&mut self) -> Expr {
        self.advance(); // съедаем 'while'
        let condition = self.parse_expression();
        self.skip_newlines();

        let body = self.parse_block_until(&[Token::End]);
        self.expect(Token::End);

        Expr::While {
            condition: Box::new(condition),
            body,
        }
    }

    fn parse_loop(&mut self) -> Expr {
        self.advance(); // съедаем 'loop'
        self.skip_newlines();

        let body = self.parse_block_until(&[Token::End]);
        self.expect(Token::End);

        Expr::Loop { body }
    }

    fn parse_return(&mut self) -> Expr {
        self.advance(); // съедаем 'return'

        if *self.current() == Token::Newline
            || *self.current() == Token::Eof
            || *self.current() == Token::End
        {
            Expr::Return(None)
        } else {
            Expr::Return(Some(Box::new(self.parse_expression())))
        }
    }

    fn parse_block_until(&mut self, terminators: &[Token]) -> Vec<Expr> {
        let mut stmts = Vec::new();
        self.skip_newlines();

        while !terminators.contains(self.current())
            && *self.current() != Token::Eof
        {
            let stmt = self.parse_statement();
            stmts.push(stmt);
            self.skip_newlines();
        }

        stmts
    }

    fn parse_assignment_or_expr(&mut self) -> Expr {
        let expr = self.parse_expression();

        // Проверяем есть ли '=' после выражения
        if *self.current() == Token::Eq {
            self.advance(); // съедаем '='
            let value = self.parse_expression();
            match expr {
                Expr::Identifier(name) => {
                    return Expr::Assign {
                        name,
                        value: Box::new(value),
                    };
                }
                _ => panic!("Invalid assignment target"),
            }
        }

        expr
    }

    // ===== Выражения (Pratt parser) =====

    fn parse_expression(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();

        while *self.current() == Token::Or {
            self.advance();
            let right = self.parse_and();
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::Or,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_comparison();

        while *self.current() == Token::And {
            self.advance();
            let right = self.parse_comparison();
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::And,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_addition();

        loop {
            let op = match self.current() {
                Token::EqEq      => BinOp::Eq,
                Token::NotEq     => BinOp::NotEq,
                Token::Less      => BinOp::Less,
                Token::Greater   => BinOp::Greater,
                Token::LessEq    => BinOp::LessEq,
                Token::GreaterEq => BinOp::GreaterEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_addition();
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_addition(&mut self) -> Expr {
        let mut left = self.parse_multiplication();

        loop {
            let op = match self.current() {
                Token::Plus  => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplication();
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_multiplication(&mut self) -> Expr {
        let mut left = self.parse_unary();

        loop {
            let op = match self.current() {
                Token::Star    => BinOp::Mul,
                Token::Slash   => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary();
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_unary(&mut self) -> Expr {
        match self.current().clone() {
            Token::Minus => {
                self.advance();
                let operand = self.parse_power();
                Expr::UnaryOp {
                    op: UnOp::Neg,
                    operand: Box::new(operand),
                }
            }
            Token::Not => {
                self.advance();
                let operand = self.parse_power();
                Expr::UnaryOp {
                    op: UnOp::Not,
                    operand: Box::new(operand),
                }
            }
            _ => self.parse_power(),
        }
    }

    fn parse_power(&mut self) -> Expr {
        let base = self.parse_call();

        if *self.current() == Token::Power {
            self.advance();
            let exp = self.parse_unary();
            Expr::BinaryOp {
                left: Box::new(base),
                op: BinOp::Pow,
                right: Box::new(exp),
            }
        } else {
            base
        }
    }

    fn parse_call(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            match self.current().clone() {
                Token::LParen => {
                    self.advance(); // съедаем '('
                    let mut args = Vec::new();

                    while *self.current() != Token::RParen
                        && *self.current() != Token::Eof
                    {
                        args.push(self.parse_expression());
                        if *self.current() == Token::Comma {
                            self.advance();
                        }
                    }

                    self.expect(Token::RParen);

                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }

                Token::Dot => {
                    self.advance(); // съедаем '.'
                    let field = match self.advance() {
                        Token::Identifier(n) => n,
                        t => panic!("Expected field name after '.', got {:?}", t),
                    };
                    expr = Expr::MemberAccess {
                        object: Box::new(expr),
                        field,
                    };
                }

                Token::LBracket => {
                    self.advance(); // съедаем '['
                    let index = self.parse_expression();
                    self.expect(Token::RBracket);
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }

                _ => break,
            }
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current().clone() {
            Token::Int(n) => {
                self.advance();
                // Проверяем диапазон 1..10
                if *self.current() == Token::DotDot {
                    self.advance();
                    let end = self.parse_expression();
                    Expr::Range {
                        start: Box::new(Expr::Int(n)),
                        end: Box::new(end),
                    }
                } else {
                    Expr::Int(n)
                }
            }

            Token::Float(n) => {
                self.advance();
                Expr::Float(n)
            }

            Token::Str(s) => {
                self.advance();
                Expr::Str(s)
            }

            Token::Bool(b) => {
                self.advance();
                Expr::Bool(b)
            }

            Token::Nil => {
                self.advance();
                Expr::Nil
            }

            Token::Identifier(name) => {
                self.advance();
                // Проверяем диапазон x..y
                if *self.current() == Token::DotDot {
                    self.advance();
                    let end = self.parse_expression();
                    Expr::Range {
                        start: Box::new(Expr::Identifier(name)),
                        end: Box::new(end),
                    }
                } else {
                    Expr::Identifier(name)
                }
            }

            Token::LParen => {
                self.advance(); // съедаем '('
                let expr = self.parse_expression();
                self.expect(Token::RParen);
                expr
            }

            Token::LBracket => {
                self.advance(); // съедаем '['
                let mut elements = Vec::new();

                while *self.current() != Token::RBracket
                    && *self.current() != Token::Eof
                {
                    elements.push(self.parse_expression());
                    if *self.current() == Token::Comma {
                        self.advance();
                    }
                }

                self.expect(Token::RBracket);
                Expr::List(elements)
            }

            Token::LBrace => {
                self.advance(); // съедаем '{'
                let mut entries = Vec::new();

                while *self.current() != Token::RBrace
                    && *self.current() != Token::Eof
                {
                    self.skip_newlines();
                    let key = match self.advance() {
                        Token::Identifier(k) => k,
                        t => panic!("Expected map key, got {:?}", t),
                    };
                    self.expect(Token::Colon);
                    let val = self.parse_expression();
                    entries.push((key, val));

                    if *self.current() == Token::Comma {
                        self.advance();
                    }
                    self.skip_newlines();
                }

                self.expect(Token::RBrace);
                Expr::Map(entries)
            }

            t => panic!(
                "Unexpected token in expression: {:?} at pos {}",
                t, self.pos
            ),
        }
    }
}