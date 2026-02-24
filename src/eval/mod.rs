mod err;
mod node;

use crate::value::{SIDimension, Value, ValueError};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Negative,
    Units(f64, SIDimension),
}

#[derive(Debug, Clone)]
pub enum NodeContent {
    Binary(Box<Node>, BinaryOp, Box<Node>),
    Unary(UnaryOp, Box<Node>),
    Function(String, Vec<Node>),
    Value(Value),
    Variable(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub content: NodeContent,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub enum NodeErrorContent {
    ValueError(ValueError),
    VarNameError(String),
    FuncNameError(String),
    ParamCountError(usize, usize),
    NestedError(String, Box<EvaluationError>),
}

#[derive(Debug, Clone)]
pub struct NodeError {
    pub content: NodeErrorContent,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Evaluator {
    pub parent: Node,
    pub evalstr: String,
    pub params: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EvaluationError {
    pub content: NodeErrorContent,
    pub start: usize,
    pub end: usize,
    pub evalstr: String,
}

#[derive(Debug)]
pub struct Environment {
    pub consts: HashMap<String, Value>,
    pub evaluators: HashMap<String, Evaluator>,
}

impl Evaluator {
    pub fn eval(&self, env: &Environment, params: &[Value]) -> Result<Value, EvaluationError> {
        // it is assumed that the amount of variables passed is correct
        let labelled_params =
            HashMap::from_iter(self.params.iter().cloned().zip(params.iter().cloned()));

        self.parent
            .eval(env, &labelled_params)
            .map_err(|e| e.to_evalerr(&self.evalstr))
    }
}
