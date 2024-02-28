use log::{error, info};

mod instructions;

const STACK_SIZE: usize = 1024;
const MEM_SIZE: usize = 1024;

#[derive(Clone, Copy)]
pub enum VMError {
    StackOverflow,
    CorruptStack,
    InvalidMemoryAddress,
    UnknownInstruction(i32),
    IOError,
}

pub struct VM {
    stack: Box<[i32]>,
    memory: Box<[i32]>,
    program: Vec<i32>,
    ip: usize,
    sp: usize,
    hf: bool,
    rf: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: vec![0; STACK_SIZE].into_boxed_slice(),
            memory: vec![0; MEM_SIZE].into_boxed_slice(),
            program: Vec::new(),
            ip: 0,
            sp: 0,
            hf: false,
            rf: false,
        }
    }

    pub fn load(&mut self, filename: &str) -> Result<(), std::io::Error> {
        use std::io::Read;
        info!("loading program from file [{filename}]");
        let mut file = std::fs::File::open(filename)?;
        let mut bytes = Vec::with_capacity(4);
        loop {
            bytes.clear();
            let read = file.by_ref().take(4).read_to_end(&mut bytes)?;
            if read == 0 {
                break;
            }
            let instruction = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
            self.program.push(instruction);
        }
        info!("program loaded into memory");
        Ok(())
    }

    pub fn run(&mut self) {
        info!("starting program execution");
        while self.ip < self.program.len() && !self.hf {
            match self.tick() {
                Ok(_) => {}
                Err(VMError::CorruptStack) => {
                    error!("corrupt stack");
                    return;
                }
                Err(VMError::InvalidMemoryAddress) => {
                    error!("invalid memory address");
                    return;
                }
                Err(VMError::StackOverflow) => {
                    error!("stack overflow");
                    return;
                }
                Err(VMError::UnknownInstruction(inst)) => {
                    error!("unknown instruction {inst:#X}");
                    return;
                }
                Err(VMError::IOError) => {
                    error!("io error");
                    return;
                }
            }
        }

        info!("completed program execution");
    }

    fn tick(&mut self) -> Result<(), VMError> {
        let inst = self.program[self.ip];
        self.execute(inst)?;
        self.ip += 1;
        Ok(())
    }

    fn execute(&mut self, inst: i32) -> Result<(), VMError> {
        // non negative values are pushed to the stack
        if inst >= 0 {
            self.push(inst)?;
        } else {
            use instructions::*;
            match inst {
                // I/O
                IN => {
                    use std::io::Write;
                    if self.rf {
                        unimplemented!("IN raw mode");
                    } else {
                        print!("?");
                        std::io::stdout().flush().map_err(|_| VMError::IOError)?;

                        let mut line = String::new();
                        std::io::stdin()
                            .read_line(&mut line)
                            .map_err(|_| VMError::IOError)?;

                        let v = line.trim().parse::<i32>().map_err(|_| VMError::IOError)?;
                        self.push(v)?;
                    }
                }
                OUT => {
                    if !self.rf {
                        let v = self.pop()?;
                        println!("{v}");
                    } else {
                        let c = (self.pop()? % 256) as u8 as char;
                        print!("{c}");
                    }
                }

                // Arithmetic
                ADD => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b + a)?;
                }
                SUB => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b - a)?;
                }
                MUL => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b * a)?;
                }
                DIV => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b / a)?;
                }
                MOD => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b % a)?;
                }
                NEG => {
                    let a = self.pop()?;
                    self.push(-a)?;
                }
                INC => {
                    let a = self.pop()?;
                    self.push(a + 1)?;
                }
                DEC => {
                    let a = self.pop()?;
                    self.push(a - 1)?;
                }

                // Bitwise operations
                AND => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b & a)?;
                }
                OR => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b | a)?;
                }
                XOR => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b ^ a)?;
                }
                NOT => {
                    let a = self.pop()?;
                    self.push(!a)?;
                }
                SHR => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b >> a)?;
                }
                SHL => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(b << a)?;
                }

                // Stack
                POP => {
                    self.pop()?;
                }
                DUP => {
                    self.assert_stack_free_space(1)?;
                    self.assert_stack_size(1)?;
                    let v = self.stack[self.sp - 1];
                    self.stack[self.sp] = v;
                    self.sp += 1;
                }
                SWP => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(a)?;
                    self.push(b)?;
                }
                OVR => {
                    self.assert_stack_size(2)?;
                    let v = self.stack[self.sp - 2];
                    self.push(v)?;
                }

                // Memory
                LOAD => {
                    let addr = self.pop()? as usize;
                    self.assert_memory_address(addr)?;
                    let v = self.memory[addr];
                    self.push(v)?;
                }
                STOR => {
                    let addr = self.pop()? as usize;
                    let v = self.pop()?;
                    self.assert_memory_address(addr)?;
                    self.memory[addr] = v;
                }

                // Jumps
                JMP => {
                    let addr = self.pop()? as usize;
                    self.assert_memory_address(addr)?;
                    self.ip = addr - 1;
                }
                JE => {
                    let addr = self.pop()? as usize;
                    self.assert_memory_address(addr)?;
                    if self.pop()? == self.pop()? {
                        self.ip = addr - 1;
                    }
                }
                JNE => {
                    let addr = self.pop()? as usize;
                    self.assert_memory_address(addr)?;
                    if self.pop()? != self.pop()? {
                        self.ip = addr - 1;
                    }
                }
                JG => {
                    let addr = self.pop()? as usize;
                    self.assert_memory_address(addr)?;
                    if self.pop()? > self.pop()? {
                        self.ip = addr - 1;
                    }
                }
                JGE => {
                    let addr = self.pop()? as usize;
                    self.assert_memory_address(addr)?;
                    if self.pop()? >= self.pop()? {
                        self.ip = addr - 1;
                    }
                }
                JL => {
                    let addr = self.pop()? as usize;
                    self.assert_memory_address(addr)?;
                    if self.pop()? < self.pop()? {
                        self.ip = addr - 1;
                    }
                }
                JLE => {
                    let addr = self.pop()? as usize;
                    self.assert_memory_address(addr)?;
                    if self.pop()? <= self.pop()? {
                        self.ip = addr - 1;
                    }
                }

                // Flags
                RF => {
                    self.rf = true;
                }
                CRF => {
                    self.rf = false;
                }

                // Other
                HALT => {
                    self.hf = true;
                }
                NOP => {}
                unk => return Err(VMError::UnknownInstruction(unk)),
            }
        }
        Ok(())
    }

    fn push(&mut self, v: i32) -> Result<(), VMError> {
        self.assert_stack_free_space(1)?;
        self.stack[self.sp] = v;
        self.sp += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<i32, VMError> {
        self.assert_stack_size(1)?;
        let v = self.stack[self.sp - 1];
        self.sp -= 1;
        Ok(v)
    }

    fn check_stack_free_space(&self, min: usize) -> bool {
        self.stack.len() - self.sp >= min
    }

    fn assert_stack_free_space(&self, min: usize) -> Result<(), VMError> {
        if !self.check_stack_free_space(min) {
            Err(VMError::StackOverflow)
        } else {
            Ok(())
        }
    }

    fn check_stack_size(&self, min: usize) -> bool {
        self.sp >= min
    }

    fn assert_stack_size(&self, min: usize) -> Result<(), VMError> {
        if !self.check_stack_size(min) {
            Err(VMError::CorruptStack)
        } else {
            Ok(())
        }
    }

    fn check_memory_address(&self, addr: usize) -> bool {
        addr < self.memory.len()
    }

    fn assert_memory_address(&self, addr: usize) -> Result<(), VMError> {
        if !self.check_memory_address(addr) {
            Err(VMError::InvalidMemoryAddress)
        } else {
            Ok(())
        }
    }

    fn print_stack(&self) {
        if self.sp == 0 {
            return;
        }
        print!("[");
        for v in &self.stack[0..self.sp - 1] {
            print!("{v}, ");
        }
        print!("{}", self.stack[self.sp - 1]);
        println!("]");
    }
}
