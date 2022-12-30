use std::thread;

use polars::prelude::*;
use wasm_bindgen::prelude::*;

use super::expr::{ExprArray, JsExpr};
use crate::{console_log, dataframe::JsDataFrame, error::JsPolarsErr, JsResult};
#[wasm_bindgen(js_name = LazyFrame)]
#[repr(transparent)]
#[derive(Clone)]
pub struct JsLazyFrame {
    ldf: LazyFrame,
}

impl From<LazyFrame> for JsLazyFrame {
    fn from(ldf: LazyFrame) -> Self {
        JsLazyFrame { ldf }
    }
}

#[wasm_bindgen(js_class=LazyFrame)]
impl JsLazyFrame {
    pub fn describe_plan(&self) -> String {
        self.ldf.describe_plan()
    }
    pub fn describe_optimized_plan(&self) -> JsResult<String> {
        let result = self
            .ldf
            .describe_optimized_plan()
            .map_err(JsPolarsErr::from)?;
        Ok(result)
    }

    #[wasm_bindgen(js_name = "__collect_from_worker")]
    pub fn collect_from_worker(&self) -> JsResult<JsDataFrame> {
        self.ldf
            .clone()
            .collect()
            .map_err(|e| JsPolarsErr::from(e).into())
            .map(|df| df.into())
    }

    pub fn select(&mut self, exprs: &js_sys::Array) -> JsResult<JsLazyFrame> {
        let exprs = to_expr_array(exprs)?;
        let ldf = self.ldf.clone();
        Ok(ldf.select(&exprs).into())
    }
}

pub(crate) fn to_expr_array(iter: &js_sys::Array) -> JsResult<Box<[Expr]>> {
    use wasm_bindgen::convert::FromWasmAbi;
    use wasm_bindgen::JsCast;
    let iterator = js_sys::try_iter(iter)?.ok_or_else(|| "need to pass iterable JS values!")?;

    iterator
        .into_iter()
        .map(|jsv| {
            let jsv = jsv?;
            let key = JsValue::from_str("ptr");
            let ptr = unsafe { js_sys::Reflect::get(&jsv, &key)? };
            let n: f64 = js_sys::Number::unchecked_from_js(ptr).into();
            let expr: JsExpr = unsafe { JsExpr::from_abi(n as u32) };
            Ok(expr.inner)
        })
        .collect()
}
