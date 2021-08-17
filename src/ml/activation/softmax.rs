use std::panic::UnwindSafe;

use crate::log;

use super::{Activation, CalcMod};

#[derive(Copy, Clone)]
pub struct SoftMax;

impl SoftMax {
    pub fn new() -> SoftMax {
        return SoftMax;
    }
}

impl Activation for SoftMax {
    /// Calculates the SoftMax of input `x`
    fn calc(&self, x: Vec<f64>, calc_mode: &CalcMod) -> Vec<f64> {
        match calc_mode {
            CalcMod::Normal => {
                let max_x = x
                    .iter()
                    .cloned()
                    .max_by(|x, y| x.partial_cmp(&y).unwrap())
                    .unwrap();

                let exps = x
                    .iter()
                    .cloned()
                    .map(|n| n.exp() - max_x)
                    .collect::<Vec<_>>();

                let exp_sum: f64 = exps.iter().clone().sum();

                exps.iter().map(|x| x / exp_sum).collect::<Vec<f64>>()
            }
            CalcMod::SIMD128 => {
                enum DataType{
                    Pair(core::arch::wasm32::v128), 
                    Alone(core::arch::wasm32::v128)
                }
                let max_x = x
                    .iter()
                    .cloned()
                    .max_by(|x, y| x.partial_cmp(&y).unwrap())
                    .unwrap();

                let exps = x
                    .chunks(2)
                    .map(|n| {
                        // n.exp() - max_x
                        if n.len()==2{
                            let v=core::arch::wasm32::f64x2(n[0].exp(),n[1].exp());
                            let maxes = core::arch::wasm32::f64x2(max_x,max_x);
                            DataType::Pair(core::arch::wasm32::f64x2_sub(v,maxes))
                        }else{
                            let v=core::arch::wasm32::f64x2(n[0].exp(),0f64);
                            let maxes = core::arch::wasm32::f64x2(max_x,max_x);
                            DataType::Alone(core::arch::wasm32::f64x2_sub(v,maxes))
                        }
                        
                    })
                    .flat_map(|x|{
                        match x{
                            DataType::Pair(v) => {
                                let a = core::arch::wasm32::f64x2_extract_lane::<0>(v);
                                let b = core::arch::wasm32::f64x2_extract_lane::<1>(v);
                                vec![a,b]
                            },
                            DataType::Alone(v) =>{
                                let a = core::arch::wasm32::f64x2_extract_lane::<0>(v);
                                vec![a]
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                let exp_sum: f64 = exps.iter().clone().sum();

                exps.chunks(2).map(|x| {
                    // x / exp_sum
                    if x.len()==2{
                        let v=core::arch::wasm32::f64x2(x[0],x[1]);
                        let exp = core::arch::wasm32::f64x2(exp_sum,exp_sum);
                        DataType::Pair(core::arch::wasm32::f64x2_div(v,exp))
                    }else{
                        let v=core::arch::wasm32::f64x2(x[0].exp(),0f64);
                        let exp = core::arch::wasm32::f64x2(exp_sum,exp_sum);
                        DataType::Alone(core::arch::wasm32::f64x2_div(v,exp))
                    }
                }).flat_map(|x|{
                    match x{
                        DataType::Pair(v) => {
                            let a = core::arch::wasm32::f64x2_extract_lane::<0>(v);
                            let b = core::arch::wasm32::f64x2_extract_lane::<1>(v);
                            vec![a,b]
                        },
                        DataType::Alone(v) =>{
                            let a = core::arch::wasm32::f64x2_extract_lane::<0>(v);
                            vec![a]
                        }
                    }
                })
                .collect::<Vec<f64>>()
            }
        }
    }

    /// Calculates the Derivative SoftMax of input `x`
    fn derivative(&self, x: Vec<f64>, calc_mode: &CalcMod) -> Vec<f64> {
        let softmaxed = self.calc(x.clone(), calc_mode);

        softmaxed
            .clone()
            .iter()
            .map(|n| n * (1f64 - n))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Activation;
    use super::SoftMax;
}
