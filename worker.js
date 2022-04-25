
import * as Comlink from 'comlink';
import * as pl from './pkg/polars.js'
self.onmessage = async event => {
  await pl.default()
  await pl.init_hooks()
  await pl.initThreadPool(navigator.hardwareConcurrency);
  console.log(pl)
  self.console.log("from worker")
  const randomArr = () => Array.from({length: 10_000}, () => Math.round((Math.random() * 100 % 100)));
  const columns = Array.from({length: 10}, (_, i) => {
    return pl.Series.new_f64(`col_${i}`, randomArr())
  })
  const buf = (await fetch("https://github.com/universalmind303/js-polars/blob/js-polars-try-again/examples/100k.json")).arrayBuffer()

  console.profile("read_json")
  let df = pl.DataFrame.read_json(buf)
  console.log(df.as_str())
  console.profileEnd("read_json")

};


