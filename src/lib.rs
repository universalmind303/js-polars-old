mod conversion;
mod dataframe;
mod datatypes;
mod error;
mod series;
mod utils;
mod pool;

use wasm_bindgen::prelude::*;
pub use pool::init_thread_pool;


pub type JsResult<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}



#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}
