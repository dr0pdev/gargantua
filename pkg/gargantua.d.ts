/* tslint:disable */
/* eslint-disable */
export function main(): void;
export function init_simulation(width: number, height: number): void;
export function update_simulation(): void;
export function get_ray_positions(): Float64Array;
export function get_black_hole_position(): Float64Array;
export function update_black_hole_mass(mass: number): void;
export function reset_simulation(): void;
export function get_ray_count(): number;
export function update_ray_count(new_count: number): void;
export function get_simulation_info(): string;
export function get_initial_ray_positions(): Float64Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: () => void;
  readonly init_simulation: (a: number, b: number) => void;
  readonly update_simulation: () => void;
  readonly get_ray_positions: () => [number, number];
  readonly get_black_hole_position: () => [number, number];
  readonly update_black_hole_mass: (a: number) => void;
  readonly reset_simulation: () => void;
  readonly get_ray_count: () => number;
  readonly update_ray_count: (a: number) => void;
  readonly get_simulation_info: () => [number, number];
  readonly get_initial_ray_positions: () => [number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
