/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function clix(cmdStr: string): Clix
export class ClixResult { }
export class Clix {
  constructor(cmdStr: string)
  expect(line: string): Clix
  run(): ClixResult
}
