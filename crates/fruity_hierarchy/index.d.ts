import {
  SignalProperty,
  Module,
} from "fruity_game_engine"

import {
  EntityId,
} from "fruity_ecs"

export class Parent {
  parentId: SignalProperty<EntityId | null | undefined>
  nestedLevel: number
  constructor()
}

export function createFruityHierarchyModule(): Module
