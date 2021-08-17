use super::CostFunction;
use crate::ml::matrix::Matrix;
use crate::ml::matrix::MatrixTrait;
use super::CostFunctions;
use std::f64;

pub struct CrossEntropy;

impl CrossEntropy {
    pub fn new() -> CrossEntropy {
        CrossEntropy
    }
}

impl CostFunction for CrossEntropy {
    fn name(&self) -> CostFunctions {
        CostFunctions::CrossEntropy
    }

    fn calc(&self, prediction: &Matrix, target: &Matrix) -> f64 {
        let eps: f64 = f64::EPSILON;

        let clipped_pred = prediction
            .row(0)
            .iter()
            .map(|n| {
                let mut r = *n;

                if *n < eps {
                    r = eps;
                } else if *n > 1f64 - eps {
                    r = 1f64 - eps;
                }

                r
            })
            .collect::<Vec<_>>();

        // log(prediction)
        let prediction_log = clipped_pred
            .iter()
            .map(|n| n.log(f64::consts::E))
            .collect::<Vec<_>>();

        // target - 1
        let target_neg = target.row(0).iter().map(|n| 1f64 - n).collect::<Vec<_>>();

        // log(prediction - 1)
        let prediction_neg_log = clipped_pred
            .iter()
            .map(|n| (1f64 - n).log(f64::consts::E))
            .collect::<Vec<_>>();

        // cost
        let cost = target.row(0).iter().enumerate().map(|(i, n)| {
            ((n * prediction_log[i]) + (target_neg[i] * prediction_neg_log[i])) * -1f64
        });

        // mean
        cost.fold(0f64, |sum, val| sum + val) / target.cols() as f64
    }
}

#[cfg(test)]
mod tests {
  


}
