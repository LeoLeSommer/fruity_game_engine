import { getBundle } from "fruity_game_engine";

export function CircleCollider(...args) {
  return getBundle().CircleCollider(...args)
}
export function RectCollider(...args) {
  return getBundle().RectCollider(...args)
}
export function createFruityPhysic2DModule(...args) {
  return getBundle().createFruityPhysic2DModule(...args)
}
