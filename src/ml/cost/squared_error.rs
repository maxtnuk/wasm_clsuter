use super::CostFunction;
use crate::ml::matrix::Matrix;
use crate::ml::matrix::MatrixTrait;
use super::CostFunctions;

pub struct SquaredError;

impl SquaredError {
    pub fn new() -> SquaredError {
        SquaredError
    }
}

impl CostFunction for SquaredError {
    fn name(&self) -> CostFunctions {
        CostFunctions::SquaredError
    }

    fn calc(&self, prediction: &Matrix, target: &Matrix) -> f64 {
        let mut errors = Vec::with_capacity(prediction.cols());

        for (i, p) in prediction.row(0).iter().enumerate() {
            errors.push((target.get(0, i) - p).powi(2) / 2f64);
        }

        errors.iter().fold(0f64, |sum, val| sum + val)
    }
}
