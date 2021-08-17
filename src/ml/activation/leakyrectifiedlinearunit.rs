use super::{Activation, CalcMod};

#[derive(Copy, Clone)]
pub struct LeakyRectifiedLinearUnit {
    alpha_gradient: f64,
}

impl LeakyRectifiedLinearUnit {
    pub fn new(alpha: f64) -> LeakyRectifiedLinearUnit {
        return LeakyRectifiedLinearUnit {
            alpha_gradient: alpha,
        };
    }
}

impl Activation for LeakyRectifiedLinearUnit {
    /// Calculates the LeakyRectifiedLinearUnit of input `x`
    fn calc(&self, x: Vec<f64>, calc_mode: &CalcMod) -> Vec<f64> {
        x.iter()
            .map(|&n| {
                if n <= 0f64 {
                    self.alpha_gradient * n
                } else {
                    n
                }
            })
            .collect::<Vec<_>>()
    }

    /// Calculates the Derivative LeakyRectifiedLinearUnit of input `x`
    fn derivative(&self, x: Vec<f64>, calc_mode: &CalcMod) -> Vec<f64> {
        x.iter()
            .map(|&n| if n <= 0f64 { self.alpha_gradient } else { n })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Activation;
    use super::LeakyRectifiedLinearUnit;
}
