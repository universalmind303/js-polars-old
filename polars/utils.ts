import * as pli from "../pkg/js_polars.js";
import {DataFrame} from "./dataframe.js";

export function waitForMsgType(target: any, type: any) {
  return new Promise(resolve => {
    target.addEventListener('message', function onMsg(event: any) {
      if (event.data == null || event.data.type !== type) return;
      target.removeEventListener('message', onMsg);
      resolve(event);
    });
  });
}

function wrap_async(methodName?: string): any {
  return function (target: any, propertyKey: string, descriptor: any) {
    methodName = methodName || propertyKey;
    function inner(this: any, ...args: any[]) {
      this.worker.postMessage({
        type: methodName,
        ptr: this.ptr,
      });
      return waitForMsgType(this.worker, methodName);
    }
    target[propertyKey] = inner;
    return inner;
  };
}


export function wrap(methodName?: string, options: { useWorker?: boolean } = {}): any {
  if (options.useWorker) {
    return wrap_async(methodName);
  }
  return function (target: any, propertyKey: string, descriptor: any) {
    let factory;
    
    switch(target.constructor.name) {
      case 'DataFrame': factory = pli.DataFrame; break;
      case '_Series': factory = pli.Series; break;
      default: factory = pli.DataFrame;
    };

    methodName = methodName || propertyKey;
    function inner(this: any, ...args: any[]) {
      const df: any = factory.__wrap(this.ptr);
      const out =  df[methodName!](...args);
      if(out instanceof pli.DataFrame || out instanceof pli.Series) {
        this.ptr = (out as any).ptr;
      } else {
        return out;
      }
    }
    target[propertyKey] = inner;
    return inner as any;
    // descriptor.enumerable = value;
  };
}

export function unwrap(methodName?: string): any {
  return function (target: any, propertyKey: string, descriptor: any) {
    methodName = methodName || propertyKey;
    function inner(this: any, ...args: any[]) {
      const df: any = (pli.DataFrame as any).__wrap(this.ptr);
      return df[methodName!](...args);
    }
    target[propertyKey] = inner;
    return inner as any;
    // descriptor.enumerable = value;
  };
}