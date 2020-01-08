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
            println!("{}", op);
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
                Some(OP::JE) => {
                    match self.stack.last() {
                        Some(tos) => {
                            if *tos == 0 {
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
                        None => panic!("Error on JE. Stack is empty")
                    }
                },
                Some(OP::JNE) => {
                    match self.stack.last() {
                        Some(tos) => {
                            if *tos != 0 {
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
                        None => panic!("Error on JE. Stack is empty")
                    }
                },
                Some(OP::JL) => {
                    match self.stack.last() {
                        Some(tos) => {
                            if *tos == -1 {
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
                        None => panic!("Error on JE. Stack is empty")
                    }
                },
                Some(OP::JLE) => {
                    match self.stack.last() {
                        Some(tos) => {
                            if *tos == -1 || *tos == 0 {
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
                        None => panic!("Error on JE. Stack is empty")
                    }
                },
                Some(OP::JG) => {
                    match self.stack.last() {
                        Some(tos) => {
                            if *tos == 1 {
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
                        None => panic!("Error on JE. Stack is empty")
                    }
                },
                Some(OP::JGE) => {
                    match self.stack.last() {
                        Some(tos) => {
                            if *tos == 0 || *tos == 1 {
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
                        None => panic!("Error on JE. Stack is empty")
                    }
                },
                Some(OP::NOT) => {
                    match self.stack.last_mut() {
                        Some(tos) => *tos = !*tos,
                        None => panic!("Error on JE. Stack is empty")
                    }
                    self.ip += 1;
                },
                Some(OP::NEG) => {
                    match self.stack.last_mut() {
                        Some(tos) => *tos = -*tos,
                        None => panic!("Error on JE. Stack is empty")
                    }
                    self.ip += 1;
                },
                Some(OP::SHL) => {
                    self.ip += 1;
                    let i64_bytes = slice_as_array_ref!(&self.bytecode[self.ip..self.ip+8], 8);
                    self.ip += 8;

                    match i64_bytes {
                        Ok(delta) => {
                            let delta = i64::from_le_bytes(*delta) as usize;
                            match self.stack.last_mut() {
                                Some(tos) => *tos = *tos << delta,
                                None => panic!("Error on SHL. Stack is empty")
                            }
                        },
                        Err(e) => panic!(e)
                    }
                },
                Some(OP::SHR) => {
                    self.ip += 1;
                    let i64_bytes = slice_as_array_ref!(&self.bytecode[self.ip..self.ip+8], 8);
                    self.ip += 8;

                    match i64_bytes {
                        Ok(delta) => {
                            let delta = i64::from_le_bytes(*delta) as usize;
                            match self.stack.last_mut() {
                                Some(tos) => *tos = *tos >> delta,
                                None => panic!("Error on SHL. Stack is empty")
                            }
                        },
                        Err(e) => panic!(e)
                    }
                },
                Some(OP::INC) => {
                    match self.stack.last_mut() {
                        Some(tos) => *tos += 1,
                        None => panic!("Error on INC. Stack is empty")
                    }
                    self.ip += 1;
                },
                Some(OP::DEC) => {
                    match self.stack.last_mut() {
                        Some(tos) => *tos -= 1,
                        None => panic!("Error on DEC. Stack is empty")
                    }
                    self.ip += 1;
                },
                Some(OP::AND) => {
                    let tos = self.stack.pop();

                    match tos {
                        Some(tos) => {
                            let tos1 = self.stack.pop();
                            match tos1 {
                                Some(tos1) => self.stack.push(tos & tos1),
                                None => panic!("Error on AND. Stack is empty")
                            }
                        },
                        None => panic!("Error on AND. Stack is empty")
                    }

                    self.ip += 1;
                },
                Some(OP::OR) => {
                    let tos = self.stack.pop();

                    match tos {
                        Some(tos) => {
                            let tos1 = self.stack.pop();
                            match tos1 {
                                Some(tos1) => self.stack.push(tos | tos1),
                                None => panic!("Error on OR. Stack is empty")
                            }
                        },
                        None => panic!("Error on OR. Stack is empty")
                    }

                    self.ip += 1;
                },
                Some(OP::XOR) => {
                    let tos = self.stack.pop();

                    match tos {
                        Some(tos) => {
                            let tos1 = self.stack.pop();
                            match tos1 {
                                Some(tos1) => self.stack.push(tos ^ tos1),
                                None => panic!("Error on XOR. Stack is empty")
                            }
                        },
                        None => panic!("Error on XOR. Stack is empty")
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
