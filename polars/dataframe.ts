import * as pli from "../pkg/js_polars.js";
import { waitForMsgType, wrap } from "./utils.js";



export interface DataFrame {
  toString(): string;
}

export class DataFrame {
  constructor(private ptr: number, private worker: Worker) {}

  @wrap('to_string')
  public toString(): any {}
  @wrap()
  public rechunk(): any {}
  @wrap('n_chunks')
  public nChunks(): any {}
  @wrap()
  public shape(): any {}
  @wrap()
  public height(): any {}
  @wrap()
  public width(): any {}
  @wrap()
  public hstack(columns: pli.Series[]): any {}
  @wrap()
  extend(other: DataFrame): any {}
  @wrap()
  drop(name: string): any {}
  @wrap()
  toRecords(): any {}
}
