import * as pli from "../pkg/js_polars.js";

export class DataFrame extends pli.DataFrame {
  constructor(ptr: number) {
    super();
    return (pli.DataFrame as any).__wrap(ptr);
  }
}
