
import * as pl from './pkg/polars.js'
self.onmessage = async event => {
  await pl.default()
  await pl.init_hooks()
  await pl.initThreadPool(navigator.hardwareConcurrency);
  const randomArr = () => Array.from({length: 10_000}, () => Math.round((Math.random() * 100 % 100)));
  const columns = Array.from({length: 10}, (_, i) => {
    return pl.Series.new_f64(`col_${i}`, randomArr())
  })
  const res = await fetch("https://raw.githubusercontent.com/universalmind303/js-polars/js-polars-try-again/examples/1k.json")
  const b = await res.arrayBuffer()
  let df = pl.DataFrame.read_json(new Int8Array(b))
  console.log(df.as_str())

};


