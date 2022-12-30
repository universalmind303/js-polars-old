import * as pli from "../pkg/js_polars.js";

import { ReadCsvOptions } from "./index.js";
let initialized = false;
export async function start(mem: WebAssembly.Memory) {
  await pli.default(undefined, mem);
  await pli.initThreadPool(navigator.hardwareConcurrency);
  await pli.init_hooks();
  initialized = true;
  self.postMessage({ type: "ready" });
}


function read_csv(buf: Uint8Array, options: ReadCsvOptions): void {
  const ptr = (
    pli.read_csv(
      buf,
      options.inferSchemaLength ?? 100,
      options.chunkSize ?? 10000,
      options.hasHeader ?? true,
      options.ignoreErrors ?? true,
      options.numRows,
      options.skipRows ?? 0,
      options.rechunk ?? false,
      options.encoding ?? "utf8",
      options.numThreads,
      options.lowMemory ?? false,
      options.parseDates ?? false,
      options.skipRowsAfterHeader ?? 0,
    ) as any
  ).ptr;
  return postMessage({
    type: "read_csv",
    ptr,
  });
}

self.addEventListener("message", async (event) => {
  switch (event.data.type) {
    case "start": {
      return await start(event.data.payload);
    }
    case "read_csv": {
      const { options } = event.data;
      return read_csv(event.data.buf, options);
    }
    case "LazyFrame::collect": {
      const lf = (pli.LazyFrame as any).__wrap(event.data.ptr);
      const df = await lf.__collect_from_worker();
      return postMessage({
        type: "LazyFrame::collect",
        ptr: df.ptr,
      });

    }
    default: {
      console.log("unknown method", event.data.method);
    }
  }
});
