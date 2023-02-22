import { getBundle } from "fruity_game_engine";

export function createFruityEcsModule(...args) {
  return getBundle().createFruityEcsModule(...args)
}
