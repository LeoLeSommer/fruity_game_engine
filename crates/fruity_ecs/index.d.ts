import {
  Signal,
  ObserverHandler,
  Module,
  ScriptValue,
} from "fruity_game_engine"

export type AnyComponent = { [key: string]: any }

export type EntityId = number

export type EntityServiceSnapshot = SerializedEntity[]

export interface EntityProperties {
  entityId: EntityId
  name: string
  enabled: boolean
}

export interface EntityReference {

  getEntityId(): EntityId
  getName(): string
  isEnabled(): boolean
}

export interface EntityService {
  onCreated: Signal<EntityReference>
  onDeleted: Signal<EntityId>
  getEntity(entityId: EntityId): EntityReference | null
  query(): ScriptQuery
  create(name: string, enabled: boolean, components: AnyComponent[]): EntityId
  createWithId(entityId: EntityId, name: string, enabled: boolean, components: AnyComponent[]): EntityId
  remove(entityId: EntityId): void
  addComponent(entityId: EntityId, components: AnyComponent[]): void
  removeComponent(entityId: EntityId, componentIndex: number): void
  clear(): void
  snapshot(): EntityServiceSnapshot
  restore(snapshot: EntityServiceSnapshot): void
}

export interface ExtensionComponentService {

}

export interface ScriptQuery<Args extends any[] = []> {
  withEntity(): ScriptQuery<[...Args, EntityReference]>;
  withId(): ScriptQuery<[...Args, EntityId]>;
  withName(): ScriptQuery<[...Args, string]>;
  withEnabled(): ScriptQuery<[...Args, boolean]>;
  with<T>(componentIdentifier: string): ScriptQuery<[...Args, T]>;
  withOptional<T>(
    componentIdentifier: string
  ): ScriptQuery<[...Args, T | null]>;
  forEach(callback: (args: Args) => void);
  onCreated(callback: (args: Args) => undefined | (() => void)): ObserverHandler;
}
export interface SerializedAnyComponent {
  className: string
  fields: {[key: string]: ScriptValue}
}

export interface SerializedEntity {
  entityId: EntityId
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

  addSystem(identifier: string, callback: ((arg0: ResourceContainer) => void), params?: SystemParams | null | undefined | void)
  addStartupSystem(identifier: string, callback: ((arg0: ResourceContainer) => (() => void) | null | undefined | void), params?: StartupSystemParams | null | undefined | void)
  isPaused(): boolean
  setPaused(paused: boolean): void
}

export function createFruityEcsModule(): Module
