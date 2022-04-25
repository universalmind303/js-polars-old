
building: `./build.sh`

you will also need to go into `pkg/snippets/wasm-*/workerHelpers.js` and replace 
`const pkg = await import('../../..');`
with 
`const pkg = await import('../../../polars.js');`

starting web server: `python server.py`

then open browser to localhost:8000/index.html
open up the console & you will see the logs from `worker.js` 

