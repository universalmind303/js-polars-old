/* eslint-disable no-undef */
import pl from "./polars/index";
import pli from "./pkg";
const col1 = pl.Series("col_1", [1, 2, 3]).inner();
const col2 = pl.Series("col_2", [1, 2, 3]).inner();

// console.log({colPtr: col.ptr});

// console.log({rt});
const df = pli.DataFrame.read_columns([col1, col2][Symbol.iterator]());
df.columns = ["foo", "bar"];
const seriesArr: any = Array.from(df.get_columns()).map((n: any) => pli.Series.wrap(n).toString());

console.log(seriesArr);
// console.log({ptr});
// const s = pli.Series.__wrap(col.ptr);
// // console.log((pli.Series as any).__wrap(df.get_columns()[0]).toString());
