pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub enum Unary {
    Negative,
}

pub enum NodeContent {
    Binary(Node, Node, BinaryOp),
}

pub struct Node {}
