import {
  Signal,
  ObserverHandler,
  Module,
  ScriptValue,
} from "fruity_game_engine"

export type AnyComponentReference<T = unknown> = T

export type AnyComponent = { [key: string]: any }

export type EntityServiceSnapshot = SerializedEntity[]

export type EntityId = number

export interface DeserializeService {

}

export interface EntityProperties {
  entityId: EntityId
  name: string
  enabled: boolean
}

export interface EntityReference {

  getAllComponents(): AnyComponentReference[]
  getComponentsByTypeIdentifier(componentIdentifier: string): AnyComponentReference[]
  getEntityId(): EntityId
  getName(): string
  isEnabled(): boolean
}

export interface EntityService {
  onCreated: Signal<EntityReference>
  onDeleted: Signal<EntityId>
  getEntityReference(entityId: EntityId): EntityReference | null
  query(): ScriptQueryBuilder
  create(name: string, enabled: boolean, components: AnyComponent[]): EntityId
  remove(entityId: EntityId): void
  addComponents(entityId: EntityId, components: AnyComponent[]): void
  removeComponent(entityId: EntityId, componentIndex: number): void
  clear(): void
  snapshot(): EntityServiceSnapshot
  restore(clearBefore: boolean, snapshot: EntityServiceSnapshot): void
}

export interface ExtensionComponentService {

}

export interface ScriptQuery<Args extends any[] = []> {
  onEntityCreated: Signal<EntityReference>;
  onEntityDeleted: Signal<EntityId>;
  forEach(callback: (args: Args) => void);
  onCreated(callback: (args: Args) => undefined | (() => void)): ObserverHandler;
}
export interface ScriptQueryBuilder<Args extends any[] = []> {
  withEntity(): ScriptQueryBuilder<[...Args, EntityReference]>;
  withId(): ScriptQueryBuilder<[...Args, EntityId]>;
  withName(): ScriptQueryBuilder<[...Args, string]>;
  withEnabled(): ScriptQueryBuilder<[...Args, boolean]>;
  with<T>(componentIdentifier: string): ScriptQueryBuilder<[...Args, T]>;
  withOptional<T>(
    componentIdentifier: string
  ): ScriptQueryBuilder<[...Args, T | null]>;
  without(
    componentIdentifier: string
  ): ScriptQueryBuilder<[...Args, null]>;
  build(): ScriptQuery<[...Args]>
}
export interface SerializedAnyComponent {
  className: string
  fields: {[key: string]: ScriptValue}
}

export interface SerializedEntity {
  localId: number
  name: string
  enabled: boolean
  components: ScriptValue[]
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
