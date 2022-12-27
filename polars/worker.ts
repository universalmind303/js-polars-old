import * as pli from "../pkg/js_polars.js";
let initialized = false;
export async function start(mem: WebAssembly.Memory) {
  await pli.default(undefined, mem);
  await pli.initThreadPool(navigator.hardwareConcurrency);
  await pli.init_hooks();
  initialized = true;
  console.log("initialized");
  self.postMessage({ type: "ready" });
}

self.addEventListener("message", async (event) => {
  switch (event.data.type) {
    case "start": {
      return await start(event.data.payload);
    }
    case "read_csv": {
      const { options } = event.data;
      const df = pli.read_csv(
        event.data.buf,
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
      );
      return postMessage({
        type: "read_csv",
        ptr: (df as any).ptr,
      });
    }
    default: {
      console.log("unknown method", event.data.method);
    }
  }
});

// onmessage = async function(e) {
//   console.log('msg from main thread')
//   console.log(e)
//   postMessage('msg from worker')
// }
// onerror = function(e) {
//   console.log('error from worker')
//   console.log(e)
// }
