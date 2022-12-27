use crate::conversion::*;
use polars::lazy::dsl;
use polars::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=Expr)]
#[repr(transparent)]
#[derive(Clone)]
pub struct JsExpr {
    pub(crate) inner: Expr,
}

pub(crate) trait ToExprs {
    fn to_exprs(self) -> Vec<Expr>;
}
impl JsExpr {
    pub(crate) fn new(inner: Expr) -> JsExpr {
        JsExpr { inner }
    }
}
impl From<Expr> for JsExpr {
    fn from(s: Expr) -> JsExpr {
        JsExpr::new(s)
    }
}
impl ToExprs for Vec<JsExpr> {
    fn to_exprs(self) -> Vec<Expr> {
        // Safety
        // repr is transparent
        // and has only got one inner field`
        unsafe { std::mem::transmute(self) }
    }
}

impl ToExprs for Vec<&JsExpr> {
    fn to_exprs(self) -> Vec<Expr> {
        self.into_iter()
            .map(|e| e.inner.clone())
            .collect::<Vec<Expr>>()
    }
}
#[wasm_bindgen(js_class=Expr)]
impl JsExpr {
    pub fn add(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Plus, rhs.inner.clone()).into()
    }

    pub fn sub(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Minus, rhs.inner.clone()).into()
    }

    pub fn mul(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Multiply, rhs.inner.clone()).into()
    }

    pub fn truediv(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::TrueDivide, rhs.inner.clone()).into()
    }

    pub fn modulus(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Modulus, rhs.inner.clone()).into()
    }

    pub fn floordiv(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Divide, rhs.inner.clone()).into()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.inner)
    }

    pub fn eq(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.eq(other.inner.clone()).into()
    }

    pub fn neq(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.neq(other.inner.clone()).into()
    }

    pub fn gt(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.gt(other.inner.clone()).into()
    }

    pub fn gt_eq(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.gt_eq(other.inner.clone()).into()
    }

    pub fn lt_eq(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.lt_eq(other.inner.clone()).into()
    }

    pub fn lt(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.lt(other.inner.clone()).into()
    }

    pub fn alias(&self, name: String) -> JsExpr {
        self.clone().inner.alias(&name).into()
    }

    pub fn is_not(&self) -> JsExpr {
        self.clone().inner.not().into()
    }

    pub fn is_null(&self) -> JsExpr {
        self.clone().inner.is_null().into()
    }

    pub fn is_not_null(&self) -> JsExpr {
        self.clone().inner.is_not_null().into()
    }

    pub fn is_infinite(&self) -> JsExpr {
        self.clone().inner.is_infinite().into()
    }

    pub fn is_finite(&self) -> JsExpr {
        self.clone().inner.is_finite().into()
    }

    pub fn is_nan(&self) -> JsExpr {
        self.clone().inner.is_nan().into()
    }

    pub fn is_not_nan(&self) -> JsExpr {
        self.clone().inner.is_not_nan().into()
    }

    pub fn min(&self) -> JsExpr {
        self.clone().inner.min().into()
    }

    pub fn max(&self) -> JsExpr {
        self.clone().inner.max().into()
    }

    pub fn mean(&self) -> JsExpr {
        self.clone().inner.mean().into()
    }

    pub fn median(&self) -> JsExpr {
        self.clone().inner.median().into()
    }

    pub fn sum(&self) -> JsExpr {
        self.clone().inner.sum().into()
    }

    pub fn n_unique(&self) -> JsExpr {
        self.clone().inner.n_unique().into()
    }

    pub fn arg_unique(&self) -> JsExpr {
        self.clone().inner.arg_unique().into()
    }

    pub fn unique(&self) -> JsExpr {
        self.clone().inner.unique().into()
    }

    pub fn unique_stable(&self) -> JsExpr {
        self.clone().inner.unique_stable().into()
    }

    pub fn first(&self) -> JsExpr {
        self.clone().inner.first().into()
    }

    pub fn last(&self) -> JsExpr {
        self.clone().inner.last().into()
    }

    pub fn list(&self) -> JsExpr {
        self.clone().inner.list().into()
    }

    pub fn quantile(&self, quantile: &JsExpr) -> JsExpr {
        todo!()
    }

    pub fn agg_groups(&self) -> JsExpr {
        self.clone().inner.agg_groups().into()
    }

    pub fn count(&self) -> JsExpr {
        self.clone().inner.count().into()
    }

    pub fn value_counts(&self, multithreaded: bool, sorted: bool) -> JsExpr {
        self.inner
            .clone()
            .value_counts(multithreaded, sorted)
            .into()
    }

    pub fn unique_counts(&self) -> JsExpr {
        self.inner.clone().unique_counts().into()
    }

    pub fn cast() -> JsExpr {
        todo!()
    }
    pub fn sort_with(&self, descending: bool, nulls_last: bool) -> JsExpr {
        self.clone()
            .inner
            .sort_with(SortOptions {
                descending,
                nulls_last,
            })
            .into()
    }
    pub fn arg_sort(&self, reverse: bool) -> JsExpr {
        self.clone()
            .inner
            .arg_sort(SortOptions {
                descending: reverse,
                nulls_last: true,
            })
            .into()
    }
    pub fn arg_max(&self) -> JsExpr {
        self.clone().inner.arg_max().into()
    }
    pub fn arg_min(&self) -> JsExpr {
        self.clone().inner.arg_min().into()
    }
    pub fn take(&self, idx: &JsExpr) -> JsExpr {
        self.clone().inner.take(idx.inner.clone()).into()
    }
    pub fn sort_by() -> JsExpr {
        todo!()
    }
    pub fn backward_fill(&self) -> JsExpr {
        self.clone().inner.backward_fill(None).into()
    }

    pub fn forward_fill(&self) -> JsExpr {
        self.clone().inner.forward_fill(None).into()
    }

    pub fn shift(&self, periods: i64) -> JsExpr {
        self.clone().inner.shift(periods).into()
    }

    pub fn shift_and_fill(&self, periods: i64, fill_value: &JsExpr) -> JsExpr {
        self.clone()
            .inner
            .shift_and_fill(periods, fill_value.inner.clone())
            .into()
    }

    pub fn fill_null(&self, expr: &JsExpr) -> JsExpr {
        self.clone().inner.fill_null(expr.inner.clone()).into()
    }
    pub fn fill_null_with_strategy(&self) -> JsExpr {
        todo!()
        // self.inner
        //     .clone()
        //     .apply(move |s| s.fill_null(strategy.0), GetOutput::same_type())
        //     .with_fmt("fill_null")
        //     .into()
    }
    pub fn fill_nan(&self, expr: &JsExpr) -> JsExpr {
        self.inner.clone().fill_nan(expr.inner.clone()).into()
    }

    pub fn drop_nulls(&self) -> JsExpr {
        self.inner.clone().drop_nulls().into()
    }

    pub fn drop_nans(&self) -> JsExpr {
        self.inner.clone().drop_nans().into()
    }

    pub fn filter(&self, predicate: &JsExpr) -> JsExpr {
        self.clone().inner.filter(predicate.inner.clone()).into()
    }

    pub fn reverse(&self) -> JsExpr {
        self.clone().inner.reverse().into()
    }

    pub fn std(&self, ddof: Option<u8>) -> JsExpr {
        let ddof = ddof.unwrap_or(1);
        self.clone().inner.std(ddof).into()
    }

    pub fn var(&self, ddof: Option<u8>) -> JsExpr {
        let ddof = ddof.unwrap_or(1);
        self.clone().inner.var(ddof).into()
    }
    pub fn is_unique(&self) -> JsExpr {
        self.clone().inner.is_unique().into()
    }

    pub fn is_first(&self) -> JsExpr {
        self.clone().inner.is_first().into()
    }

    pub fn explode(&self) -> JsExpr {
        self.clone().inner.explode().into()
    }
    pub fn take_every(&self, n: i64) -> JsExpr {
        self.clone()
            .inner
            .map(
                move |s: Series| Ok(s.take_every(n as usize)),
                GetOutput::same_type(),
            )
            .with_fmt("take_every")
            .into()
    }
    pub fn tail(&self, n: Option<i64>) -> JsExpr {
        let n = n.map(|v| v as usize);
        self.clone().inner.tail(n).into()
    }

    pub fn head(&self, n: Option<i64>) -> JsExpr {
        let n = n.map(|v| v as usize);
        self.clone().inner.head(n).into()
    }
    pub fn slice(&self, offset: &JsExpr, length: &JsExpr) -> JsExpr {
        self.inner
            .clone()
            .slice(offset.inner.clone(), length.inner.clone())
            .into()
    }
    pub fn round(&self, decimals: u32) -> JsExpr {
        self.clone().inner.round(decimals).into()
    }

    pub fn floor(&self) -> JsExpr {
        self.clone().inner.floor().into()
    }

    pub fn ceil(&self) -> JsExpr {
        self.clone().inner.ceil().into()
    }
    pub fn clip(&self) -> JsExpr {
        todo!()
    }
    pub fn abs(&self) -> JsExpr {
        self.clone().inner.abs().into()
    }
    pub fn is_duplicated(&self) -> JsExpr {
        self.clone().inner.is_duplicated().into()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct When {
    predicate: Expr,
}
#[wasm_bindgen]
#[derive(Clone)]
pub struct WhenThen {
    predicate: Expr,
    then: Expr,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WhenThenThen {
    inner: dsl::WhenThenThen,
}

#[wasm_bindgen]
impl When {
    pub fn then(&self, expr: &JsExpr) -> WhenThen {
        WhenThen {
            predicate: self.predicate.clone(),
            then: expr.inner.clone(),
        }
    }
}
#[wasm_bindgen]
impl WhenThen {
    pub fn when(&self, predicate: &JsExpr) -> WhenThenThen {
        let e = dsl::when(self.predicate.clone())
            .then(self.then.clone())
            .when(predicate.inner.clone());
        WhenThenThen { inner: e }
    }

    pub fn otherwise(&self, expr: &JsExpr) -> JsExpr {
        dsl::ternary_expr(
            self.predicate.clone(),
            self.then.clone(),
            expr.inner.clone(),
        )
        .into()
    }
}

#[wasm_bindgen]
impl WhenThenThen {
    pub fn when(&self, predicate: &JsExpr) -> WhenThenThen {
        Self {
            inner: self.inner.clone().when(predicate.inner.clone()),
        }
    }

    pub fn then(&self, expr: &JsExpr) -> WhenThenThen {
        Self {
            inner: self.inner.clone().then(expr.inner.clone()),
        }
    }
    pub fn otherwise(&self, expr: &JsExpr) -> JsExpr {
        self.inner.clone().otherwise(expr.inner.clone()).into()
    }
}

#[wasm_bindgen]
pub fn when(predicate: &JsExpr) -> When {
    When {
        predicate: predicate.inner.clone(),
    }
}


#[wasm_bindgen]
pub fn col(name: String) -> JsExpr {
    dsl::col(&name).into()
}

#[wasm_bindgen]
pub fn count() -> JsExpr {
    dsl::count().into()
}

#[wasm_bindgen]
pub fn first() -> JsExpr {
    dsl::first().into()
}

#[wasm_bindgen]
pub fn last() -> JsExpr {
    dsl::last().into()
}

#[wasm_bindgen]
pub fn cols(names: JsValue) -> JsExpr {
    let names: Vec<String> = serde_wasm_bindgen::from_value(names).unwrap();
    dsl::cols(names).into()
}