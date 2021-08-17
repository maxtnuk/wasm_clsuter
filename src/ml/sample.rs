/// A sample of the given dataset
use wasm_bindgen::prelude::*;
#[derive(Clone,Debug)]
pub struct Sample {
    pub inputs: Vec<f64>,
    pub outputs: Option<Vec<f64>>,
}

macro_rules! s_input {
    [$($x:expr),*] => {
        {
            let input= vec![$($x),*];
            Sample{
                inputs: input,
                outputs: None 
            }
        }
    };
}

impl Sample {
    pub fn new(inputs: Vec<f64>, outputs: Vec<f64>) -> Sample {
        Sample {
            inputs: inputs,
            outputs: Some(outputs),
        }
    }

    pub fn get_inputs_count(&self) -> usize {
        self.inputs.len()
    }

    pub fn get_outputs_count(&self) -> usize {
        match &self.outputs {
            &Some(ref outputs) => outputs.len(),
            &None => 0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inputs_count() {
        let sample = Sample::new(vec![1f64, 0f64], vec![0f64]);
        assert_eq!(sample.get_inputs_count(), 2);
    }

    #[test]
    fn outputs_count() {
        let sample = Sample::new(vec![1f64, 0f64], vec![0f64]);
        assert_eq!(sample.get_outputs_count(), 1);
    }
}
