(()=>{"use strict";var e,r,t={517:(e,r,t)=>{function o(e,r){return new Promise((t=>{e.addEventListener("message",(function o({data:n}){null!=n&&n.type===r&&(e.removeEventListener("message",o),t(n))}))}))}let n;async function a(e,r,a){const s={type:"wasm_bindgen_worker_init",module:e,memory:r,receiver:a.receiver()};n=await Promise.all(Array.from({length:a.numThreads()},(async()=>{const e=new Worker(new URL(t.p+t.u(517),t.b),{type:void 0});return e.postMessage(s),await o(e,"wasm_bindgen_worker_ready"),e}))),a.build()}t.d(r,{Q:()=>a}),o(self,"wasm_bindgen_worker_init").then((async e=>{const r=await t.e(290).then(t.bind(t,290));await r.default(e.module,e.memory),postMessage({type:"wasm_bindgen_worker_ready"}),r.wbg_rayon_start_worker(e.receiver)}))},442:(e,r,t)=>{var o=t(290);self.onmessage=async e=>{await o.default(),await o.init_hooks(),console.log(o),self.console.log("from worker");const r=()=>Array.from({length:1e5},(()=>Math.round(100*Math.random()%20))),t=o.Series.new_f64("a",r()),n=o.Series.new_f64("b",r());let a=o.DataFrame.read_columns([t,n][Symbol.iterator]());const s=o.Series.new_f64("c",r());a=a.add(s),console.log(a.as_str())}}},o={};function n(e){var r=o[e];if(void 0!==r)return r.exports;var a=o[e]={exports:{}};return t[e](a,a.exports,n),a.exports}n.m=t,n.x=()=>{var e=n.O(void 0,[290],(()=>n(442)));return n.O(e)},e=[],n.O=(r,t,o,a)=>{if(!t){var s=1/0;for(u=0;u<e.length;u++){for(var[t,o,a]=e[u],i=!0,l=0;l<t.length;l++)(!1&a||s>=a)&&Object.keys(n.O).every((e=>n.O[e](t[l])))?t.splice(l--,1):(i=!1,a<s&&(s=a));if(i){e.splice(u--,1);var c=o();void 0!==c&&(r=c)}}return r}a=a||0;for(var u=e.length;u>0&&e[u-1][2]>a;u--)e[u]=e[u-1];e[u]=[t,o,a]},n.d=(e,r)=>{for(var t in r)n.o(r,t)&&!n.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:r[t]})},n.f={},n.e=e=>Promise.all(Object.keys(n.f).reduce(((r,t)=>(n.f[t](e,r),r)),[])),n.k=e=>e+".polars.css",n.u=e=>e+".polars.js",n.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),n.o=(e,r)=>Object.prototype.hasOwnProperty.call(e,r),n.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},n.p="/dist/",(()=>{n.b=self.location+"";var e={318:1,517:1};n.f.i=(r,t)=>{e[r]||importScripts(n.p+n.u(r))};var r=self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[],t=r.push.bind(r);r.push=r=>{var[o,a,s]=r;for(var i in a)n.o(a,i)&&(n.m[i]=a[i]);for(s&&s(n);o.length;)e[o.pop()]=1;t(r)}})(),r=n.x,n.x=()=>n.e(290).then(r),n.x()})();
//# sourceMappingURL=318.polars.js.map