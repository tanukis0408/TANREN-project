#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Op {
    // Константы
    LoadConst(usize),
    LoadNil,
    LoadTrue,
    LoadFalse,

    // Переменные
    GetLocal(usize),
    SetLocal(usize),
    GetGlobal(String),
    SetGlobal(String),

    // Арифметика
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Neg,

    // Сравнение
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEq,
    GreaterEq,

    // Логика
    And,
    Or,
    Not,

    // Переходы
    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),

    // Функции
    MakeFunc(usize),
    Call(usize),
    Return,

    // Встроенные
    Say,

    // Коллекции
    MakeList(usize),
    MakeMap(usize),
    MakeRange,
    GetIndex,
    SetIndex,
    GetField(String),

    // Итерация
    IterInit,
    IterNext(usize),

    // Стек
    Pop,
    Dup,

    Halt,
}

#[derive(Debug, Clone)]
pub enum Const {
    Int(i64),
    Float(f64),
    Str(String),
    Func(FuncProto),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FuncProto {
    pub name: String,
    pub params: Vec<String>,
    pub code: Vec<Op>,
    pub constants: Vec<Const>,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<Op>,
    pub constants: Vec<Const>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_const(&mut self, val: Const) -> usize {
        self.constants.push(val);
        self.constants.len() - 1
    }

    pub fn emit(&mut self, op: Op) -> usize {
        self.code.push(op);
        self.code.len() - 1
    }

    pub fn patch(&mut self, idx: usize, op: Op) {
        self.code[idx] = op;
    }
}