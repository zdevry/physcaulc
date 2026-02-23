use super::{EvaluationError, NodeError, NodeErrorContent};

impl NodeError {
    pub fn to_evalerr(&self, s: &str) -> EvaluationError {
        EvaluationError {
            content: self.content.clone(),
            strpos: self.strpos,
            evalstr: s.into(),
        }
    }

    pub fn name(var: &str, pos: usize) -> Self {
        Self {
            strpos: pos,
            content: NodeErrorContent::NameError(var.into()),
        }
    }
}
