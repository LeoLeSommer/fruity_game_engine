import { getBundle } from "fruity_game_engine";

export function createFruityHierarchyModule(...args) {
  return getBundle().createFruityHierarchyModule(...args)
}

export function Parent(...args) {
  return getBundle().Parent(...args)
}

Parent.fruityGetType = function() {
  return getBundle().Parent_getType()
}
