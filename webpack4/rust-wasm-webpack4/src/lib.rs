use wasm_bindgen::prelude::*;

// 计算斐波那契数列
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 字符串处理函数
#[wasm_bindgen]
pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

// 在控制台打印消息
#[wasm_bindgen]
pub fn log_message(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

// 导出结构体
#[wasm_bindgen]
pub struct Point {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    // 计算两点间距离
    pub fn distance_to(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // 获取坐标字符串
    pub fn to_string(&self) -> String {
        format!("({:.1}, {:.1})", self.x, self.y)
    }
}

#[wasm_bindgen(start)]
pub fn main_js() {}