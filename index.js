import * as pl from "./dist/index.js"

const filepath = "http://localhost:8000/example.csv"

const csv_buf = await fetch(filepath).then((res) => res.arrayBuffer());
let arr = new Uint8Array(csv_buf)

const df = await pl.readCsv(arr)
console.table(df.toRecords())