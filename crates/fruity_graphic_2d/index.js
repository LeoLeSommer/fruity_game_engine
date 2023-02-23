import { getBundle } from "fruity_game_engine";

export function Camera(...args) {
  return getBundle().Camera(...args)
}
export function Rotate2D(...args) {
  return getBundle().Rotate2D(...args)
}
export function Scale2D(...args) {
  return getBundle().Scale2D(...args)
}
export function Sprite(...args) {
  return getBundle().Sprite(...args)
}
export function Transform2D(...args) {
  return getBundle().Transform2D(...args)
}
export function Translate2D(...args) {
  return getBundle().Translate2D(...args)
}
export function createFruityGraphic2DModule(...args) {
  return getBundle().createFruityGraphic2DModule(...args)
}
