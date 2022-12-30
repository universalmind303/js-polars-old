pub mod conversion;
pub use conversion::*;
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

#[wasm_bindgen(js_class=Expr)]
impl JsExpr {
    /// Take absolute values
    pub fn abs(&self) -> JsExpr {
        self.clone().inner.abs().into()
    }

    /// Get the group indexes of the group by operation.
    /// Should be used in aggregation context only.
    /// @example
    /// ```js
    /// > df = pl.DataFrame({
    /// ...  "group": [
    /// ...      "one",
    /// ...      "one",
    /// ...      "one",
    /// ...      "two",
    /// ...      "two",
    /// ...      "two",
    /// ...  ],
    /// ...  "value": [94, 95, 96, 97, 97, 99],
    /// ... })
    /// >>> df.groupby("group", {maintain_order:True}).agg(pl.col("value").agg_groups())
    /// shape: (2, 2)
    /// ┌───────┬───────────┐
    /// │ group ┆ value     │
    /// │ ---   ┆ ---       │
    /// │ str   ┆ list[u32] │
    /// ╞═══════╪═══════════╡
    /// │ one   ┆ [0, 1, 2] │
    /// ├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
    /// │ two   ┆ [3, 4, 5] │
    /// └───────┴───────────┘
    /// ```
    pub fn agg_groups(&self) -> JsExpr {
        self.clone().inner.agg_groups().into()
    }

    /// Rename the output of an expression.
    /// @param name - new name
    /// @example
    /// ```js
    /// > df = pl.DataFrame({
    /// ...   "a": [1, 2, 3],
    /// ...   "b": ["a", "b", None],
    /// ... })
    /// > df
    /// shape: (3, 2)
    /// ╭─────┬──────╮
    /// │ a   ┆ b    │
    /// │ --- ┆ ---  │
    /// │ i64 ┆ str  │
    /// ╞═════╪══════╡
    /// │ 1   ┆ "a"  │
    /// ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    /// │ 2   ┆ "b"  │
    /// ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    /// │ 3   ┆ null │
    /// ╰─────┴──────╯
    /// > df.select([
    /// ...   pl.col("a").alias("bar"),
    /// ...   pl.col("b").alias("foo"),
    /// ... ])
    /// shape: (3, 2)
    /// ╭─────┬──────╮
    /// │ bar ┆ foo  │
    /// │ --- ┆ ---  │
    /// │ i64 ┆ str  │
    /// ╞═════╪══════╡
    /// │ 1   ┆ "a"  │
    /// ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    /// │ 2   ┆ "b"  │
    /// ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    /// │ 3   ┆ null │
    /// ╰─────┴──────╯
    /// ```
    pub fn alias(&self, alias: &str) -> JsExpr {
        self.clone().inner.alias(alias).into()
    }
    /// Get the index of the maximal value.
    pub fn arg_max(&self) -> JsExpr {
        self.clone().inner.arg_max().into()
    }
    /// Get the index of the minimal value.
    pub fn arg_min(&self) -> JsExpr {
        self.clone().inner.arg_min().into()
    }
    /// Get the index values that would sort this column.
    /// @param reverse
    /// - false -> order from small to large.
    /// - true -> order from large to small.
    /// @returns UInt32 Series
    pub fn arg_sort(&self, reverse: bool) -> JsExpr {
        self.clone()
            .inner
            .arg_sort(SortOptions {
                descending: reverse,
                nulls_last: true,
            })
            .into()
    }

    /// Get index of first unique value.
    pub fn arg_unique(&self) -> JsExpr {
        self.clone().inner.arg_unique().into()
    }

    /// Fill missing values with the next to be seen values
    pub fn backward_fill(&self) -> JsExpr {
        self.clone().inner.backward_fill(None).into()
    }

    /// Cast between data types.
    pub fn cast() -> JsExpr {
        todo!()
    }

    /// Count the number of values in this expression
    pub fn count(&self) -> JsExpr {
        self.clone().inner.count().into()
    }

    /// Calculate the n-th discrete difference.
    /// @param n - number of slots to shift
    /// @param nullBehavior -  'ignore' or 'drop'
    pub fn diff(&self) -> JsExpr {
        todo!()
    }

    /// Compute the dot/inner product between two Expressions
    /// @param other Expression to compute dot product with
    pub fn dot(&self, other: &JsExpr) -> JsExpr {
        todo!()
        // self.inner.clone().dot(other.inner.clone()).into()
    }

    ///  Exclude certain columns from a wildcard/regex selection.
    ///
    ///  You may also use regexes in the exclude list. They must start with `^` and end with `$`.
    ///
    ///  @param columns Column(s) to exclude from selection
    ///  @example
    ///  ```js
    ///   > df = pl.DataFrame({
    ///   ...   "a": [1, 2, 3],
    ///   ...   "b": ["a", "b", None],
    ///   ...   "c": [None, 2, 1],
    ///   ...})
    ///   > df
    ///   shape: (3, 3)
    ///   ╭─────┬──────┬──────╮
    ///   │ a   ┆ b    ┆ c    │
    ///   │ --- ┆ ---  ┆ ---  │
    ///   │ i64 ┆ str  ┆ i64  │
    ///   ╞═════╪══════╪══════╡
    ///   │ 1   ┆ "a"  ┆ null │
    ///   ├╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    ///   │ 2   ┆ "b"  ┆ 2    │
    ///   ├╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    ///   │ 3   ┆ null ┆ 1    │
    ///   ╰─────┴──────┴──────╯
    ///   > df.select(
    ///   ...   pl.col("*").exclude("b"),
    ///   ... )
    ///  shape: (3, 2)
    ///  ╭─────┬──────╮
    ///  │ a   ┆ c    │
    ///  │ --- ┆ ---  │
    ///  │ i64 ┆ i64  │
    ///  ╞═════╪══════╡
    ///  │ 1   ┆ null │
    ///  ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    ///  │ 2   ┆ 2    │
    ///  ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    ///  │ 3   ┆ 1    │
    ///  ╰─────┴──────╯
    ///  ```
    /// /
    pub fn exclude(&self) -> JsExpr {
        todo!()
    }

    /// Explode a list or utf8 Series.
    /// This means that every item is expanded to a new row.
    pub fn explode(&self) -> JsExpr {
        self.clone().inner.explode().into()
    }
    pub fn extend(&self) -> JsExpr {
        todo!()
    }
    pub fn extend_constant(&self) -> JsExpr {
        todo!()
    }

    /// Fill nan value with a fill value
    pub fn fill_nan(&self, expr: &JsExpr) -> JsExpr {
        self.inner.clone().fill_nan(expr.inner.clone()).into()
    }
    /// Fill null value with a fill value or strategy
    pub fn fill_null(&self, expr: &JsExpr) -> JsExpr {
        self.clone().inner.fill_null(expr.inner.clone()).into()
    }

    /// Filter a single column.
    /// Mostly useful in in aggregation context.
    /// If you want to filter on a DataFrame level, use `LazyFrame.filter`.
    /// @param predicate -  Boolean expression.
    pub fn filter(&self, predicate: &JsExpr) -> JsExpr {
        self.clone().inner.filter(predicate.inner.clone()).into()
    }

    /// Get the first value.
    pub fn first(&self) -> JsExpr {
        self.clone().inner.first().into()
    }

    /// Fill missing values with the latest seen values
    pub fn forward_fill(&self) -> JsExpr {
        self.clone().inner.forward_fill(None).into()
    }
}
