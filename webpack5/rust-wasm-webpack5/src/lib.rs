use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: f64) -> Calculator {
        Calculator { value: initial }
    }

    pub fn add(&mut self, num: f64) {
        self.value += num;
    }

    pub fn multiply(&mut self, num: f64) {
        self.value *= num;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[wasm_bindgen]
pub fn format_greeting(name: &str) -> String {
    format!("Hello, {}! 👋", name)
}

#[wasm_bindgen]
pub fn log_message(msg: &str) {
    web_sys::console::log_1(&msg.into());
}