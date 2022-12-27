import * as pl from "./dist/index.js"

console.log(pl);

const filepath = "http://localhost:8000/example.csv"

const csv_buf = await fetch(filepath).then((res) => res.arrayBuffer());
let arr = new Uint8Array(csv_buf)

const df = await pl.readCsv(arr)
const s = df.height()
console.log(df.drop("foo"))
console.table(df.toRecords())
// const s = 

// const s = await df.toString()
// console.log(s)