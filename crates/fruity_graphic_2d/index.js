import { getBundle } from "fruity_game_engine";

export function Camera(...args) {
  return getBundle().Camera(...args)
}
export function Rotate2d(...args) {
  return getBundle().Rotate2d(...args)
}
export function Scale2d(...args) {
  return getBundle().Scale2d(...args)
}
export function Sprite(...args) {
  return getBundle().Sprite(...args)
}
export function Transform2d(...args) {
  return getBundle().Transform2d(...args)
}
export function Translate2d(...args) {
  return getBundle().Translate2d(...args)
}
export function createFruityGraphic2DModule(...args) {
  return getBundle().createFruityGraphic2DModule(...args)
}
