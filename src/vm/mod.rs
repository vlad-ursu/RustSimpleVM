mod instructions;
use instructions::OP;

macro_rules! slice_as_array_ref {
    ($slice:expr, $len:expr) => {
        {
            fn slice_as_array_ref<T>(slice: &[T])
                                     -> Result<&[T; $len], String> {
                if slice.len() != $len {
                    return Err(String::from("Unspecified"));
                }
                Ok(unsafe {
                    &*(slice.as_ptr() as *const [T; $len])
                })
            }
            slice_as_array_ref($slice)
        }
    }
}

pub struct VM<'a> {
    bytecode: &'a [u8],
    ip: usize,
    stack: Vec<i64>
}

impl VM<'_> {
    pub fn new(code: &[u8]) -> VM {
        VM {
            bytecode: code,
            ip: 0,
            stack: Vec::with_capacity(1_000_000) // 4MB
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.bytecode.len() {
            let op = self.bytecode[self.ip];
            match OP::from_u8(op) {
                Some(OP::NOP) => self.ip += 1,
                Some(OP::PUSH) => {
                    self.ip += 1;
                    let i64_bytes = slice_as_array_ref!(&self.bytecode[self.ip..self.ip+8], 8);
                    self.ip += 8;

                    match i64_bytes {
                        Ok(x) => {
                            let value = i64::from_le_bytes(*x);
                            self.stack.push(value);
                        },
                        Err(e) => panic!(e)
                    }
                },
                Some(OP::DUP) => {
                    match self.stack.last(){
                        Some(x) => {
                            let x = *x;
                            self.stack.push(x);
                        }
                        None => panic!("Error on DUP. Stack is empty")
                    }
                    self.ip += 1;
                },
                Some(OP::PRNT) => {
                    let value = self.stack.pop();
                    match value {
                        Some(x) => println!("{}", x),
                        None => {println!("???")}
                    }
                    self.ip += 1;
                },
                _ => {panic!("Invalid instruction")}
            }
        }
    }
}
