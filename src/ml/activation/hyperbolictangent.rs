use std::f64;
use itertools::Itertools;

use super::{Activation, CalcMod};

#[derive(Copy, Clone)]
pub struct HyperbolicTangent;

impl HyperbolicTangent {
    pub fn new() -> HyperbolicTangent {
        return HyperbolicTangent;
    }
}

impl Activation for HyperbolicTangent {
    /// Calculates the tanh of input `x`
    fn calc(&self, x: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        x.iter().map(|n| n.tanh()).collect::<Vec<_>>()
    }

    /// Calculates the Derivative tanh of input `x`
    fn derivative(&self, x: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        x.iter()
        .map(|n| {
            let tanh_factor = n.tanh();
            1f64 - (tanh_factor * tanh_factor)
        })
        .collect::<Vec<_>>()
    }
}


#[cfg(test)]
mod tests {
    use super::Activation;
    use super::HyperbolicTangent;

}
