import { getBundle } from "fruity_game_engine";

export function createFruityPhysic2DModule(...args) {
  return getBundle().createFruityPhysic2DModule(...args)
}

export function CircleCollider(...args) {
  return getBundle().CircleCollider(...args)
}

CircleCollider.fruityGetType = function() {
  return getBundle().CircleCollider_getType()
}

export function RectCollider(...args) {
  return getBundle().RectCollider(...args)
}

RectCollider.fruityGetType = function() {
  return getBundle().RectCollider_getType()
}
