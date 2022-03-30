console.log('index.js')
const w = new Worker('./worker.js', {type: 'module'})
w.postMessage(1)