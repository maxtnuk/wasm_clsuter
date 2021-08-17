

use std::{cell::RefCell, rc::Rc};

use cluster::{ClusterConfig, Color, NN, json_to_dataset};
use itertools::iterate;
use ml::{activation::CalcMod, matrix::MatrixTrait, sample::Sample};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};
use wasm_bindgen::JsCast;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

mod cluster;
#[macro_use]
mod ml;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
// map data gen 
fn test_data_gen(map_ratio: u32)->Vec<Sample>{
    let (x_max ,x_min )= (3f64,-3f64);
    let x_step = (x_max-x_min) / (map_ratio as f64);

    let (y_max ,y_min )= (3f64,-3f64);
    let y_step = (y_max-y_min) / (map_ratio as f64);

    let mut samples =Vec::new();
    for n in iterate(y_max, |i| i - y_step).take_while(|&i| i >= y_min) {
        for m in iterate(x_min, |j| j + x_step).take_while(|&j| j <= x_max) {
            samples.push(s_input![m, n])
        }
    }
    samples
}

// without simd
#[wasm_bindgen]
pub fn train(
    data_json: String,
    map_ratio: u32,
    config: ClusterConfig,
    ctx: &CanvasRenderingContext2d,
    edge: u32,
    margin: u32,
    color_list: String
) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let base = margin as f64;
    let rect = ((edge - 2 * margin) / map_ratio) as f64;

    let mut network = NN::new(&config);

    let dataset = json_to_dataset(config.classes as usize,data_json);
    let calc_mode = CalcMod::Normal;
    let calc_cloned=calc_mode.clone();

    network.on_error(move |err| {
        log(format!("error :{}",err).as_str());
    });
    let ctx_clone  =  ctx.clone();
    let color_list= serde_json::from_str::<Vec<Color>>(color_list.as_str()).unwrap();
    let epoch_dataset = test_data_gen(map_ratio);
   
    network.on_epoch(move |nn| {
        let ctx = &ctx_clone;
        ctx.clear_rect(0f64,0f64,edge.into(),edge.into());
        epoch_dataset.iter().map(|sample|{
            let eval_value = nn.evaluate(sample,&calc_mode);
            let first_val = eval_value.body().get(0).unwrap();
            first_val
                .iter()
                .cloned()
                .enumerate()
                .max_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap())
                .unwrap()
        }).enumerate().for_each(|(i,(n,alpha))|{
            let mut color = color_list[n].clone();
            color.alpha= alpha;
            let color_string = JsValue::from(color.color_str());
            ctx.set_fill_style(&color_string);
            let x = (i % (map_ratio as usize)) as f64;
            let y =(i /(map_ratio as usize)) as f64;
            ctx.fill_rect(base + x * rect, base + y * rect, rect, rect);
        });
    });
    network.train(dataset,calc_cloned);

    Ok(())
}
