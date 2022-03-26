use polars::prelude::{DataFrame, Series};
use wasm_bindgen::JsCast;

use super::{
    error::JsPolarsErr,
    series::*,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JsDataFrame {
    df: DataFrame,
}

impl From<DataFrame> for JsDataFrame {
    fn from(df: DataFrame) -> Self {
        Self { df }
    }
}


#[wasm_bindgen]
impl JsDataFrame {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        DataFrame::new_no_checks(vec![]).into()
    }

    pub fn assign(&self, series: Series) -> Result<JsDataFrame, JsValue> {
        let mut df = self.df.clone();
        df.with_column(series.series).map_err(JsPolarsErr::from)?;
        Ok(df.into())
    }
}