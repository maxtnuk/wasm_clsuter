use super::{Activation, CalcMod};

#[derive(Copy, Clone)]
pub struct Sigmoid;

impl Sigmoid {
    pub fn new() -> Sigmoid {
        return Sigmoid;
    }
}

impl Activation for Sigmoid {
    /// Calculates the Sigmoid of input `x`
    fn calc(&self, x: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        x.iter()
            .map(|n| 1f64 / (1f64 + (-n).exp()))
            .collect::<Vec<_>>()
    }

    /// Calculates the Derivative Sigmoid of input `x`
    fn derivative(&self, x: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        x.iter().map(|n| n * (1f64 - n)).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Activation;
    use super::Sigmoid;

}
