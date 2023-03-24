import {
  SignalProperty,
  Module,
} from "fruity_game_engine"

import {
  EntityLocation,
} from "fruity_ecs"

export class Parent {
  parent: SignalProperty<EntityReference | null | undefined | void>
  nestedLevel: number
  constructor()
}

export function createFruityHierarchyModule(): Module
