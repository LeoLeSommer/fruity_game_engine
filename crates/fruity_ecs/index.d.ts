import {
  Module,
  ScriptValue,
  Signal,
  ObserverHandler,
} from "fruity_game_engine";

export type EntityId = number;
export type AnyComponent = { [key: string]: any };

export function createFruityEcsModule(): Module;

export interface ScriptCallback {
  call(args: ScriptValue[]): ScriptValue;
}

export interface ExtensionComponentService {}

export interface SystemParams {
  poolIndex: number;
  ignorePause: boolean;
}

export interface StartupSystemParams {
  ignorePause: boolean;
}

export interface SystemService {
  addSystem(
    identifier: string,
    callback: ScriptCallback,
    params?: SystemParams | null | undefined
  );
  addStartupSystem(
    identifier: string,
    callback: ScriptCallback,
    params?: StartupSystemParams | null | undefined
  );
  isPaused(): boolean;
  setPaused(value: boolean);
}

export interface EntityReference {
  getEntityId(): EntityId;
  getName(): string;
  isEnabled(): boolean;
}

export interface EntityService {
  onCreated: Signal<EntityReference>;
  onDeleted: Signal<EntityId>;
  getEntity(entityId: EntityId): EntityReference | null;
  query(): ScriptQuery<[]>;
  create(name: string, enabled: boolean, components: AnyComponent[]): EntityId;
  createWithId(
    entityId: EntityId,
    name: string,
    enabled: boolean,
    components: AnyComponent[]
  ): EntityId;
  remove(entityId: EntityId);
  addComponent(entityId: EntityId, components: AnyComponent[]);
  removeComponent(entityId: EntityId, componentIndex: number);
  clear();
}

export interface ScriptQuery<Args extends any[]> {
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
