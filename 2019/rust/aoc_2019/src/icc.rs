use std::collections::VecDeque;

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(PartialEq, PartialOrd)]
enum Operation {
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

#[derive(Clone)]
pub struct IntCodeComputer {
    pub program: Vec<i64>,
    pub pc: usize,
    pub break_on_output: bool,
    pub terminated: bool,
    pub relative_base: i64,
    pub output: String,
    pub previous_operation: i64,
    pub inputq: VecDeque<i64>,
}

impl IntCodeComputer {
    pub fn new(program: Vec<i64>, output_break: bool) -> IntCodeComputer {
        IntCodeComputer {
            program: program.to_vec().clone(),
            pc: 0,
            break_on_output: output_break,
            terminated: false,
            relative_base: 0,
            output: "".to_string(),
            previous_operation: 0,
            inputq: VecDeque::new(),
        }
    }

    pub fn execute(&mut self) {
        loop {
            self.execute_one();
            if self.previous_operation == 99
                || (self.previous_operation == 4 && self.break_on_output)
            {
                break;
            }
        }
    }

    pub fn execute_n_breaking(&mut self, n: usize) {
        for _i in 0..n {
            self.execute_one();
            if self.previous_operation == 99
                || (self.previous_operation == 4 && self.break_on_output)
            {
                break;
            }
        }
    }

    pub fn execute_one(&mut self) {
        let s = format!("{:0>5}", self.program[self.pc].to_string());
        let mut c = s.chars();
        let p2 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            '2' => ParameterMode::Relative,
            _ => unreachable!(),
        };
        let p1 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            '2' => ParameterMode::Relative,
            _ => unreachable!(),
        };
        let p0 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            '2' => ParameterMode::Relative,
            _ => unreachable!(),
        };
        self.previous_operation = c.take(2).collect::<String>().parse::<i64>().unwrap();
        match self.previous_operation {
            1 => self.add(p0, p1, p2),
            2 => self.mul(p0, p1, p2),
            3 => self.input(p0),
            4 => self.produce_output(p0),
            5 => self.conditional(p0, p1, p2, Operation::JumpIfTrue),
            6 => self.conditional(p0, p1, p2, Operation::JumpIfFalse),
            7 => self.conditional(p0, p1, p2, Operation::LessThan),
            8 => self.conditional(p0, p1, p2, Operation::Equals),
            9 => self.relative_base(p0),
            99 => self.terminated = true,
            _ => panic!(),
        }
    }

    fn immediate(&self, offset: usize) -> i64 {
        self.program[self.pc + offset]
    }

    fn position(&self, offset: usize) -> i64 {
        self.program[self.immediate(offset) as usize]
    }

    fn relative(&self, offset: usize) -> i64 {
        self.program[(self.immediate(offset) + self.relative_base) as usize]
    }

    fn position_address(&self, offset: usize) -> usize {
        self.immediate(offset) as usize
    }

    fn relative_address(&self, offset: usize) -> usize {
        (self.immediate(offset) + self.relative_base) as usize
    }

    fn add(&mut self, p0: ParameterMode, p1: ParameterMode, p2: ParameterMode) {
        let a0 = match p0 {
            ParameterMode::Position => self.position(1),
            ParameterMode::Immediate => self.immediate(1),
            ParameterMode::Relative => self.relative(1),
        };
        let a1 = match p1 {
            ParameterMode::Position => self.position(2),
            ParameterMode::Immediate => self.immediate(2),
            ParameterMode::Relative => self.relative(2),
        };
        let output_addr = match p2 {
            ParameterMode::Position => self.position_address(3),
            ParameterMode::Relative => self.relative_address(3),
            _ => unreachable!(),
        };
        self.program[output_addr as usize] = a0 + a1;
        self.pc += 4;
    }

    fn mul(&mut self, p0: ParameterMode, p1: ParameterMode, p2: ParameterMode) {
        let a0 = match p0 {
            ParameterMode::Position => self.position(1),
            ParameterMode::Immediate => self.immediate(1),
            ParameterMode::Relative => self.relative(1),
        };
        let a1 = match p1 {
            ParameterMode::Position => self.position(2),
            ParameterMode::Immediate => self.immediate(2),
            ParameterMode::Relative => self.relative(2),
        };
        let output_addr = match p2 {
            ParameterMode::Position => self.position_address(3),
            ParameterMode::Relative => self.relative_address(3),
            _ => unreachable!(),
        };
        self.program[output_addr as usize] = a0 * a1;
        self.pc += 4;
    }

    fn input(&mut self, p0: ParameterMode) {
        let output_addr = match p0 {
            ParameterMode::Position => self.position_address(1),
            ParameterMode::Relative => self.relative_address(1),
            _ => unreachable!(),
        };
        if !self.inputq.is_empty() {
            self.program[output_addr as usize] = self.inputq.pop_front().unwrap();
        } else {
            self.program[output_addr as usize] = -1;
        }
        self.pc += 2;
    }

    fn produce_output(&mut self, p0: ParameterMode) {
        let s0 = match p0 {
            ParameterMode::Position => self.position(1),
            ParameterMode::Immediate => self.immediate(1),
            ParameterMode::Relative => self.relative(1),
        };
        self.pc += 2;
        self.output.push_str(&s0.to_string());
    }

    pub fn consume_output(&mut self) -> String {
        let output = self.output.clone();
        self.output.clear();
        output
    }

    fn conditional(
        &mut self,
        p0: ParameterMode,
        p1: ParameterMode,
        p2: ParameterMode,
        o: Operation,
    ) {
        let o0 = match p0 {
            ParameterMode::Position => self.position(1),
            ParameterMode::Immediate => self.immediate(1),
            ParameterMode::Relative => self.relative(1),
        };
        let o1 = match p1 {
            ParameterMode::Position => self.position(2),
            ParameterMode::Immediate => self.immediate(2),
            ParameterMode::Relative => self.relative(2),
        };
        let output_addr = match p2 {
            ParameterMode::Position => self.position_address(3),
            ParameterMode::Relative => self.relative_address(3),
            _ => unreachable!(),
        };
        if o == Operation::JumpIfTrue {
            if o0 != 0 {
                self.pc = o1 as usize;
            } else {
                self.pc += 3;
            }
        } else if o == Operation::JumpIfFalse {
            if o0 == 0 {
                self.pc = o1 as usize;
            } else {
                self.pc += 3;
            }
        } else if o == Operation::LessThan {
            self.program[output_addr as usize] = if o0 < o1 { 1 } else { 0 };
            self.pc += 4;
        } else if o == Operation::Equals {
            self.program[output_addr as usize] = if o0 == o1 { 1 } else { 0 };
            self.pc += 4;
        }
    }

    fn relative_base(&mut self, p0: ParameterMode) {
        let relative_adjustment = match p0 {
            ParameterMode::Position => self.position(1),
            ParameterMode::Immediate => self.immediate(1),
            ParameterMode::Relative => self.relative(1),
        };
        self.relative_base += relative_adjustment;
        self.pc += 2;
    }
}
