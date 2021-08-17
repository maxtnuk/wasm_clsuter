use wasm_bindgen::prelude::*;
use crate::{log, ml::{activation::{CalcMod, HyperbolicTangent, RectifiedLinearUnit, Sigmoid, SoftMax, SoftPlus}, cost::cross_entropy::CrossEntropy, nl::NeuralLayer, nn::NeuralNetwork, sample::Sample}};

use serde::Deserialize;

#[wasm_bindgen]
#[derive(Debug,Deserialize)]
pub struct Point {
    #[serde(rename(deserialize = "X"))]
    x: f64,
    #[serde(rename(deserialize = "Y"))]
    y: f64,
    #[serde(rename(deserialize = "Class"))]
    class: u32,
}
#[wasm_bindgen]
#[derive(Debug,Deserialize,Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    #[serde(skip)]
    pub alpha: f64
}

impl Color{
    pub fn color_str(&self)->String{
        format!("rgba({}, {}, {}, {})",self.red,self.green,self.blue,self.alpha)
    }
}

#[wasm_bindgen]
#[derive(Clone,Default)]
pub struct ClusterConfig{
    pub classes: u32,
    pub epochs: i32,
    pub learning_rate: f64,
}
#[wasm_bindgen]
impl ClusterConfig{
    pub fn new()->Self {
        Self::default()
    }
}

pub struct NN {
    pub(crate) network: NeuralNetwork,
    pub(crate) config: ClusterConfig
}

fn get_point_class(total_classes: usize,class: usize) -> Vec<f64> {
    let mut one_hot_vec = Vec::new();
    for i in 0..total_classes{
        one_hot_vec.push(if i==class{1f64}else{0f64});
    }
    // log(format!("class: {} vec:{:?}",class,one_hot_vec).as_str());
    one_hot_vec
}

pub(crate) fn json_to_dataset(classes: usize,data_string: String) -> Vec<Sample> {

    let raw_data=serde_json::from_str::<Vec<Point>>(&data_string).unwrap();
    
    raw_data.iter().map(|point| {
        Sample::new(
            vec![point.x, point.y],
            get_point_class(classes,point.class as usize),
        )
    }).collect::<Vec<_>>()
}

impl NN {
    pub fn new(config: &ClusterConfig) -> NN {
        let mut neural_network = NeuralNetwork::new();

        neural_network.add_layer(NeuralLayer::new(4, 2, HyperbolicTangent::new()));
        neural_network.add_layer(NeuralLayer::new(4, 4, RectifiedLinearUnit::new()));
        neural_network.add_layer(NeuralLayer::new(config.classes as usize, 4, SoftMax::new()));

        neural_network.set_cost_function(CrossEntropy);

        NN { 
            network: neural_network,
            config: config.clone()
        }
    }
    pub fn on_epoch<FN>(&mut self, callback_fn: FN)
    where
        FN: 'static + Fn(&NeuralNetwork),
    {
        self.network.on_epoch(callback_fn);
    }

    pub fn on_error<FN>(&mut self, callback_fn: FN)
    where
        FN: 'static + Fn(f64),
    {
        self.network.on_error(callback_fn);
    }

    pub fn train(&mut self,dataset:Vec<Sample>,calc_mod: CalcMod){
        self.network.train(dataset, self.config.epochs, self.config.learning_rate,calc_mod);
    }
}
