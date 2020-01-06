#[repr(u8)]
pub enum OP {
    NOP,    // No instruction
    PUSH,   // PUSH value on stack
    DUP,    // PUSH value from top of stack again
    POP,    // POP value from stack
    CMP,    // POP(v1); POP(v2); PUSH(if v1 < v2 => -1 else if v1 == v2 => 0 else if v1 > v2 => 1)
    JMP,    // JUMP to addr
    JE,     // JUMP to addr if top of stack is 0
    JNE,    // JUMP to addr if top of stack != 0
    JL,     // JUMP to addr if top of stack is -1
    JLE,    // JUMP to addr if top of stack is -1 or 0
    JG,     // JUMP to addr if top of stack is 1
    JGE,    // JUMP to addr if top of stack is 1 or 0
    NOT,    // Apply NOT on topmost element of stack
    NEG,    // Multiply topmost element of stack with -1
    ROL,    // Rotate left the topmost element of the stack
    ROR,    // Rotate right the topmost element of the stack
    SHL,    // Shift left the topmost element of the stack
    SHR,    // Shifth right the topmost element of the stack
    INC,    // Increment the topmost element of the stack
    DEC,    // Decrement the topmost element of the stack
    AND,    // POP(v1); POP(v2); PUSH(v1 & v2);
    OR,     // POP(v1); POP(v2); PUSH(v1 | v2);
    XOR,    // POP(v1); POP(v2); PUSH(v1 ^ v2);
    ADD,    // POP(v1); POP(v2); PUSH(v1 + v2);
    SUB,    // POP(v1); POP(v2); PUSH(v1 - v2);
    MUL,    // POP(v1); POP(v2); PUSH(v1 * v2);
    DIV,    // POP(v1); POP(v2); PUSH(v1 / v2);
    PRNT    // Print topmost value on stack
}

impl OP {
    pub fn from_u8(op: u8) -> Option<OP> {
        match op {
            0 => Some(OP::NOP),
            1 => Some(OP::PUSH),
            2 => Some(OP::DUP),
            3 => Some(OP::POP),
            4 => Some(OP::CMP),
            5 => Some(OP::JMP),
            6 => Some(OP::JE),
            7 => Some(OP::JNE),
            8 => Some(OP::JL),
            9 => Some(OP::JLE),
            10 => Some(OP::JG),
            11 => Some(OP::JGE),
            12 => Some(OP::NOT),
            13 => Some(OP::NEG),
            14 => Some(OP::ROL),
            15 => Some(OP::ROR),
            16 => Some(OP::SHL),
            17 => Some(OP::SHR),
            18 => Some(OP::INC),
            19 => Some(OP::DEC),
            20 => Some(OP::AND),
            21 => Some(OP::OR),
            22 => Some(OP::XOR),
            23 => Some(OP::ADD),
            24 => Some(OP::SUB),
            25 => Some(OP::MUL),
            26 => Some(OP::DIV),
            27 => Some(OP::PRNT),
            _ => None
        }
    }
}
