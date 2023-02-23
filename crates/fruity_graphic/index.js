import { getBundle } from "fruity_game_engine";

export function Color(...args) {
  return getBundle().Color(...args)
}
export function Vector2D(...args) {
  return getBundle().Vector2D(...args)
}
export function Vector3D(...args) {
  return getBundle().Vector3D(...args)
}
export function createFruityGraphicModule(...args) {
  return getBundle().createFruityGraphicModule(...args)
}
