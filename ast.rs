#[derive(Debug, Clone)]
pub enum Expr {
    // Литералы
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,

    // Переменная
    Identifier(String),

    // Блок
    Block(Vec<Expr>),

    // Операции
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },

    UnaryOp {
        op: UnOp,
        operand: Box<Expr>,
    },

    // Присваивание
    Assign {
        name: String,
        value: Box<Expr>,
    },

    // Let (неизменяемое)
    Let {
        name: String,
        value: Box<Expr>,
    },

    // Вызов функции
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    // Вывод
    Say(Box<Expr>),

    // Условия
    If {
        condition: Box<Expr>,
        then_body: Vec<Expr>,
        elif_clauses: Vec<(Expr, Vec<Expr>)>,
        else_body: Option<Vec<Expr>>,
    },

    // Циклы
    For {
        var: String,
        iterable: Box<Expr>,
        body: Vec<Expr>,
    },

    While {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },

    Loop {
        body: Vec<Expr>,
    },

    // Функция
    FnDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Expr>,
    },

    // Лямбда
    Lambda {
        params: Vec<String>,
        body: Vec<Expr>,
    },

    // Возврат
    Return(Option<Box<Expr>>),

    // Управление циклом
    Break,
    Next,

    // Диапазон
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },

    // Коллекции
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),

    // Доступ
    MemberAccess {
        object: Box<Expr>,
        field: String,
    },

    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    NotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Neg,
    Not,
}