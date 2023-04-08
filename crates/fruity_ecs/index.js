import { getBundle } from "fruity_game_engine";

export function Enabled(...args) {
  return getBundle().Enabled(...args)
}
export function Name(...args) {
  return getBundle().Name(...args)
}
export function createFruityEcsModule(...args) {
  return getBundle().createFruityEcsModule(...args)
}
