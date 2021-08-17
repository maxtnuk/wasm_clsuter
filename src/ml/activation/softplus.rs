use std::f64;
use super::{Activation, CalcMod};

#[derive(Copy, Clone)]
pub struct SoftPlus;

impl SoftPlus {
    pub fn new() -> SoftPlus {
        return SoftPlus;
    }
}

impl Activation for SoftPlus {
    /// Calculates the SoftPlus of input `x`
    fn calc(&self, x: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        x.iter().map(|n| (1f64 + n.exp()).ln()).collect::<Vec<_>>()
    }

    /// Calculates the Derivative SoftPlus of input `x`
    fn derivative(&self, x: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        x.iter()
            .map(|n| 1f64 / (1f64 + (-n).exp()))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Activation;
    use super::SoftPlus;

  
}
