pub enum Opcode {
    NOP,
    ADD(ArithmeticTarget),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
