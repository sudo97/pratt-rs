use parser::Parser;

struct Mem(Vec<i64>); // Memory of a stack machine

mod parser;
#[macro_use]
mod to_tokens;

#[derive(Debug, Clone)]
enum Op {
    Push(i64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Negate,
}

impl Mem {
    fn new() -> Mem {
        Mem(Vec::new())
    }

    fn run(&mut self, ops: &[Op]) -> Option<()> {
        for op in ops {
            match op {
                Op::Push(val) => self.0.push(*val),
                Op::Pop => {
                    self.0.pop();
                }
                Op::Add => {
                    let a = self.0.pop()?;
                    let b = self.0.pop()?;
                    self.0.push(a + b);
                }
                Op::Sub => {
                    let a = self.0.pop()?;
                    let b = self.0.pop()?;
                    self.0.push(b - a);
                }
                Op::Mul => {
                    let a = self.0.pop()?;
                    let b = self.0.pop()?;
                    self.0.push(a * b);
                }
                Op::Div => {
                    let a = self.0.pop()?;
                    let b = self.0.pop()?;
                    self.0.push(b / a);
                }
                Op::Negate => {
                    let a = self.0.pop()?;
                    self.0.push(-a);
                }
            }
        }
        Some(())
    }

    fn top(&self) -> Option<i64> {
        self.0.last().copied()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

fn beautify(program: Vec<Token>) -> String {
    let mut res = String::new();
    for token in program {
        match token {
            Token::Number(n) => res.push_str(&format!("{} ", n)),
            Token::Plus => res.push_str("+ "),
            Token::Minus => res.push_str("- "),
            Token::Star => res.push_str("* "),
            Token::Slash => res.push_str("/ "),
            Token::LParen => res.push_str("( "),
            Token::RParen => res.push_str(") "),
        }
    }
    res
}

fn test_program(input: Vec<Token>, expected: i64) {
    println!("program: {}", beautify(input.clone()));
    if let Some(program) = Parser::new(input).parse_expression(0) {
        println!("Bytecode: {:?}", program);

        let mut mem = Mem::new();

        mem.run(&program);

        if let Some(res) = mem.top() {
            println!("Result: {}", res);
            println!("Expected: {}", expected);
            // assert_eq!(res, expected);
        } else {
            println!("Error, stack empty");
        }
    } else {
        println!("Error, unable to parse");
    }
}

fn main() {
    to_test!(test_program, 2 + 2 * 2);
    to_test!(test_program, 2 * 2 + 2);
    to_test!(test_program, 2 * (2 + 2));
    to_test!(test_program, 2 * (2) + 2);
    to_test!(test_program, 2 + (2) * 2);
    to_test!(test_program, (2) * 2 + 2);
    to_test!(test_program, (2 * 2 + 2));
    to_test!(test_program, -(2 * 2 + 2));
    to_test!(test_program, -(2 * 2 + 2) / 3);
    to_test!(test_program, 9 - (2 * 2 + 2) / 3);
    to_test!(test_program, 9 - 6 / 3);
    to_test!(test_program, 2 + (2 * 2));

    let prog = to_tokens!(+ -);
    test_program(prog, 0) // should be error
}
