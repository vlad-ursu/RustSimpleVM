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
    fn unary_op_with_arg<U>(&mut self, condition: U) where U: Fn(i64) -> bool {
        match self.stack.last() {
            Some(tos) => {
                if condition(*tos) {
                    self.ip += 1;
                    let i64_bytes = slice_as_array_ref!(&self.bytecode[self.ip..self.ip+8], 8);
                    // self.ip += 8;

                    match i64_bytes {
                        Ok(addr) => {
                            let addr = i64::from_le_bytes(*addr) as usize;
                            self.ip = addr;
                        },
                        Err(e) => panic!(e)
                    }
                } else {
                    self.ip += 1;
                }
            },
            None => panic!("Empty stack.")
        }
    }

    fn unary_op_inplace<UI>(&mut self, operation: UI) where UI: Fn(&mut i64) {
        match self.stack.last_mut() {
            Some(tos) => operation(tos),
            None => panic!("Empty stack.")
        }
        self.ip += 1;
    }

    fn unary_op_with_arg_inplace<UAI>(&mut self, operation: UAI) where UAI: Fn(&mut i64, usize) {
        self.ip += 1;
        let i64_bytes = slice_as_array_ref!(&self.bytecode[self.ip..self.ip+8], 8);
        self.ip += 8;

        match i64_bytes {
            Ok(delta) => {
                let delta = i64::from_le_bytes(*delta) as usize;
                match self.stack.last_mut() {
                    Some(tos) => operation(tos, delta),
                    None => panic!("Empty stack.")
                }
            },
            Err(e) => panic!(e)
        }
    }

    fn binary_op<B>(&mut self, operation: B) where B: Fn(i64, i64) -> i64 {
        let tos = self.stack.pop();
        
        match tos {
            Some(tos) => {
                let tos1 = self.stack.pop();
                match tos1 {
                    Some(tos1) => self.stack.push(operation(tos, tos1)),
                    None => panic!("Empty stack.")
                }
            },
            None => panic!("Empty stack.")
        }

        self.ip += 1;
    } 

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
            // println!("{}", op);
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
                Some(OP::POP) => {
                    self.stack.pop();
                    self.ip += 1;
                },
                Some(OP::CMP) => {
                    match self.stack.pop() {
                        Some(tos) => {
                            match self.stack.pop() {
                                Some(tos1) => {
                                    if tos1 < tos {
                                        self.stack.push(-1);
                                    } else if tos1 == tos {
                                        self.stack.push(0);
                                    } else {
                                        self.stack.push(1);
                                    }
                                },
                                None => panic!("Error on CMP. Stack is empty")
                            }
                        },
                        None => panic!("Error on CMP. Stack is empty")
                    }
                    self.ip += 1;
                },
                Some(OP::JMP) => {
                    self.ip += 1;
                    let i64_bytes = slice_as_array_ref!(&self.bytecode[self.ip..self.ip+8], 8);

                    match i64_bytes {
                        Ok(addr) => {
                            let addr = i64::from_le_bytes(*addr) as usize;
                            self.ip = addr;
                        },
                        Err(e) => panic!(e)
                    }
                },
                Some(OP::JE) => self.unary_op_with_arg( |x: i64| { x == 0 } ),
                Some(OP::JNE) => self.unary_op_with_arg( |x: i64| { x != 0 } ),
                Some(OP::JL) => self.unary_op_with_arg( |x: i64| { x == -1 } ),
                Some(OP::JLE) => self.unary_op_with_arg( |x: i64| { x == -1 || x == 0 } ),
                Some(OP::JG) => self.unary_op_with_arg( |x: i64| { x == 1 } ),
                Some(OP::JGE) => self.unary_op_with_arg( |x: i64| { x == 0 || x == 1 } ),
                Some(OP::NOT) => self.unary_op_inplace( |x: &mut i64| { *x = !*x } ),
                Some(OP::NEG) => self.unary_op_inplace( |x: &mut i64| { *x = -*x } ),
                Some(OP::SHL) => self.unary_op_with_arg_inplace( |x: &mut i64, y: usize| { *x = *x << y } ),
                Some(OP::SHR) => self.unary_op_with_arg_inplace( |x: &mut i64, y: usize| { *x = *x >> y } ),
                Some(OP::INC) => self.unary_op_inplace( |x: &mut i64| { *x += 1 } ),
                Some(OP::DEC) => self.unary_op_inplace( |x: &mut i64| { *x -= 1 } ),
                Some(OP::AND) => self.binary_op( |x, y: i64| { x & y } ),
                Some(OP::OR) => self.binary_op( |x, y: i64| { x | y } ),
                Some(OP::XOR) => self.binary_op( |x, y: i64| { x ^ y } ),
                Some(OP::ADD) => self.binary_op( |x, y: i64| { x + y } ),
                Some(OP::SUB) => self.binary_op( |x, y: i64| { y - x } ),
                Some(OP::MUL) => self.binary_op( |x, y: i64| { x * y } ),
                Some(OP::DIV) => self.binary_op( |x, y: i64| { 
                    if x == 0 {
                        panic!("Division by 0.");
                    } else {
                        y / x
                    }
                } ),
                Some(OP::MOD) => self.binary_op( |x, y: i64| { 
                    if x == 0 {
                        panic!("Division by 0.");
                    } else {
                        y % x
                    }
                } ),
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
