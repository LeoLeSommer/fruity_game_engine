import { getBundle } from "fruity_game_engine";

export function Parent(...args) {
  return getBundle().Parent(...args)
}
export function createFruityHierarchyModule(...args) {
  return getBundle().createFruityHierarchyModule(...args)
}
