use std::collections::HashMap;

#[derive(Debug)]
enum Expression {
    Register(char),
    Value(i64),
}

#[derive(Debug)]
enum Instruction {
    Invalid,
    Snd(Expression),
    Set(char, Expression),
    Add(char, Expression),
    Mul(char, Expression),
    Mod(char, Expression),
    Rcv(Expression),
    Jgz(char, Expression),
}

struct Context {
    pc: i64,
    registers: HashMap<char, i64>,
    snd: i64,
}

impl Context {
    fn value_of_register(&mut self, r: &char) -> &mut i64 {
        self.registers.entry(*r).or_insert(0)
    }

    fn value_of_expression(&self, expr: &Expression) -> i64 {
        match *expr {
            Expression::Register(x) => *self.registers.get(&x).unwrap(),
            Expression::Value(v) => v,
        }
    }

    fn run(&mut self, instruction: &Instruction) -> (bool, i64) {
        let mut is_rcv = false;
        self.pc += 1;
        match *instruction {
            Instruction::Snd(ref e) => self.snd = self.value_of_expression(e),
            Instruction::Set(ref r, ref e) => *self.value_of_register(r) = self.value_of_expression(e),
            Instruction::Add(ref r, ref e) => *self.value_of_register(r) += self.value_of_expression(e),
            Instruction::Mul(ref r, ref e) => *self.value_of_register(r) *= self.value_of_expression(e),
            Instruction::Mod(ref r, ref e) => *self.value_of_register(r) %= self.value_of_expression(e),
            Instruction::Rcv(ref e) => {
                if self.value_of_expression(e) != 0 {
                    is_rcv = true;
                }
            }
            Instruction::Jgz(ref r, ref e) => {
                if *self.value_of_register(r) > 0 {
                    self.pc -= 1;
                    self.pc += self.value_of_expression(e);
                }
            }
            Instruction::Invalid => assert!(false),
        }
        (is_rcv, self.snd)
    }
}

fn get_register(expr: &str) -> char {
    expr.chars().next().unwrap()
}

fn parse_expression(expr: &str) -> Expression {
    match expr.parse::<i64>() {
        Ok(v) => Expression::Value(v),
        Err(_) => Expression::Register(expr.chars().next().unwrap()),
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let tokens: Vec<&str> = line.trim().split_whitespace().collect();
    match tokens[0] {
        "snd" => Instruction::Snd(parse_expression(tokens[1])),
        "set" => Instruction::Set(get_register(tokens[1]), parse_expression(tokens[2])),
        "add" => Instruction::Add(get_register(tokens[1]), parse_expression(tokens[2])),
        "mul" => Instruction::Mul(get_register(tokens[1]), parse_expression(tokens[2])),
        "mod" => Instruction::Mod(get_register(tokens[1]), parse_expression(tokens[2])),
        "rcv" => Instruction::Rcv(parse_expression(tokens[1])),
        "jgz" => Instruction::Jgz(get_register(tokens[1]), parse_expression(tokens[2])),
        _ => Instruction::Invalid,
    }
}

pub fn last_recovered_freq(input: &str) -> i64 {
    let program: Vec<Instruction> = input.trim().lines().map(parse_instruction).collect();
    let mut context = Context{pc: 0, registers: HashMap::new(), snd: 0};
    loop {
        let pc = context.pc as usize;
        let (is_rcv, snd) = context.run(&program[pc]);
        if is_rcv {
            return snd;
        }
    }
}
