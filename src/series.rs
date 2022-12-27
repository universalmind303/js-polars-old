// use crate::conversion::str_to_null_behavior;
// use crate::conversion::str_to_rankmethod;
// use crate::conversion::Wrap;
// use crate::utils::str_to_polarstype;
// use crate::{console_log, log};
use polars::prelude::*;
use wasm_bindgen::JsCast;

// use super::{error::JsPolarsErr, JsResult};
// use crate::conversion::FromJsValue;
use crate::{
    conversion::Wrap, dataframe::JsDataFrame, error::JsPolarsErr, extern_iterator, extern_struct,
    JsResult,
};

use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=Series)]
#[repr(transparent)]
pub struct JsSeries {
    pub(crate) series: Series,
}

impl JsSeries {
    pub(crate) fn new(series: Series) -> Self {
        JsSeries { series }
    }
}

impl From<Series> for JsSeries {
    fn from(series: Series) -> Self {
        Self { series }
    }
}

// impl wasm_bindgen::convert::FromWasmAbi for JsSeries {

// }
impl Deref for JsSeries {
    type Target = Series;

    fn deref(&self) -> &Self::Target {
        &self.series
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Series")]
    pub type ExternSeries;

    #[wasm_bindgen(typescript_type = "any")]
    pub type ExternAnyValue;

    #[wasm_bindgen(method, getter = ptr)]
    fn ptr(this: &ExternSeries) -> f64;

    #[wasm_bindgen(static_method_of = ExternSeries)]
    fn wrap(ptr: u32) -> ExternSeries;

    #[wasm_bindgen(typescript_type = "Series[]")]
    pub type SeriesArray;

}

extern_struct!(ExternSeries, JsSeries);
extern_iterator!(SeriesArray, ExternSeries, JsSeries);

#[wasm_bindgen(js_class=Series)]
impl JsSeries {
    #[wasm_bindgen]
    pub fn new_int_8_array(name: String, arr: &mut [i8]) -> JsSeries {
        Series::new(&name, arr).into()
    }
    #[wasm_bindgen]
    pub fn new_uint_8_array(name: String, arr: &mut [u8]) -> JsSeries {
        Series::new(&name, arr).into()
    }
    #[wasm_bindgen]
    pub fn new_int_16_array(name: String, arr: &mut [i16]) -> JsSeries {
        Series::new(&name, arr).into()
    }
    #[wasm_bindgen]
    pub fn new_uint_16_array(name: String, arr: &mut [u16]) -> JsSeries {
        Series::new(&name, arr).into()
    }
    #[wasm_bindgen]
    pub fn new_int_32_array(name: String, arr: &mut [i32]) -> JsSeries {
        Series::new(&name, arr).into()
    }
    #[wasm_bindgen]
    pub fn new_uint_32_array(name: String, arr: &mut [u32]) -> JsSeries {
        Series::new(&name, arr).into()
    }

    #[wasm_bindgen]
    pub fn new_int_64_array(name: String, arr: &mut [i64]) -> JsSeries {
        Series::new(&name, arr).into()
    }
    #[wasm_bindgen]
    pub fn new_uint_64_array(name: String, arr: &mut [u64]) -> JsSeries {
        Series::new(&name, arr).into()
    }
    #[wasm_bindgen]
    pub fn new_float_32_array(name: String, arr: &mut [f32]) -> JsSeries {
        Series::new(&name, arr).into()
    }
    #[wasm_bindgen]
    pub fn new_float_64_array(name: String, arr: &mut [f64]) -> JsSeries {
        Series::new(&name, arr).into()
    }
    #[wasm_bindgen]
    pub fn new_opt_str_array(name: String, arr: js_sys::Array) -> JsSeries {
        let len = arr.length() as usize;
        let mut builder = Utf8ChunkedBuilder::new(&name, len, len * 25);
        for value in arr.iter() {
            if value.is_null() {
                builder.append_null();
            } else {
                builder.append_value(value.as_string().unwrap().as_str());
            }
        }
        builder.finish().into_series().into()
    }
    #[wasm_bindgen]
    pub fn new_opt_bool_array(name: String, arr: js_sys::Array) -> JsSeries {
        let len = arr.length() as usize;
        let mut builder = BooleanChunkedBuilder::new(&name, len);
        for value in arr.iter() {
            if value.is_null() {
                builder.append_null();
            } else {
                builder.append_option(value.as_bool());
            }
        }
        builder.finish().into_series().into()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.series.name().to_owned()
    }
    pub fn to_string(&self) -> String {
        format!("{}", self.series)
    }
    pub fn estimated_size(&self) -> i64 {
        self.series.estimated_size() as i64
    }

    pub fn rechunk(&mut self, in_place: bool) -> Option<JsSeries> {
        let series = self.series.rechunk();
        if in_place {
            self.series = series;
            None
        } else {
            Some(series.into())
        }
    }

    pub fn bitand(&self, other: &JsSeries) -> JsResult<JsSeries> {
        let out = self
            .series
            .bitand(&other.series)
            .map_err(JsPolarsErr::from)?;
        Ok(out.into())
    }
    pub fn bitor(&self, other: &JsSeries) -> JsResult<JsSeries> {
        let out = self
            .series
            .bitor(&other.series)
            .map_err(JsPolarsErr::from)?;
        Ok(out.into())
    }
    pub fn bitxor(&self, other: &JsSeries) -> JsResult<JsSeries> {
        let out = self
            .series
            .bitxor(&other.series)
            .map_err(JsPolarsErr::from)?;
        Ok(out.into())
    }
    pub fn cumsum(&self, reverse: Option<bool>) -> JsSeries {
        let reverse = reverse.unwrap_or(false);
        self.series.cumsum(reverse).into()
    }
    pub fn cummax(&self, reverse: Option<bool>) -> JsSeries {
        let reverse = reverse.unwrap_or(false);
        self.series.cummax(reverse).into()
    }
    pub fn cummin(&self, reverse: Option<bool>) -> JsSeries {
        let reverse = reverse.unwrap_or(false);
        self.series.cummin(reverse).into()
    }
    pub fn cumprod(&self, reverse: Option<bool>) -> JsSeries {
        let reverse = reverse.unwrap_or(false);
        self.series.cumprod(reverse).into()
    }
    pub fn chunk_lengths(&self) -> Vec<u32> {
        self.series.chunk_lengths().map(|i| i as u32).collect()
    }
    pub fn rename(&mut self, name: String) {
        self.series.rename(&name);
    }
    pub fn mean(&self) -> Option<f64> {
        match self.series.dtype() {
            DataType::Boolean => {
                let s = self.series.cast(&DataType::UInt8).unwrap();
                s.mean()
            }
            _ => self.series.mean(),
        }
    }

    pub fn n_chunks(&self) -> u32 {
        self.series.n_chunks() as u32
    }

    pub fn limit(&self, num_elements: f64) -> JsSeries {
        self.series.limit(num_elements as usize).into()
    }

    pub fn slice(&self, offset: i64, length: f64) -> JsSeries {
        self.series.slice(offset, length as usize).into()
    }

    pub fn append(&mut self, other: &JsSeries) -> JsResult<()> {
        self.series
            .append(&other.series)
            .map_err(JsPolarsErr::from)?;
        Ok(())
    }

    pub fn extend(&mut self, other: &JsSeries) -> JsResult<()> {
        self.series
            .extend(&other.series)
            .map_err(JsPolarsErr::from)?;
        Ok(())
    }
    pub fn filter(&self, filter: &JsSeries) -> JsResult<JsSeries> {
        let filter_series = &filter.series;
        if let Ok(ca) = filter_series.bool() {
            let series = self.series.filter(ca).map_err(JsPolarsErr::from)?;
            Ok(JsSeries { series })
        } else {
            let err = "Expected a boolean mask".to_string();
            Err(err.into())
        }
    }

    pub fn add(&self, other: &JsSeries) -> JsSeries {
        (&self.series + &other.series).into()
    }

    pub fn sub(&self, other: &JsSeries) -> JsSeries {
        (&self.series - &other.series).into()
    }

    pub fn mul(&self, other: &JsSeries) -> JsSeries {
        (&self.series * &other.series).into()
    }

    pub fn div(&self, other: &JsSeries) -> JsSeries {
        (&self.series / &other.series).into()
    }

    pub fn rem(&self, other: &JsSeries) -> JsSeries {
        (&self.series % &other.series).into()
    }

    pub fn head(&self, length: Option<i64>) -> JsSeries {
        (self.series.head(length.map(|l| l as usize))).into()
    }

    pub fn tail(&self, length: Option<i64>) -> JsSeries {
        (self.series.tail(length.map(|l| l as usize))).into()
    }

    pub fn sort(&self, reverse: Option<bool>) -> JsSeries {
        let reverse = reverse.unwrap_or(false);
        self.series.sort(reverse).into()
    }

    pub fn argsort(&self, reverse: bool, nulls_last: bool) -> JsSeries {
        self.series
            .argsort(SortOptions {
                descending: reverse,
                nulls_last,
            })
            .into_series()
            .into()
    }

    pub fn unique(&self) -> JsResult<JsSeries> {
        let unique = self.series.unique().map_err(JsPolarsErr::from)?;
        Ok(unique.into())
    }

    pub fn unique_stable(&self) -> JsResult<JsSeries> {
        let unique = self.series.unique_stable().map_err(JsPolarsErr::from)?;
        Ok(unique.into())
    }

    pub fn value_counts(&self, sorted: bool) -> JsResult<JsDataFrame> {
        let df = self
            .series
            .value_counts(true, sorted)
            .map_err(JsPolarsErr::from)?;
        Ok(df.into())
    }

    pub fn arg_unique(&self) -> JsResult<JsSeries> {
        let arg_unique = self.series.arg_unique().map_err(JsPolarsErr::from)?;
        Ok(arg_unique.into_series().into())
    }

    pub fn arg_min(&self) -> Option<i64> {
        self.series.arg_min().map(|v| v as i64)
    }

    pub fn arg_max(&self) -> Option<i64> {
        self.series.arg_max().map(|v| v as i64)
    }

    pub fn take(&self, indices: Vec<u32>) -> JsResult<JsSeries> {
        let indices = UInt32Chunked::from_vec("", indices);

        let take = self.series.take(&indices).map_err(JsPolarsErr::from)?;
        Ok(JsSeries::new(take))
    }

    pub fn take_with_series(&self, indices: &JsSeries) -> JsResult<JsSeries> {
        let idx = indices.series.u32().map_err(JsPolarsErr::from)?;
        let take = self.series.take(idx).map_err(JsPolarsErr::from)?;
        Ok(JsSeries::new(take))
    }

    pub fn null_count(&self) -> JsResult<i64> {
        Ok(self.series.null_count() as i64)
    }

    pub fn has_validity(&self) -> bool {
        self.series.has_validity()
    }

    pub fn is_null(&self) -> JsSeries {
        Self::new(self.series.is_null().into_series())
    }

    pub fn is_not_null(&self) -> JsSeries {
        Self::new(self.series.is_not_null().into_series())
    }

    pub fn is_not_nan(&self) -> JsResult<JsSeries> {
        let ca = self.series.is_not_nan().map_err(JsPolarsErr::from)?;
        Ok(ca.into_series().into())
    }

    pub fn is_nan(&self) -> JsResult<JsSeries> {
        let ca = self.series.is_nan().map_err(JsPolarsErr::from)?;
        Ok(ca.into_series().into())
    }

    pub fn is_finite(&self) -> JsResult<JsSeries> {
        let ca = self.series.is_finite().map_err(JsPolarsErr::from)?;
        Ok(ca.into_series().into())
    }

    pub fn is_infinite(&self) -> JsResult<JsSeries> {
        let ca = self.series.is_infinite().map_err(JsPolarsErr::from)?;
        Ok(ca.into_series().into())
    }

    pub fn is_unique(&self) -> JsResult<JsSeries> {
        let ca = self.series.is_unique().map_err(JsPolarsErr::from)?;
        Ok(ca.into_series().into())
    }

    pub fn sample_frac(
        &self,
        frac: f64,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> JsResult<JsSeries> {
        // Safety:
        // Wrap is transparent.
        let s = self
            .series
            .sample_frac(frac, with_replacement, shuffle, seed)
            .map_err(JsPolarsErr::from)?;
        Ok(s.into())
    }

    pub fn is_duplicated(&self) -> JsResult<JsSeries> {
        let ca = self.series.is_duplicated().map_err(JsPolarsErr::from)?;
        Ok(ca.into_series().into())
    }

    pub fn explode(&self) -> JsResult<JsSeries> {
        let s = self.series.explode().map_err(JsPolarsErr::from)?;
        Ok(s.into())
    }

    pub fn take_every(&self, n: i64) -> JsSeries {
        let s = self.series.take_every(n as usize);
        s.into()
    }

    pub fn series_equal(&self, other: &JsSeries, null_equal: bool, strict: bool) -> bool {
        if strict {
            self.series.eq(&other.series)
        } else if null_equal {
            self.series.series_equal_missing(&other.series)
        } else {
            self.series.series_equal(&other.series)
        }
    }

    pub fn eq(&self, rhs: &JsSeries) -> JsResult<JsSeries> {
        Ok(Self::new(
            self.series
                .equal(&rhs.series)
                .map_err(JsPolarsErr::from)?
                .into_series(),
        ))
    }

    pub fn neq(&self, rhs: &JsSeries) -> JsResult<JsSeries> {
        Ok(Self::new(
            self.series
                .not_equal(&rhs.series)
                .map_err(JsPolarsErr::from)?
                .into_series(),
        ))
    }

    pub fn gt(&self, rhs: &JsSeries) -> JsResult<JsSeries> {
        Ok(Self::new(
            self.series
                .gt(&rhs.series)
                .map_err(JsPolarsErr::from)?
                .into_series(),
        ))
    }

    pub fn gt_eq(&self, rhs: &JsSeries) -> JsResult<JsSeries> {
        Ok(Self::new(
            self.series
                .gt_eq(&rhs.series)
                .map_err(JsPolarsErr::from)?
                .into_series(),
        ))
    }

    pub fn lt(&self, rhs: &JsSeries) -> JsResult<JsSeries> {
        Ok(Self::new(
            self.series
                .lt(&rhs.series)
                .map_err(JsPolarsErr::from)?
                .into_series(),
        ))
    }

    pub fn lt_eq(&self, rhs: &JsSeries) -> JsResult<JsSeries> {
        Ok(Self::new(
            self.series
                .lt_eq(&rhs.series)
                .map_err(JsPolarsErr::from)?
                .into_series(),
        ))
    }

    pub fn _not(&self) -> JsResult<JsSeries> {
        let bool = self.series.bool().map_err(JsPolarsErr::from)?;
        Ok((!bool).into_series().into())
    }

    pub fn as_str(&self) -> JsResult<String> {
        Ok(format!("{:?}", self.series))
    }

    pub fn len(&self) -> i64 {
        self.series.len() as i64
    }

    pub fn to_physical(&self) -> JsSeries {
        let s = self.series.to_physical_repr().into_owned();
        s.into()
    }
}

// pub fn reinterpret(s: &Series, signed: bool) -> polars::prelude::Result<Series> {
//     match (s.dtype(), signed) {
//         (DataType::UInt64, true) => {
//             let ca = s.u64().unwrap();
//             Ok(ca.reinterpret_signed().into_series())
//         }
//         (DataType::UInt64, false) => Ok(s.clone()),
//         (DataType::Int64, false) => {
//             let ca = s.i64().unwrap();
//             Ok(ca.reinterpret_unsigned().into_series())
//         }
//         (DataType::Int64, true) => Ok(s.clone()),
//         _ => Err(PolarsError::ComputeError(
//             "reinterpret is only allowed for 64bit integers dtype, use cast otherwise".into(),
//         )),
//     }
// }
pub(crate) fn to_series_collection(iter: js_sys::Iterator) -> Vec<Series> {
    let cols: Vec<Series> = iter
        .into_iter()
        .map(|jsv| {
            let jsv = jsv.unwrap();
            let key = JsValue::from_str("ptr");
            let ptr = unsafe { js_sys::Reflect::get(&jsv, &key).unwrap() };
            let n: f64 = js_sys::Number::unchecked_from_js(ptr).into();
            let ser: JsSeries = unsafe { JsSeries::from_abi(n as u32) };
            ser.series
        })
        .collect();
    cols
}

// pub(crate) fn to_jsseries_collection(s: Vec<Series>) -> Vec<u32> {
//     use wasm_bindgen::convert::IntoWasmAbi;
//     let s: Vec<u32> = s
//         .into_iter()
//         .map(move |series| {
//             let js_ser = JsSeries { series };

//             js_ser.into_abi()
//         })
//         .collect();

//     s
//     // todo!()
// }
