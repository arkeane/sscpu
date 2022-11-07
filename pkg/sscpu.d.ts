/* tslint:disable */
/* eslint-disable */
/**
*/
export function sanity_check(): void;
/**
* @param {Uint16Array} program
* @param {Uint16Array} data
* @returns {any}
*/
export function cpu_run(program: Uint16Array, data: Uint16Array): any;
/**
* @param {Uint16Array} program
* @param {Uint16Array} data
* @returns {any}
*/
export function cpu_init_for_step(program: Uint16Array, data: Uint16Array): any;
/**
* @param {any} jscpu
* @returns {any}
*/
export function cpu_run_step(jscpu: any): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly sanity_check: () => void;
  readonly cpu_run: (a: number, b: number, c: number, d: number) => number;
  readonly cpu_init_for_step: (a: number, b: number, c: number, d: number) => number;
  readonly cpu_run_step: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
