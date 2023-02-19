import {
  Signal,
  ScriptCallback,
  ObserverHandler,
  Module,
} from "fruity_game_engine"

export type AnyComponent = { [key: string]: any }

export type EntityId = number

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
  onCreated(callback: ScriptCallback): ObserverHandler;
}
export interface StartupSystemParams {
  ignorePause: boolean
}

export interface SystemParams {
  poolIndex: number
  ignorePause: boolean
}

export interface SystemService {
  addSystem(identifier: string, callback: ScriptCallback, params: SystemParams | null | undefined)
  addStartupSystem(identifier: string, callback: ScriptCallback, params: StartupSystemParams | null | undefined)
  isPaused(): boolean
  setPaused(paused: boolean): void
}

export function createFruityEcsModule(): Module
