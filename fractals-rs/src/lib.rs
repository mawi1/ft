use std::collections::{BTreeMap, HashMap};
use std::f64;

use hsv::hsv_to_rgb;
use num_complex::Complex;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

mod utils;

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

#[derive(Clone, Copy)]
struct ComplexPlaneArea {
    pub x: f64,
    pub y: f64,
    pub length: f64,
}

#[wasm_bindgen]
pub struct FractalPlotter {
    canvas_size: u32,
    context: CanvasRenderingContext2d,
    area_history: Vec<ComplexPlaneArea>,
}

#[wasm_bindgen]
impl FractalPlotter {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        set_panic_hook();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self {
            canvas_size: canvas.width(),
            context,
            area_history: vec![ComplexPlaneArea {
                x: -1.5,
                y: -1.5,
                length: 3.0,
            }],
        }
    }

    fn do_plot(&mut self, c_opt: Option<C>) {
        let data = get_set(
            self.canvas_size,
            self.area_history.last().unwrap(),
            c_opt.map(|c| Complex {
                re: c.real,
                im: c.imaginary,
            }),
        );
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&data),
            self.canvas_size,
            self.canvas_size,
        )
        .unwrap();
        self.context.put_image_data(&data, 0.0, 0.0).unwrap();
    }

    #[wasm_bindgen]
    pub fn plot(&mut self, c_opt: Option<C>, selection_opt: Option<Selection>) {
        if let Some(selection) = selection_opt {
            let x_p = selection.x as f64 / self.canvas_size as f64;
            let y_p = selection.y as f64 / self.canvas_size as f64;
            let l_p = selection.length / self.canvas_size as f64;

            let last_area = self.area_history.last().unwrap();
            self.area_history.push(ComplexPlaneArea {
                x: last_area.x + x_p * last_area.length,
                y: last_area.y + y_p * last_area.length,
                length: last_area.length * l_p,
            });
        }
        self.do_plot(c_opt);
    }

    #[wasm_bindgen]
    pub fn go_back(&mut self, c_opt: Option<C>) {
        if self.area_history.len() >= 2 {
            self.area_history.pop();
            self.do_plot(c_opt);
        }
    }
}

fn get_set(canvas_size: u32, area: &ComplexPlaneArea, c_opt: Option<Complex<f64>>) -> Vec<u8> {
    let mut iterations: Vec<u32> = Vec::new();
    let mut histogram: BTreeMap<u32, u32> = BTreeMap::new();
    for y in 0..canvas_size {
        for x in 0..canvas_size {
            let z = Complex {
                re: area.x + area.length * (x as f64 / canvas_size as f64),
                im: area.y + area.length * (y as f64 / canvas_size as f64),
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
