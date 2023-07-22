import { getBundle } from "fruity_game_engine";

export function createFruityEcsModule(...args) {
  return getBundle().createFruityEcsModule(...args)
}

export function Enabled(...args) {
  return getBundle().Enabled(...args)
}

Enabled.fruityGetType = function() {
  return getBundle().Enabled_getType()
}

export function Name(...args) {
  return getBundle().Name(...args)
}

Name.fruityGetType = function() {
  return getBundle().Name_getType()
}
