use crate::{
    eval::{
        BinaryOp, Environment, Node, NodeContent, NodeError, NodeErrorContent, UnaryOp, UnitTerm,
    },
    f64plus::FloatPlus,
    value::{Quantity, SIDimension, Value},
};
use std::collections::HashMap;

impl Node {
    pub fn eval(
        &self,
        env: &Environment,
        params: &HashMap<String, Value>,
    ) -> Result<Value, NodeError> {
        match &self.content {
            NodeContent::Binary(lhs, op, rhs) => {
                eval_binary(lhs, rhs, *op, env, params, self.start, self.end)
            }
            NodeContent::Unary(op, operand) => {
                eval_unary(operand, op, env, params, self.start, self.end)
            }
            NodeContent::Function(func, param_nodes) => {
                eval_func(func, param_nodes, env, params, self.start, self.end)
            }
            NodeContent::Value(val) => Ok(val.clone()),
            NodeContent::Variable(var) => eval_var(var, env, params, self.start, self.end),
        }
    }
}

fn eval_func(
    func: &str,
    param_nodes: &[Node],
    env: &Environment,
    params: &HashMap<String, Value>,
    start: usize,
    end: usize,
) -> Result<Value, NodeError> {
    let evaluator = env.evaluators.get(func).ok_or_else(|| NodeError {
        content: NodeErrorContent::FuncNameError(func.into()),
        start,
        end,
    })?;

    if evaluator.params.len() != param_nodes.len() {
        return Err(NodeError {
            content: NodeErrorContent::ParamCountError(evaluator.params.len(), param_nodes.len()),
            start,
            end,
        });
    }

    let param_values = param_nodes
        .iter()
        .map(|node| node.eval(env, params))
        .collect::<Result<Vec<Value>, NodeError>>()?;

    evaluator.eval(env, &param_values).map_err(|e| NodeError {
        content: NodeErrorContent::NestedError(func.into(), Box::new(e)),
        start,
        end,
    })
}

fn eval_unary(
    operand: &Node,
    op: &UnaryOp,
    env: &Environment,
    params: &HashMap<String, Value>,
    start: usize,
    end: usize,
) -> Result<Value, NodeError> {
    let operand_value = operand.eval(env, params)?;
    match op {
        UnaryOp::Negative => Ok(operand_value.negative()),
        UnaryOp::Units(units) => {
            let conversion = eval_unit_factors(&units, env)?;
            Ok(operand_value.mul(&conversion.into()).unwrap())
        }
    }
}

fn eval_unit_factors(units: &[UnitTerm], env: &Environment) -> Result<Quantity, NodeError> {
    let mut result_factor = 1.0;
    let mut result_dim = SIDimension::DIMLESS;

    for term in units {
        let conversion = env.units.get(&term.unit).ok_or_else(|| NodeError {
            content: NodeErrorContent::UnitNameError(term.unit.clone()),
            start: term.start,
            end: term.end,
        })?;

        result_factor *= conversion.factor.powf(term.power.into());
        result_dim = result_dim.mul(&conversion.dim.pow(term.power));
    }

    Ok(Quantity {
        value: FloatPlus::Scalar(result_factor),
        derivatives: HashMap::new(),
        dim: result_dim,
    })
}

fn eval_binary(
    lhs: &Node,
    rhs: &Node,
    op: BinaryOp,
    env: &Environment,
    params: &HashMap<String, Value>,
    start: usize,
    end: usize,
) -> Result<Value, NodeError> {
    let left = lhs.eval(env, params)?;
    let right = rhs.eval(env, params)?;

    match op {
        BinaryOp::Add => left.add(&right),
        BinaryOp::Sub => left.sub(&right),
        BinaryOp::Mul => left.mul(&right),
        BinaryOp::Div => left.div(&right),
        BinaryOp::Pow => left.pow(&right),
    }
    .map_err(|e| NodeError {
        content: NodeErrorContent::ValueError(e),
        start,
        end,
    })
}

fn eval_var(
    var: &str,
    env: &Environment,
    params: &HashMap<String, Value>,
    start: usize,
    end: usize,
) -> Result<Value, NodeError> {
    let result = params.get(var).or_else(|| env.consts.get(var));

    match result {
        Some(v) => Ok(v.clone()),
        None => Err(NodeError {
            content: NodeErrorContent::VarNameError(var.into()),
            start,
            end,
        }),
    }
}
