
import * as Comlink from 'comlink';
import * as pl from './pkg/polars.js'
import fs from "fs"
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
  
  const buf = fs.readFileSync("./examples/100k.json")
  console.log(buf)
  // const s1 = pl.Series.new_f64("a", randomArr())
  // const s2 = pl.Series.new_f64("b", randomArr())
  let df = pl.DataFrame.read_json(columns[Symbol.iterator]())
  // const s3 = pl.Series.new_f64("c", randomArr())
  console.profile("unique")
  console.log(df.as_single_chunk_par().as_str())
  console.profileEnd("unique")

  console.profile("as_single_ptr")
  console.log(df.as_)
  console.profileEnd("as_single_ptr")
};


