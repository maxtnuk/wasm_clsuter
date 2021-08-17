use super::{Activation, CalcMod};

#[derive(Copy, Clone)]
pub struct Identity;

impl Identity {
    pub fn new() -> Identity {
        return Identity;
    }
}

impl Activation for Identity {
    /// Calculates the Identity of input `x`
    fn calc(&self, x: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        x
    }

    /// Calculates the Derivative Identity of input `x`
    fn derivative(&self, v: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        v.iter().map(|_| 1f64).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Activation;
    use super::Identity;
}
