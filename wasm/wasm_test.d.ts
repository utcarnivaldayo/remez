/* tslint:disable */
/* eslint-disable */
/**
* @param {Float64Array} a
* @param {Float64Array} b
* @returns {number}
*/
export function norm(a: Float64Array, b: Float64Array): number;
/**
* @param {Float64Array} a
* @param {Float64Array} b
*/
export function substruct(a: Float64Array, b: Float64Array): void;
/**
* @param {number} order
* @param {number} f_pass
* @param {number} f_stop
* @param {Float64Array} coefficients
* @param {Float64Array} axis_freq
* @param {Float64Array} magnitude_response
* @returns {number}
*/
export function wrap_remez(order: number, f_pass: number, f_stop: number, coefficients: Float64Array, axis_freq: Float64Array, magnitude_response: Float64Array): number;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly add: (a: number, b: number) => number;
  readonly norm: (a: number, b: number, c: number, d: number) => number;
  readonly substruct: (a: number, b: number, c: number, d: number) => void;
  readonly wrap_remez: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        