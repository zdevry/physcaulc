use super::{EvaluationError, NodeError};

impl NodeError {
    pub fn to_evalerr(&self, s: &str) -> EvaluationError {
        EvaluationError {
            content: self.content.clone(),
            start: self.start,
            end: self.end,
            evalstr: s.into(),
        }
    }
}
