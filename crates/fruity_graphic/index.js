import { getBundle } from "fruity_game_engine";

export function Color(...args) {
  return getBundle().Color(...args)
}
export function Vector2d(...args) {
  return getBundle().Vector2d(...args)
}
export function Vector3d(...args) {
  return getBundle().Vector3d(...args)
}
export function createFruityGraphicModule(...args) {
  return getBundle().createFruityGraphicModule(...args)
}
