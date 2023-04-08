import {
  Signal,
  ObserverHandler,
  Module,
  ScriptValue,
} from "fruity_game_engine"

export type EntityServiceSnapshot = SerializedEntity[]

export class Enabled {
  0: boolean
  constructor(enabled: boolean)
}

export interface EntityReference {

  getEntityId(): EntityId
  getName(): string
  setName(name: string): void
  isEnabled(): boolean
  setEnabled(enabled: boolean): void
  getAllComponents(): AnyComponentReference[]
  getComponentsByTypeIdentifier(componentIdentifier: string): AnyComponentReference[]
}

export interface EntityService {
  onCreated: Signal<EntityReference>
  onDeleted: Signal<EntityId>
  getEntityReference(entityId: EntityId): EntityReference | null
  createEntity(name: string, enabled: boolean, components: Component[]): EntityId
  removeEntity(entityId: EntityId): Component[]
  addComponents(entityId: EntityId, newComponents: Component[]): void
  removeComponent(entityId: EntityId, componentIndex: number): void
  clear(): void
  snapshot(): EntityServiceSnapshot
  restore(clearBefore: boolean, snapshot: EntityServiceSnapshot): void
}

export interface ExtensionComponentService {

}

export class Name {
  0: string
  constructor(string: string)
}

export interface SerializationService {

}

export interface StartupSystemParams {
  ignorePause?: boolean | null | undefined | void
  executeInMainThread?: boolean | null | undefined | void
}

export interface SystemParams {
  poolIndex?: number | null | undefined | void
  ignorePause?: boolean | null | undefined | void
  executeInMainThread?: boolean | null | undefined | void
}

export interface SystemService {

  addSystem(identifier: string, callback: (() => void), params?: SystemParams | null | undefined | void)
  addStartupSystem(identifier: string, callback: (() => (() => void) | null | undefined | void), params?: StartupSystemParams | null | undefined | void)
  isPaused(): boolean
  setPaused(paused: boolean): void
}

export function createFruityEcsModule(): Module
