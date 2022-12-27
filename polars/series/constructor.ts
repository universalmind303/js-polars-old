import {Series} from ".";

export interface SeriesConstructor {
  // (name: string, values: any[], dtype?: DataType, strict?: boolean): Series
  new(name: string, values: any[]): Series
  // isSeries(arg: any): arg is Series;
  // readonly prototype: any[];
}

export interface SeriesConstructor {
  /**
   * Creates a series from an iterable object.
   * @param iterable An iterable object to convert to an array.
   */
  from<T>(iterable: Iterable<T> | ArrayLike<T>): Series;
  /**
   * Returns a new series from a set of elements.
   * @param items A set of elements to include in the new array object.
   */
  of<T>(...items: T[]): Series;
}