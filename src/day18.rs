use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn last_recovered_freq(input: &str) -> i64 {
    let program: Vec<Instruction> = input.trim().lines().map(parse_instruction).collect();
    let mut context = Context::new(program);
    return context.run();
}

pub fn num_sends(input: &str) -> (usize, usize) {
    let p0: Vec<Instruction> = input.trim().lines().map(parse_instruction).collect();
    let p1 = p0.to_vec();
    let (tx0, rx0) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();
    let (rtx0, rrx0) = mpsc::channel();
    let (rtx1, rrx1) = mpsc::channel();

    thread::spawn(move || {
        let mut context = ParallelContext::new(0, p0, tx0, rx1);
        context.run();
        rtx0.send(context.num_sends).unwrap();
    });

    thread::spawn(move || {
        let mut context = ParallelContext::new(1, p1, tx1, rx0);
        context.run();
        rtx1.send(context.num_sends).unwrap();
    });

    (rrx0.recv().unwrap(), rrx1.recv().unwrap())
}

#[derive(Debug, Clone, Copy)]
enum Expression {
    Register(char),
    Value(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Invalid,
    Snd(Expression),
    Set(char, Expression),
    Add(char, Expression),
    Mul(char, Expression),
    Mod(char, Expression),
    Rcv(char),
    Jgz(Expression, Expression),
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
        "rcv" => Instruction::Rcv(get_register(tokens[1])),
        "jgz" => Instruction::Jgz(parse_expression(tokens[1]), parse_expression(tokens[2])),
        _ => Instruction::Invalid,
    }
}

struct Context {
    program: Vec<Instruction>,
    pc: i64,
    registers: HashMap<char, i64>,
    snd: i64,
}

impl Context {
    fn new(program: Vec<Instruction>) -> Context {
        Context{program: program, pc: 0, registers: HashMap::new(), snd: 0}
    }

    fn value_of_register(&mut self, r: &char) -> &mut i64 {
        self.registers.entry(*r).or_insert(0)
    }

    fn value_of_expression(&self, expr: &Expression) -> i64 {
        match *expr {
            Expression::Register(x) => *self.registers.get(&x).unwrap(),
            Expression::Value(v) => v,
        }
    }

    fn execute(&mut self, instruction: &Instruction) -> (bool, i64) {
        let mut is_rcv = false;
        self.pc += 1;
        match *instruction {
            Instruction::Snd(ref e) => self.snd = self.value_of_expression(e),
            Instruction::Set(ref r, ref e) => *self.value_of_register(r) = self.value_of_expression(e),
            Instruction::Add(ref r, ref e) => *self.value_of_register(r) += self.value_of_expression(e),
            Instruction::Mul(ref r, ref e) => *self.value_of_register(r) *= self.value_of_expression(e),
            Instruction::Mod(ref r, ref e) => *self.value_of_register(r) %= self.value_of_expression(e),
            Instruction::Rcv(ref r) => {
                if *self.value_of_register(r) != 0 {
                    is_rcv = true;
                }
            }
            Instruction::Jgz(ref e1, ref e2) => {
                if self.value_of_expression(e1) > 0 {
                    self.pc -= 1;
                    self.pc += self.value_of_expression(e2);
                }
            }
            Instruction::Invalid => assert!(false),
        }
        (is_rcv, self.snd)
    }

    fn run(&mut self) -> i64 {
        loop {
            let pc = self.pc as usize;
            let instruction = self.program[pc].clone();
            let (is_rcv, snd) = self.execute(&instruction);
            if is_rcv {
                return snd;
            }
        }
    }
}

struct ParallelContext {
    id: i64,
    program: Vec<Instruction>,
    pc: i64,
    registers: HashMap<char, i64>,
    tx: mpsc::Sender<i64>,
    rx: mpsc::Receiver<i64>,
    num_sends: usize,
}

impl ParallelContext {
    fn new(id: i64, program: Vec<Instruction>, tx: mpsc::Sender<i64>, rx: mpsc::Receiver<i64>) -> ParallelContext {
        let mut c = ParallelContext{id: id, program: program, pc: 0, registers: HashMap::new(), tx: tx, rx: rx, num_sends: 0};
        c.registers.entry('p').or_insert(c.id);
        c
    }

    fn value_of_register(&mut self, r: &char) -> &mut i64 {
        self.registers.entry(*r).or_insert(0)
    }

    fn value_of_expression(&self, expr: &Expression) -> i64 {
        match *expr {
            Expression::Register(x) => *self.registers.get(&x).unwrap(),
            Expression::Value(v) => v,
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        self.pc += 1;
        match *instruction {
            Instruction::Snd(ref e) => {
                let v = self.value_of_expression(e);
                self.tx.send(v).unwrap();
                self.num_sends += 1;
            }
            Instruction::Set(ref r, ref e) => *self.value_of_register(r) = self.value_of_expression(e),
            Instruction::Add(ref r, ref e) => *self.value_of_register(r) += self.value_of_expression(e),
            Instruction::Mul(ref r, ref e) => *self.value_of_register(r) *= self.value_of_expression(e),
            Instruction::Mod(ref r, ref e) => *self.value_of_register(r) %= self.value_of_expression(e),
            Instruction::Rcv(ref r) => {
                match self.rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(v) => {
                        *self.value_of_register(r) = v;
                    }
                    Err(_) => {
                        self.pc = -1;
                    }
                }
            }
            Instruction::Jgz(ref e1, ref e2) => {
                if self.value_of_expression(e1) > 0 {
                    self.pc -= 1;
                    self.pc += self.value_of_expression(e2);
                }
            }
            Instruction::Invalid => assert!(false),
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc == -1 {
                break;
            }
            let pc = self.pc as usize;
            let instruction = self.program[pc].clone();
            self.execute(&instruction);
        }
    }
}
