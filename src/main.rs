use parser::Parser;

struct Mem(Vec<i64>); // Memory of a stack machine

mod parser;

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
}

fn main() {
    let inp = vec![
        Token::Number(2),
        Token::Plus,
        Token::Number(2),
        Token::Star,
        Token::Number(2),
        Token::Minus,
        Token::Number(1),
        Token::Plus,
        Token::Number(10),
        Token::Slash,
        Token::Number(2),
    ]; // 2 + 2 * 2 - 1 + 10 / 2

    if let Some(program) = Parser::new(inp).parse_expression(0) {
        println!("Bytecode: {:?}", program);

        let mut mem = Mem::new();

        mem.run(&program);

        if let Some(res) = mem.top() {
            println!("Result: {}", res);
        } else {
            println!("Error, stack empty");
        }
    } else {
        println!("Error, unable to parse");
    }
}
