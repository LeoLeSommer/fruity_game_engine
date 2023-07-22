import { getBundle } from "fruity_game_engine";

export function createFruityGraphic2DModule(...args) {
  return getBundle().createFruityGraphic2DModule(...args)
}

export function Camera(...args) {
  return getBundle().Camera(...args)
}

Camera.fruityGetType = function() {
  return getBundle().Camera_getType()
}

export function Rotate2D(...args) {
  return getBundle().Rotate2D(...args)
}

Rotate2D.fruityGetType = function() {
  return getBundle().Rotate2D_getType()
}

export function Scale2D(...args) {
  return getBundle().Scale2D(...args)
}

Scale2D.fruityGetType = function() {
  return getBundle().Scale2D_getType()
}

export function Sprite(...args) {
  return getBundle().Sprite(...args)
}

Sprite.fruityGetType = function() {
  return getBundle().Sprite_getType()
}

export function Transform2D(...args) {
  return getBundle().Transform2D(...args)
}

Transform2D.fruityGetType = function() {
  return getBundle().Transform2D_getType()
}

export function Translate2D(...args) {
  return getBundle().Translate2D(...args)
}

Translate2D.fruityGetType = function() {
  return getBundle().Translate2D_getType()
}
