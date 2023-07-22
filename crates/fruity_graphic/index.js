import { getBundle } from "fruity_game_engine";

export function createFruityGraphicModule(...args) {
  return getBundle().createFruityGraphicModule(...args)
}

export function Color(...args) {
  return getBundle().Color(...args)
}

Color.fruityGetType = function() {
  return getBundle().Color_getType()
}

export function Vector2D(...args) {
  return getBundle().Vector2D(...args)
}

Vector2D.fruityGetType = function() {
  return getBundle().Vector2D_getType()
}

export function Vector3D(...args) {
  return getBundle().Vector3D(...args)
}

Vector3D.fruityGetType = function() {
  return getBundle().Vector3D_getType()
}
