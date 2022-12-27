import { DataFrame } from "./dataframe.js";
import * as pli from "../pkg/js_polars.js";
import { waitForMsgType } from "./utils.js";
const wasm = await pli.default();

const worker = new Worker(new URL("./worker.js", import.meta.url), {
  type: "module",
});

// worker.onmessage = async (event) => {

//   if (event.data.message === "repost") {
//     worker.postMessage(event.data.data);
//   }
// };
worker.postMessage({ type: "start", payload: wasm.memory });
await waitForMsgType(worker, "ready");

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
): Promise<DataFrame> {
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
  return new DataFrame(ptr, worker);
}
