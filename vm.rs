use crate::bytecode::*;
use crate::value::Value;
use std::collections::HashMap;
use std::io::{self, Write};

struct CallFrame {
    code: Vec<Op>,
    constants: Vec<Const>,
    ip: usize,
    locals: HashMap<usize, Value>,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    locals: HashMap<usize, Value>,
    globals: HashMap<String, Value>,
    call_stack: Vec<CallFrame>,
    iterators: Vec<RangeIter>,
}

struct RangeIter {
    current: i64,
    end: i64,
    items: Option<Vec<Value>>,
    item_pos: usize,
}

impl RangeIter {
    fn from_range(start: i64, end: i64) -> Self {
        RangeIter {
            current: start,
            end,
            items: None,
            item_pos: 0,
        }
    }

    fn from_list(items: Vec<Value>) -> Self {
        RangeIter {
            current: 0,
            end: 0,
            items: Some(items),
            item_pos: 0,
        }
    }

    fn next_val(&mut self) -> Option<Value> {
        if let Some(ref items) = self.items.clone() {
            if self.item_pos < items.len() {
                let val = items[self.item_pos].clone();
                self.item_pos += 1;
                Some(val)
            } else {
                None
            }
        } else {
            if self.current <= self.end {
                let val = Value::Int(self.current);
                self.current += 1;
                Some(val)
            } else {
                None
            }
        }
    }
}

impl VM {
    fn expect_arity(name: &str, args: &[Value], expected: usize) {
        if args.len() != expected {
            panic!(
                "{}() expects {} argument(s), got {}",
                name,
                expected,
                args.len()
            );
        }
    }

    fn as_f64_pair(a: &Value, b: &Value) -> Option<(f64, f64)> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Some((*a as f64, *b as f64)),
            (Value::Int(a), Value::Float(b)) => Some((*a as f64, *b)),
            (Value::Float(a), Value::Int(b)) => Some((*a, *b as f64)),
            (Value::Float(a), Value::Float(b)) => Some((*a, *b)),
            _ => None,
        }
    }

    pub fn new(chunk: Chunk) -> Self {
        let mut vm = VM {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
            locals: HashMap::new(),
            globals: HashMap::new(),
            call_stack: Vec::new(),
            iterators: Vec::new(),
        };

        let builtins = vec![
            "len", "type", "int", "float", "str",
            "input", "sqrt", "abs", "max", "min",
            "push", "print",
        ];

        for name in builtins {
            vm.globals.insert(
                name.to_string(),
                Value::Builtin(name.to_string()),
            );
        }

        vm
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow")
    }

    fn peek(&self) -> &Value {
        self.stack.last().expect("Stack empty")
    }

    fn get_local(&self, idx: usize) -> Value {
        if let Some(frame) = self.call_stack.last() {
            frame.locals.get(&idx).cloned().unwrap_or(Value::Nil)
        } else {
            self.locals.get(&idx).cloned().unwrap_or(Value::Nil)
        }
    }

    fn set_local(&mut self, idx: usize, val: Value) {
        if let Some(frame) = self.call_stack.last_mut() {
            frame.locals.insert(idx, val);
        } else {
            self.locals.insert(idx, val);
        }
    }

    fn set_ip(&mut self, ip: usize) {
        if let Some(frame) = self.call_stack.last_mut() {
            frame.ip = ip;
        } else {
            self.ip = ip;
        }
    }

    fn inc_ip(&mut self) {
        if let Some(frame) = self.call_stack.last_mut() {
            frame.ip += 1;
        } else {
            self.ip += 1;
        }
    }

    fn get_op(&self) -> Option<Op> {
        if let Some(frame) = self.call_stack.last() {
            frame.code.get(frame.ip).cloned()
        } else {
            self.chunk.code.get(self.ip).cloned()
        }
    }

    fn get_const(&self, idx: usize) -> Const {
        if let Some(frame) = self.call_stack.last() {
            frame.constants[idx].clone()
        } else {
            self.chunk.constants[idx].clone()
        }
    }

    fn call_builtin(&self, name: &str, args: Vec<Value>) -> Value {
        match name {
            "len" => {
                Self::expect_arity(name, &args, 1);
                match &args[0] {
                    Value::Str(s)  => Value::Int(s.len() as i64),
                    Value::List(l) => Value::Int(l.len() as i64),
                    _ => panic!("len() requires str or list"),
                }
            }
            "type" => {
                Self::expect_arity(name, &args, 1);
                Value::Str(args[0].type_name().to_string())
            }
            "int" => {
                Self::expect_arity(name, &args, 1);
                match &args[0] {
                    Value::Str(s)   => Value::Int(s.parse().unwrap_or(0)),
                    Value::Float(f) => Value::Int(*f as i64),
                    Value::Int(n)   => Value::Int(*n),
                    _ => Value::Int(0),
                }
            }
            "float" => {
                Self::expect_arity(name, &args, 1);
                match &args[0] {
                    Value::Str(s)   => Value::Float(s.parse().unwrap_or(0.0)),
                    Value::Int(n)   => Value::Float(*n as f64),
                    Value::Float(f) => Value::Float(*f),
                    _ => Value::Float(0.0),
                }
            }
            "str" => {
                Self::expect_arity(name, &args, 1);
                Value::Str(format!("{}", args[0]))
            }
            "input" => {
                if args.len() > 1 {
                    panic!("input() expects 0 or 1 argument(s), got {}", args.len());
                }
                if args.len() == 1 {
                    print!("{}", args[0]);
                    io::stdout().flush().unwrap();
                }
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                Value::Str(line.trim().to_string())
            }
            "sqrt" => {
                Self::expect_arity(name, &args, 1);
                match &args[0] {
                    Value::Int(n)   => Value::Float((*n as f64).sqrt()),
                    Value::Float(f) => Value::Float(f.sqrt()),
                    _ => panic!("sqrt() requires number"),
                }
            }
            "abs" => {
                Self::expect_arity(name, &args, 1);
                match &args[0] {
                    Value::Int(n)   => Value::Int(n.abs()),
                    Value::Float(f) => Value::Float(f.abs()),
                    _ => panic!("abs() requires number"),
                }
            }
            "max" => {
                Self::expect_arity(name, &args, 2);
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b))     => Value::Int(*a.max(b)),
                    (Value::Float(a), Value::Float(b)) => Value::Float(a.max(*b)),
                    (Value::Int(a), Value::Float(b))   => Value::Float((*a as f64).max(*b)),
                    (Value::Float(a), Value::Int(b))   => Value::Float((*a).max(*b as f64)),
                    _ => panic!("max() requires numbers"),
                }
            }
            "min" => {
                Self::expect_arity(name, &args, 2);
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b))     => Value::Int(*a.min(b)),
                    (Value::Float(a), Value::Float(b)) => Value::Float(a.min(*b)),
                    (Value::Int(a), Value::Float(b))   => Value::Float((*a as f64).min(*b)),
                    (Value::Float(a), Value::Int(b))   => Value::Float((*a).min(*b as f64)),
                    _ => panic!("min() requires numbers"),
                }
            }
            "push" => {
                Self::expect_arity(name, &args, 2);
                match &args[0] {
                    Value::List(items) => {
                        let mut new_list = items.clone();
                        new_list.push(args[1].clone());
                        Value::List(new_list)
                    }
                    _ => panic!("push() requires list"),
                }
            }
            "print" => {
                for arg in &args {
                    print!("{}", arg);
                }
                io::stdout().flush().unwrap();
                Value::Nil
            }
            _ => panic!("Unknown builtin: {}", name),
        }
    }

    pub fn run(&mut self) {
        loop {
            let op = match self.get_op() {
                Some(op) => op,
                None => break,
            };
            self.inc_ip();

            match op {
                Op::LoadConst(idx) => {
                    let val = match self.get_const(idx) {
                        Const::Int(n)   => Value::Int(n),
                        Const::Float(n) => Value::Float(n),
                        Const::Str(s)   => Value::Str(s),
                        Const::Func(fp) => Value::Func(fp),
                    };
                    self.push(val);
                }

                Op::LoadNil   => self.push(Value::Nil),
                Op::LoadTrue  => self.push(Value::Bool(true)),
                Op::LoadFalse => self.push(Value::Bool(false)),

                Op::GetLocal(idx) => {
                    let val = self.get_local(idx);
                    self.push(val);
                }

                Op::SetLocal(idx) => {
                    let val = self.pop();
                    self.set_local(idx, val);
                }

                Op::GetGlobal(name) => {
                    let val = self.globals
                        .get(&name)
                        .cloned()
                        .unwrap_or(Value::Nil);
                    self.push(val);
                }

                Op::SetGlobal(name) => {
                    let val = self.pop();
                    self.globals.insert(name, val);
                }

                Op::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b))     => Value::Int(a + b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                        (Value::Int(a), Value::Float(b))   => Value::Float(*a as f64 + b),
                        (Value::Float(a), Value::Int(b))   => Value::Float(a + *b as f64),
                        (Value::Str(a), Value::Str(b))     => Value::Str(format!("{}{}", a, b)),
                        (Value::Str(a), Value::Int(b))     => Value::Str(format!("{}{}", a, b)),
                        (Value::Str(a), Value::Float(b))   => Value::Str(format!("{}{}", a, b)),
                        (Value::Str(a), Value::Bool(b))    => Value::Str(format!("{}{}", a, b)),
                        (Value::Int(a), Value::Str(b))     => Value::Str(format!("{}{}", a, b)),
                        (Value::Float(a), Value::Str(b))   => Value::Str(format!("{}{}", a, b)),
                        (Value::Bool(a), Value::Str(b))    => Value::Str(format!("{}{}", a, b)),
                        _ => panic!("Cannot add {} and {}", a.type_name(), b.type_name()),
                    };
                    self.push(result);
                }

                Op::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b))     => Value::Int(a - b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                        (Value::Int(a), Value::Float(b))   => Value::Float(*a as f64 - b),
                        (Value::Float(a), Value::Int(b))   => Value::Float(a - *b as f64),
                        _ => panic!("Cannot subtract {} and {}", a.type_name(), b.type_name()),
                    };
                    self.push(result);
                }

                Op::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b))     => Value::Int(a * b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                        (Value::Int(a), Value::Float(b))   => Value::Float(*a as f64 * b),
                        (Value::Float(a), Value::Int(b))   => Value::Float(a * *b as f64),
                        _ => panic!("Cannot multiply {} and {}", a.type_name(), b.type_name()),
                    };
                    self.push(result);
                }

                Op::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => {
                            if *b == 0 { panic!("Division by zero") }
                            Value::Int(a / b)
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            if *b == 0.0 { panic!("Division by zero") }
                            Value::Float(a / b)
                        }
                        (Value::Int(a), Value::Float(b))   => {
                            if *b == 0.0 { panic!("Division by zero") }
                            Value::Float(*a as f64 / b)
                        }
                        (Value::Float(a), Value::Int(b))   => {
                            if *b == 0 { panic!("Division by zero") }
                            Value::Float(a / *b as f64)
                        }
                        _ => panic!("Cannot divide {} by {}", a.type_name(), b.type_name()),
                    };
                    self.push(result);
                }

                Op::Mod => {
                    let b = self.pop();
                    let a = self.pop();
                    match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => {
                            if *b == 0 {
                                panic!("Modulo by zero");
                            }
                            self.push(Value::Int(a % b))
                        }
                        _ => panic!("Cannot mod {} and {}", a.type_name(), b.type_name()),
                    }
                }

                Op::Pow => {
                    let b = self.pop();
                    let a = self.pop();
                    match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => {
                            self.push(Value::Int((*a as f64).powi(*b as i32) as i64))
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.push(Value::Float(a.powf(*b)))
                        }
                        (Value::Int(a), Value::Float(b)) => {
                            self.push(Value::Float((*a as f64).powf(*b)))
                        }
                        _ => panic!("Cannot pow"),
                    }
                }

                Op::Neg => {
                    let val = self.pop();
                    match val {
                        Value::Int(n)   => self.push(Value::Int(-n)),
                        Value::Float(n) => self.push(Value::Float(-n)),
                        _ => panic!("Cannot negate {}", val.type_name()),
                    }
                }

                Op::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b))     => a == b,
                        (Value::Float(a), Value::Float(b)) => a == b,
                        (Value::Str(a), Value::Str(b))     => a == b,
                        (Value::Bool(a), Value::Bool(b))   => a == b,
                        (Value::Nil, Value::Nil)           => true,
                        _                                  => false,
                    };
                    self.push(Value::Bool(result));
                }

                Op::NotEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b))     => a != b,
                        (Value::Str(a), Value::Str(b))     => a != b,
                        (Value::Bool(a), Value::Bool(b))   => a != b,
                        (Value::Nil, Value::Nil)           => false,
                        _                                  => true,
                    };
                    self.push(Value::Bool(result));
                }

                Op::Less => {
                    let b = self.pop();
                    let a = self.pop();
                    match Self::as_f64_pair(&a, &b) {
                        Some((a, b)) => self.push(Value::Bool(a < b)),
                        None => panic!("Cannot compare {} < {}", a.type_name(), b.type_name()),
                    }
                }

                Op::Greater => {
                    let b = self.pop();
                    let a = self.pop();
                    match Self::as_f64_pair(&a, &b) {
                        Some((a, b)) => self.push(Value::Bool(a > b)),
                        None => panic!("Cannot compare {} > {}", a.type_name(), b.type_name()),
                    }
                }

                Op::LessEq => {
                    let b = self.pop();
                    let a = self.pop();
                    match Self::as_f64_pair(&a, &b) {
                        Some((a, b)) => self.push(Value::Bool(a <= b)),
                        None => panic!("Cannot compare {} <= {}", a.type_name(), b.type_name()),
                    }
                }

                Op::GreaterEq => {
                    let b = self.pop();
                    let a = self.pop();
                    match Self::as_f64_pair(&a, &b) {
                        Some((a, b)) => self.push(Value::Bool(a >= b)),
                        None => panic!("Cannot compare {} >= {}", a.type_name(), b.type_name()),
                    }
                }

                Op::Not => {
                    let val = self.pop();
                    self.push(Value::Bool(!val.is_truthy()));
                }

                Op::And => {
                    let b = self.pop();
                    let a = self.pop();
                    if !a.is_truthy() {
                        self.push(a);
                    } else {
                        self.push(b);
                    }
                }

                Op::Or => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_truthy() {
                        self.push(a);
                    } else {
                        self.push(b);
                    }
                }

                Op::Jump(target) => {
                    self.set_ip(target);
                }

                Op::JumpIfFalse(target) => {
                    let val = self.pop();
                    if !val.is_truthy() {
                        self.set_ip(target);
                    }
                }

                Op::JumpIfTrue(target) => {
                    let val = self.pop();
                    if val.is_truthy() {
                        self.set_ip(target);
                    }
                }

                Op::Call(argc) => {
                    let mut args = Vec::new();
                    for _ in 0..argc {
                        args.push(self.pop());
                    }
                    args.reverse();

                    let callee = self.pop();

                    match callee {
                        Value::Builtin(name) => {
                            let result = self.call_builtin(&name, args);
                            self.push(result);
                        }
                        Value::Func(proto) => {
                            let mut locals = HashMap::new();
                            for (i, arg) in args.into_iter().enumerate() {
                                locals.insert(i, arg);
                            }
                            let frame = CallFrame {
                                code: proto.code,
                                constants: proto.constants,
                                ip: 0,
                                locals,
                            };
                            self.call_stack.push(frame);
                        }
                        Value::Nil => {
                            panic!("Error: function not found");
                        }
                        _ => panic!("Error: cannot call {}", callee.type_name()),
                    }
                }

                Op::Return => {
                    let ret_val = self.pop();
                    self.call_stack.pop();
                    self.push(ret_val);
                }

                Op::Say => {
                    let val = self.pop();
                    println!("{}", val);
                }

                Op::MakeRange => {
                    let end = self.pop();
                    let start = self.pop();
                    match (&start, &end) {
                        (Value::Int(a), Value::Int(b)) => {
                            self.push(Value::Range(*a, *b));
                        }
                        _ => panic!("Range requires int..int"),
                    }
                }

                Op::MakeList(count) => {
                    let mut items = Vec::new();
                    for _ in 0..count {
                        items.push(self.pop());
                    }
                    items.reverse();
                    self.push(Value::List(items));
                }

                Op::MakeMap(count) => {
                    let mut map = HashMap::new();
                    for _ in 0..count {
                        let val = self.pop();
                        let key = self.pop();
                        if let Value::Str(k) = key {
                            map.insert(k, val);
                        }
                    }
                    self.push(Value::Map(map));
                }

                Op::GetField(field) => {
                    let obj = self.pop();
                    match obj {
                        Value::Map(map) => {
                            let val = map.get(&field)
                                .cloned()
                                .unwrap_or(Value::Nil);
                            self.push(val);
                        }
                        _ => panic!(
                            "Cannot access field '{}' on {}",
                            field,
                            obj.type_name()
                        ),
                    }
                }

                Op::GetIndex => {
                    let index = self.pop();
                    let obj = self.pop();
                    match (&obj, &index) {
                        (Value::List(items), Value::Int(i)) => {
                            let idx = if *i < 0 {
                                (items.len() as i64 + i) as usize
                            } else {
                                *i as usize
                            };
                            let val = items.get(idx)
                                .cloned()
                                .unwrap_or(Value::Nil);
                            self.push(val);
                        }
                        (Value::Map(map), Value::Str(key)) => {
                            let val = map.get(key)
                                .cloned()
                                .unwrap_or(Value::Nil);
                            self.push(val);
                        }
                        _ => panic!(
                            "Cannot index {} with {}",
                            obj.type_name(),
                            index.type_name()
                        ),
                    }
                }

                Op::IterInit => {
                    let val = self.pop();
                    match val {
                        Value::Range(start, end) => {
                            self.iterators.push(RangeIter::from_range(start, end));
                        }
                        Value::List(items) => {
                            self.iterators.push(RangeIter::from_list(items));
                        }
                        _ => panic!("Cannot iterate over {}", val.type_name()),
                    }
                }

                Op::IterNext(exit_target) => {
                    let next = if let Some(iter) = self.iterators.last_mut() {
                        iter.next_val()
                    } else {
                        None
                    };

                    match next {
                        Some(val) => self.push(val),
                        None => {
                            self.iterators.pop();
                            self.set_ip(exit_target);
                        }
                    }
                }

                Op::Pop => { self.pop(); }

                Op::Dup => {
                    let val = self.peek().clone();
                    self.push(val);
                }

                Op::MakeFunc(_) => {}
                Op::SetIndex   => {}

                Op::Halt => break,
            }
        }
    }
}
