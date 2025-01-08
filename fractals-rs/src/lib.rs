mod utils;

use hsv::hsv_to_rgb;
use num_complex::Complex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

use web_sys::HtmlCanvasElement;

const MAX_ITERATIONS: u32 = 200;

#[wasm_bindgen]
pub struct C {
    pub real: f64,
    pub imaginary: f64,
}

#[wasm_bindgen]
impl C {
    #[wasm_bindgen(constructor)]
    pub fn new(real: f64, imaginary: f64) -> Self {
        C { real, imaginary }
    }
}

#[wasm_bindgen]
pub struct Selection {
    pub x: f64,
    pub y: f64,
    pub length: f64,
}

#[wasm_bindgen]
impl Selection {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, length: f64) -> Self {
        Selection { x, y, length }
    }
}

#[wasm_bindgen]
pub fn draw(canvas: &HtmlCanvasElement, c_opt: Option<C>) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let width = canvas.width();
    let height = canvas.height();
    let data = get_set(
        width,
        height,
        -1.5,
        1.5,
        -1.5,
        1.5,
        c_opt.map(|c| Complex {
            re: c.real,
            im: c.imaginary,
        }),
    );
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height).unwrap();
    context.put_image_data(&data, 0.0, 0.0).unwrap();
}

fn get_iter_count(z: Complex<f64>, c_opt: Option<Complex<f64>>) -> u32 {
    let mut z = z;
    let c = c_opt.unwrap_or(z.clone());

    for i in 0..=MAX_ITERATIONS {
        if z.norm() > 2.0 {
            return i;
        }
        z = z.powu(2) + c;
    }
    MAX_ITERATIONS
}

fn get_set(
    canvas_width: u32,
    canvas_height: u32,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    c_opt: Option<Complex<f64>>,
) -> Vec<u8> {
    let mut iterations: Vec<u32> = Vec::new();
    let mut histogram: BTreeMap<u32, u32> = BTreeMap::new();
    for y in 0..canvas_height {
        for x in 0..canvas_width {
            let z = Complex {
                re: x_min + (x_max - x_min) * (x as f64 / canvas_width as f64),
                im: y_min + (y_max - y_min) * (y as f64 / canvas_height as f64),
            };
            let iter_count = get_iter_count(z, c_opt);
            iterations.push(iter_count);
            if iter_count < MAX_ITERATIONS {
                histogram
                    .entry(iter_count)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }
    let mut colors: HashMap<u32, (u8, u8, u8)> = HashMap::new();
    let mut iter_sum = 0;
    let histogram_sum = histogram.values().sum::<u32>() as f64;
    for k in histogram.keys() {
        iter_sum += histogram[k];
        let hue = 270.0 - (iter_sum as f64 / histogram_sum) * 80.0;
        let color = hsv_to_rgb(hue, 1.0, 1.0);
        colors.insert(*k, color);
    }

    iterations
        .iter()
        .flat_map(|i| {
            if *i == MAX_ITERATIONS {
                [0, 0, 0, 255]
            } else {
                let color = colors[i];
                [color.0, color.1, color.2, 255]
            }
        })
        .collect()
}
