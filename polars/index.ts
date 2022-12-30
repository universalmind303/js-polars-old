import { DataFrame } from "./dataframe.js";
import { Series } from "./series/index.js";
import * as pli from "../pkg/js_polars.js";
export * from "./lazy/index.js";

import { waitForMsgType } from "./utils.js";
const wasm = await pli.default();
export { DataFrame, Series };
const worker = new Worker(new URL("./worker.js", import.meta.url), {
  type: "module",
});

(window as any).__polars_worker__ = worker;

worker.postMessage({ type: "start", payload: wasm.memory });
await waitForMsgType(worker, "ready");
const s = pli.Series.new_opt_bool_array("foo", [
  true,
  false,
  true,
  false,
  true,
  false,
]);
console.log(s);
const readCsvDefaultOptions = {
  inferSchemaLength: 100,
  hasHeader: true,
  ignoreErrors: true,
  chunkSize: 10000,
  skipRows: 0,
  sep: ",",
  rechunk: false,
  encoding: "utf8",
  lowMemory: false,
  parseDates: false,
  skipRowsAfterHeader: 0,
};

export interface ReadCsvOptions {
  inferSchemaLength?: number;
  hasHeader?: boolean;
  ignoreErrors?: boolean;
  chunkSize?: number;
  skipRows?: number;
  sep?: string;
  rechunk?: boolean;
  encoding?: string;
  lowMemory?: boolean;
  parseDates?: boolean;
  skipRowsAfterHeader?: number;
  numRows?: number;
  numThreads?: number;
}

export async function readCsv(
  buf: Uint8Array,
  options: ReadCsvOptions = readCsvDefaultOptions,
): Promise<pli.DataFrame> {
  worker.postMessage(
    {
      type: "read_csv",
      options,
      buf,
    },
    [buf.buffer],
  );
  const event: any = await waitForMsgType(worker, "read_csv");
  const ptr = event.data.ptr;
  return (pli.DataFrame as any).__wrap(ptr);
}

(pli.LazyFrame.prototype as any).collect = async function () {
  const ptr = this.ptr;
  worker.postMessage({
    type: "LazyFrame::collect",
    ptr,
  });
  const event: any = await waitForMsgType(worker, "LazyFrame::collect");
  const df_ptr = event.data.ptr;
  return (pli.DataFrame as any).__wrap(df_ptr);
}