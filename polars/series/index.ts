import { DataType } from "../../pkg/js_polars.js";
import * as pli from "../../pkg/js_polars.js";
import { SeriesConstructor } from "./constructor.js";
import { wrap } from "../utils.js";

export interface Series {
  /**
   * __Append a Series to this one.__
   * ___
   * @param {Series} other - Series to append.
   * @example
   * > const s = pl.Series("a", [1, 2, 3])
   * > const s2 = pl.Series("b", [4, 5, 6])
   * > s.append(s2)
   * shape: (6,)
   * Series: 'a' [i64]
   * [
   *         1
   *         2
   *         3
   *         4
   *         5
   *         6
   * ]
   */
  append(other: Series): void;
}

class _Series implements Series {
  constructor(arg0: any, arg1?: any, dtype?: any, strict?: any) {
    throw new Error("Method not implemented.");
  }
  static from(iterable) {
    return new Series("", iterable);
  }
  static of(...items) {
    return new Series("", items);
  }

  @wrap("as_str")
  public toString(): any {}

  get [Symbol.toStringTag]() {
    return "Series";
  }
  @wrap()
  abs() {}

  @wrap()
  append(other: Series) {}
  @wrap()
  argMax() {}
  @wrap()
  argMin() {}
  @wrap()
  argSort(reverse = false) {}
  @wrap()
  argTrue() {}
  @wrap()
  argUnique() {}
  @wrap("alias")
  as(name) {}
  @wrap()
  bitand(other) {}
  @wrap()
  bitor(other) {}
  @wrap()
  bitxor(other) {}
  @wrap()
  cast(dtype, strict = false) {}
  @wrap()
  chunkLengths() {}
  @wrap()
  clone() {}

  concat(other) {
    const s = this.clone();
    this.append(other);
    return s;
  }
  @wrap()
  cumMax(reverse: any = false) {}
  @wrap()
  cumMin(reverse: any = false) {}
  @wrap()
  cumProd(reverse: any = false) {}
  @wrap()
  cumSum(reverse: any = false) {}
}

export const Series: SeriesConstructor = _Series as any;
