# README.md

```markdown
# 🔩 Metal

> A simple, fast and modern programming language.
> Простой, быстрый и современный язык программирования.

---

## What is Metal?

Metal is a programming language designed to be
a joy to write every single day.

No semicolons. No curly braces. No boilerplate.
Just clean, readable code that works.

```metal
say "Hello, World!"

fn greet(name)
  say "Hello, " + name + "!"
end

greet("TANUKIS")

for i in 1..5
  say i
end
```

---

## Philosophy

```
Write less.    — Минимум символов
Build more.    — Максимум возможностей  
Enjoy coding.  — Удовольствие от кода
```

1. Minimal syntax
2. High readability
3. Fast startup
4. Batteries included
5. Beautiful code by default

---

## Features

| Feature | Status |
|---------|--------|
| Variables | ✅ |
| Functions | ✅ |
| if / elif / else | ✅ |
| for / while / loop | ✅ |
| Lists | ✅ |
| Maps | ✅ |
| Ranges | ✅ |
| Built-in functions | ✅ |
| String concat | ✅ |
| REPL | ✅ |
| Modules | 🔜 |
| Classes | 🔜 |
| Async | 🔜 |
| Standard library | 🔜 |
| Native compilation | 🔜 |

---

## Syntax

### Variables
```metal
name = "TANUKIS"
age  = 20
pi   = 3.14
flag = true
```

### Functions
```metal
fn add(a, b)
  return a + b
end

say add(3, 4)
```

### Conditions
```metal
if x > 10
  say "big"
elif x > 5
  say "medium"
else
  say "small"
end
```

### Loops
```metal
for i in 1..10
  say i
end

while x > 0
  x = x - 1
end
```

### Lists
```metal
items = [1, 2, 3, 4, 5]
say len(items)
items = push(items, 6)
```

### Maps
```metal
user = {
  name: "TANUKIS"
  age: 20
}
say user.name
```

---

## Built-in Functions

| Function | Description |
|----------|-------------|
| `say(x)` | Print with newline |
| `len(x)` | Length of str or list |
| `type(x)` | Type name of value |
| `str(x)` | Convert to string |
| `int(x)` | Convert to integer |
| `float(x)` | Convert to float |
| `input(prompt)` | Read user input |
| `sqrt(x)` | Square root |
| `abs(x)` | Absolute value |
| `max(a, b)` | Maximum of two |
| `min(a, b)` | Minimum of two |
| `push(list, x)` | Add to list |
| `print(x)` | Print without newline |

---

## Installation

### Requirements
- Rust 1.70+
- Cargo

### Build from source

```bash
git clone https://github.com/TANUKIS/metal
cd metal
cargo build --release
```

### Run

```bash
# Run a file
cargo run -- hello.mt

# Run explicit command
cargo run -- run hello.mt

# Validate file (lex/parse/compile only)
cargo run -- check hello.mt

# Start REPL
cargo run --
```

---

## Examples

### Hello World
```metal
say "Hello, World!"
```

### Calculator
```metal
fn add(a, b)
  return a + b
end

fn mul(a, b)
  return a * b
end

say add(10, 20)
say mul(3, 4)
```

### FizzBuzz
```metal
i = 1
while i <= 100
  if i % 15 == 0
    say "FizzBuzz"
  elif i % 3 == 0
    say "Fizz"
  elif i % 5 == 0
    say "Buzz"
  else
    say i
  end
  i = i + 1
end
```

### User input
```metal
name = input("Your name: ")
say "Hello, " + name + "!"
```

---

## Project Structure

```
metal/
├── Cargo.toml
├── README.md
├── main.rs           # Entry point + REPL + tests
├── token.rs          # Token types
├── lexer.rs          # Lexer
├── ast.rs            # AST nodes
├── parser.rs         # Parser
├── bytecode.rs       # Bytecode instructions
├── compiler.rs       # AST → Bytecode
├── value.rs          # Runtime values
├── vm.rs             # Virtual machine
├── ide/
│   └── metal-ide/    # VS Code-based Metal IDE extension + theme
└── examples/
    ├── hello.mt
    ├── fizzbuzz.mt
    └── calculator.mt
```

---

## Roadmap

```
v0.1.0  ✅  Lexer, Parser, Compiler, VM, REPL
v0.2.0  ✅  Built-in functions, String + Number concat
v0.3.0  🔜  String interpolation, Better errors
v0.4.0  🔜  Modules (use math)
v0.5.0  🔜  Classes and structs
v1.0.0  🔜  Standard library, Package manager
v2.0.0  🔜  Native compilation via LLVM
```

---

## Author

**TANUKIS**
— Built Metal from scratch in Rust
— Six months of ideas, now running as code

> 半年間の夢が、今日コードになった。
> Six months of dreams became code today.

---

## License

MIT

---

## Status

🔩 Metal is in early development.
Active and growing every day.

```
Write less. Build more. Enjoy coding.
```
```

---

Сохрани как `README.md` в корне проекта `C:\metalpg\`.
