use super::{Activation, CalcMod};

#[derive(Copy, Clone)]
pub struct RectifiedLinearUnit;

impl RectifiedLinearUnit {
    pub fn new() -> RectifiedLinearUnit {
        return RectifiedLinearUnit;
    }
}

impl Activation for RectifiedLinearUnit {
    /// Calculates the RectifiedLinearUnit of input `x`
    fn calc(&self, x: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        x.iter()
            .map(|&n| if n <= 0f64 { 0f64 } else { n })
            .collect::<Vec<_>>()
    }

    /// Calculates the Derivative RectifiedLinearUnit of input `x`
    fn derivative(&self, x: Vec<f64>,calc_mode: &CalcMod) -> Vec<f64> {
        x.iter()
            .map(|&n| if n <= 0f64 { 0f64 } else { n })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Activation;
    use super::RectifiedLinearUnit;

    
}
