use crate::ast::*;
use crate::bytecode::*;
use std::collections::HashMap;

pub struct Compiler {
    pub chunk: Chunk,
    scopes: Vec<HashMap<String, usize>>,
    local_count: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            scopes: vec![HashMap::new()],
            local_count: 0,
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_local(&self, name: &str) -> Option<usize> {
        for scope in self.scopes.iter().rev() {
            if let Some(&idx) = scope.get(name) {
                return Some(idx);
            }
        }
        None
    }

    fn declare_local(&mut self, name: &str) -> usize {
        let idx = self.local_count;
        self.local_count += 1;
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), idx);
        }
        idx
    }

    pub fn compile(&mut self, program: &[Expr]) {
        for expr in program {
            self.compile_expr(expr);
        }
        self.chunk.emit(Op::Halt);
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Int(n) => {
                let idx = self.chunk.add_const(Const::Int(*n));
                self.chunk.emit(Op::LoadConst(idx));
            }

            Expr::Float(n) => {
                let idx = self.chunk.add_const(Const::Float(*n));
                self.chunk.emit(Op::LoadConst(idx));
            }

            Expr::Str(s) => {
                let idx = self.chunk.add_const(Const::Str(s.clone()));
                self.chunk.emit(Op::LoadConst(idx));
            }

            Expr::Bool(true) => {
                self.chunk.emit(Op::LoadTrue);
            }

            Expr::Bool(false) => {
                self.chunk.emit(Op::LoadFalse);
            }

            Expr::Nil => {
                self.chunk.emit(Op::LoadNil);
            }

            Expr::Identifier(name) => {
                if let Some(idx) = self.resolve_local(name) {
                    self.chunk.emit(Op::GetLocal(idx));
                } else {
                    self.chunk.emit(Op::GetGlobal(name.clone()));
                }
            }

            Expr::Assign { name, value } => {
                self.compile_expr(value);
                if let Some(idx) = self.resolve_local(name) {
                    self.chunk.emit(Op::SetLocal(idx));
                } else {
                    let idx = self.declare_local(name);
                    self.chunk.emit(Op::SetLocal(idx));
                }
            }

            Expr::Let { name, value } => {
                self.compile_expr(value);
                let idx = self.declare_local(name);
                self.chunk.emit(Op::SetLocal(idx));
            }

            Expr::BinaryOp { left, op, right } => {
                match op {
                    BinOp::And => {
                        self.compile_expr(left);
                        let jump = self.chunk.emit(Op::JumpIfFalse(0));
                        self.chunk.emit(Op::Pop);
                        self.compile_expr(right);
                        let end = self.chunk.code.len();
                        self.chunk.patch(jump, Op::JumpIfFalse(end));
                        return;
                    }
                    BinOp::Or => {
                        self.compile_expr(left);
                        let jump = self.chunk.emit(Op::JumpIfTrue(0));
                        self.chunk.emit(Op::Pop);
                        self.compile_expr(right);
                        let end = self.chunk.code.len();
                        self.chunk.patch(jump, Op::JumpIfTrue(end));
                        return;
                    }
                    _ => {}
                }

                self.compile_expr(left);
                self.compile_expr(right);

                match op {
                    BinOp::Add       => { self.chunk.emit(Op::Add); }
                    BinOp::Sub       => { self.chunk.emit(Op::Sub); }
                    BinOp::Mul       => { self.chunk.emit(Op::Mul); }
                    BinOp::Div       => { self.chunk.emit(Op::Div); }
                    BinOp::Mod       => { self.chunk.emit(Op::Mod); }
                    BinOp::Pow       => { self.chunk.emit(Op::Pow); }
                    BinOp::Eq        => { self.chunk.emit(Op::Equal); }
                    BinOp::NotEq     => { self.chunk.emit(Op::NotEqual); }
                    BinOp::Less      => { self.chunk.emit(Op::Less); }
                    BinOp::Greater   => { self.chunk.emit(Op::Greater); }
                    BinOp::LessEq    => { self.chunk.emit(Op::LessEq); }
                    BinOp::GreaterEq => { self.chunk.emit(Op::GreaterEq); }
                    BinOp::And       => {}
                    BinOp::Or        => {}
                }
            }

            Expr::UnaryOp { op, operand } => {
                self.compile_expr(operand);
                match op {
                    UnOp::Neg => { self.chunk.emit(Op::Neg); }
                    UnOp::Not => { self.chunk.emit(Op::Not); }
                }
            }

            Expr::Say(value) => {
                self.compile_expr(value);
                self.chunk.emit(Op::Say);
            }

            Expr::If { condition, then_body, elif_clauses, else_body } => {
                self.compile_expr(condition);
                let jump_false = self.chunk.emit(Op::JumpIfFalse(0));

                self.push_scope();
                for stmt in then_body {
                    self.compile_expr(stmt);
                }
                self.pop_scope();

                let jump_end = self.chunk.emit(Op::Jump(0));
                let mut end_jumps = vec![jump_end];

                let next_pos = self.chunk.code.len();
                self.chunk.patch(jump_false, Op::JumpIfFalse(next_pos));

                for (elif_cond, elif_body) in elif_clauses {
                    self.compile_expr(elif_cond);
                    let elif_jump = self.chunk.emit(Op::JumpIfFalse(0));

                    self.push_scope();
                    for stmt in elif_body {
                        self.compile_expr(stmt);
                    }
                    self.pop_scope();

                    let j = self.chunk.emit(Op::Jump(0));
                    end_jumps.push(j);

                    let pos = self.chunk.code.len();
                    self.chunk.patch(elif_jump, Op::JumpIfFalse(pos));
                }

                if let Some(else_stmts) = else_body {
                    self.push_scope();
                    for stmt in else_stmts {
                        self.compile_expr(stmt);
                    }
                    self.pop_scope();
                }

                let end_pos = self.chunk.code.len();
                for j in end_jumps {
                    self.chunk.patch(j, Op::Jump(end_pos));
                }
            }

            Expr::For { var, iterable, body } => {
                self.compile_expr(iterable);
                self.chunk.emit(Op::IterInit);

                let var_idx = self.declare_local(var);
                let loop_start = self.chunk.code.len();

                let iter_next = self.chunk.emit(Op::IterNext(0));
                self.chunk.emit(Op::SetLocal(var_idx));

                self.push_scope();
                for stmt in body {
                    self.compile_expr(stmt);
                }
                self.pop_scope();

                self.chunk.emit(Op::Jump(loop_start));

                let loop_end = self.chunk.code.len();
                self.chunk.patch(iter_next, Op::IterNext(loop_end));
            }

            Expr::While { condition, body } => {
                let loop_start = self.chunk.code.len();

                self.compile_expr(condition);
                let exit = self.chunk.emit(Op::JumpIfFalse(0));

                self.push_scope();
                for stmt in body {
                    self.compile_expr(stmt);
                }
                self.pop_scope();

                self.chunk.emit(Op::Jump(loop_start));

                let end = self.chunk.code.len();
                self.chunk.patch(exit, Op::JumpIfFalse(end));
            }

            Expr::Loop { body } => {
                let loop_start = self.chunk.code.len();

                self.push_scope();
                for stmt in body {
                    self.compile_expr(stmt);
                }
                self.pop_scope();

                self.chunk.emit(Op::Jump(loop_start));
            }

            Expr::FnDecl { name, params, body } => {
                // Компилируем тело функции отдельным компилятором
                let mut func_compiler = Compiler::new();

                // Параметры — первые локальные переменные
                for param in params {
                    func_compiler.declare_local(param);
                }

                // Тело функции
                for stmt in body {
                    func_compiler.compile_expr(stmt);
                }

                // Если функция ничего не вернула — вернуть nil
                func_compiler.chunk.emit(Op::LoadNil);
                func_compiler.chunk.emit(Op::Return);

                let proto = FuncProto {
                    name: name.clone(),
                    params: params.clone(),
                    code: func_compiler.chunk.code,
                    constants: func_compiler.chunk.constants,
                };

                // Сохраняем функцию в глобальные переменные
                let idx = self.chunk.add_const(Const::Func(proto));
                self.chunk.emit(Op::LoadConst(idx));
                self.chunk.emit(Op::SetGlobal(name.clone()));
            }

            Expr::Lambda { params, body } => {
                let mut func_compiler = Compiler::new();

                for param in params {
                    func_compiler.declare_local(param);
                }

                for stmt in body {
                    func_compiler.compile_expr(stmt);
                }

                func_compiler.chunk.emit(Op::Return);

                let proto = FuncProto {
                    name: "<lambda>".to_string(),
                    params: params.clone(),
                    code: func_compiler.chunk.code,
                    constants: func_compiler.chunk.constants,
                };

                let idx = self.chunk.add_const(Const::Func(proto));
                self.chunk.emit(Op::LoadConst(idx));
            }

            Expr::Call { callee, args } => {
                self.compile_expr(callee);
                let argc = args.len();
                for arg in args {
                    self.compile_expr(arg);
                }
                self.chunk.emit(Op::Call(argc));
            }

            Expr::Return(val) => {
                if let Some(v) = val {
                    self.compile_expr(v);
                } else {
                    self.chunk.emit(Op::LoadNil);
                }
                self.chunk.emit(Op::Return);
            }

            Expr::Break => {
                self.chunk.emit(Op::Jump(usize::MAX));
            }

            Expr::Next => {
                self.chunk.emit(Op::Jump(usize::MAX));
            }

            Expr::Range { start, end } => {
                self.compile_expr(start);
                self.compile_expr(end);
                self.chunk.emit(Op::MakeRange);
            }

            Expr::List(elements) => {
                for elem in elements {
                    self.compile_expr(elem);
                }
                self.chunk.emit(Op::MakeList(elements.len()));
            }

            Expr::Map(entries) => {
                for (key, val) in entries {
                    let idx = self.chunk.add_const(Const::Str(key.clone()));
                    self.chunk.emit(Op::LoadConst(idx));
                    self.compile_expr(val);
                }
                self.chunk.emit(Op::MakeMap(entries.len()));
            }

            Expr::MemberAccess { object, field } => {
                self.compile_expr(object);
                self.chunk.emit(Op::GetField(field.clone()));
            }

            Expr::Index { object, index } => {
                self.compile_expr(object);
                self.compile_expr(index);
                self.chunk.emit(Op::GetIndex);
            }

            Expr::Block(stmts) => {
                self.push_scope();
                for stmt in stmts {
                    self.compile_expr(stmt);
                }
                self.pop_scope();
            }
        }
    }
}