import { Module, SignalProperty } from "fruity_game_engine";
import { EntityId } from "fruity_ecs";

export function createFruityHierarchyModule(): Module;
export class Parent {
  constructor();
  parentId: SignalProperty<EntityId | null>;
  nestedLevel: number;
}
