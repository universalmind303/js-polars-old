
import * as Comlink from 'comlink';
import * as pl from './pkg/polars.js'

self.onmessage = async event => {
  await pl.default()
  await pl.init_hooks()
  await pl.initThreadPool(2);
  console.log(pl)
  self.console.log("from worker")
  const randomArr = () => Array.from({length: 1_000_000}, () => Math.round((Math.random() * 100)));
  const columns = Array.from({length: 100}, (i) => {
    return pl.Series.new_f64(`col_${i}`, randomArr())

  })
  const s1 = pl.Series.new_f64("a", randomArr())
  const s2 = pl.Series.new_f64("b", randomArr())
  let df = pl.DataFrame.read_columns(columns[Symbol.iterator]())
  const s3 = pl.Series.new_f64("c", randomArr())

  df = df.add(s3)
  console.log(df.unique(false, ['a'], "first").as_str())
};


