use wasm_bindgen::prelude::*;

// 斐波那契数列计算
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 计算器结构体
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

    pub fn subtract(&mut self, num: f64) {
        self.value -= num;
    }

    pub fn multiply(&mut self, num: f64) {
        self.value *= num;
    }

    pub fn divide(&mut self, num: f64) {
        self.value /= num;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

// 字符串处理函数
#[wasm_bindgen]
pub fn format_greeting(name: &str) -> String {
    format!("Hello, {}! 👋", name)
}

// 日志函数
#[wasm_bindgen]
pub fn log_message(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

// 数学计算函数
#[wasm_bindgen]
pub fn calculate_pi(iterations: u32) -> f64 {
    let mut pi = 0.0;
    for k in 0..iterations {
        let term = if k % 2 == 0 {
            1.0 / (2 * k + 1) as f64
        } else {
            -1.0 / (2 * k + 1) as f64
        };
        pi += term;
    }
    4.0 * pi
}