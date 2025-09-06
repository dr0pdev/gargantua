/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const main: () => void;
export const init_simulation: (a: number, b: number) => void;
export const update_simulation: () => void;
export const get_ray_positions: () => [number, number];
export const get_black_hole_position: () => [number, number];
export const update_black_hole_mass: (a: number) => void;
export const reset_simulation: () => void;
export const get_ray_count: () => number;
export const update_ray_count: (a: number) => void;
export const get_simulation_info: () => [number, number];
export const get_initial_ray_positions: () => [number, number];
export const __wbindgen_export_0: WebAssembly.Table;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
