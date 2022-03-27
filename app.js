const pl = require("./pkg")
const assert = require("assert")

let s1 = pl.Series.new_i8("a", [1, 2, 3])
let s2 = pl.Series.new_i8("a", [1, 2, 3])
console.log(s1.get_idx(0))
let s3 = s1.bitand(s2)
console.log(s3)
// assert(s.mean() === 2)
// console.log(s.mean())
// console.log(s.toString())
// console.log(s.toJSON())
// s.log()

// let df = new pl.DataFrame();
// df.assign(s);

// console.log(df)