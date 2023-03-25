export type ScriptValue = any

export type ScriptObject = {[key: string]: ScriptValue}

export type JsIntrospectObject = { [key: string]: any }

export type ResourceReference<T> = T

export interface Signal<T> {
  notify(event: T);
  addObserver(callback: (value: T) => void);
}

export interface SignalProperty<T> {
  value: T;
  onUpdated: Signal<T>;
}

export interface ObserverHandler {
  dispose();
}

export type SettingsElem =
  | number
  | boolean
  | string
  | SettingsElem[]
  | Settings
  | null

export type Settings = { [key: string]: SettingsElem }

export type StartMiddleware = (world: World) => void

export type FrameMiddleware = (world: World) => void

export type EndMiddleware = (world: World) => void

export type SetupWorldMiddlewareNext = (world: World, settings: Settings) => void

export type RunWorldMiddlewareNext = (world: World, settings: Settings) => void

export type SetupWorldMiddleware = (world: World, settings: Settings, next: SetupWorldMiddlewareNext) => void

export type RunWorldMiddleware = (world: World, settings: Settings, next: RunWorldMiddlewareNext) => void

export interface FrameService {

  getDelta(): number
}

export interface Module {
  name: string
  dependencies: string[]
  setup?: ((arg0: World, arg1: Settings) => void) | null | undefined | void
  setupAsync?: ((arg0: World, arg1: Settings) => Promise<unknown>) | null | undefined | void
  loadResources?: ((arg0: World, arg1: Settings) => void) | null | undefined | void
  loadResourcesAsync?: ((arg0: World, arg1: Settings) => Promise<unknown>) | null | undefined | void
  setupWorldMiddleware?: SetupWorldMiddleware | null | undefined | void
  runWorldMiddleware?: RunWorldMiddleware | null | undefined | void
}

export interface ResourceContainer {

  require<T>(identifier: string): T
  get<T>(identifier: string): T | null
  contains(identifier: string): boolean
  add(identifier: string, resource: JsIntrospectObject)
  remove(identifier: string): void
  loadResourcesSettingsAsync(settings: Settings): Promise<unknown>
}

export class World {

  constructor(settings: Settings)
  registerModule(module: Module): void
  setupModulesAsync(): Promise<unknown>
  loadResourcesAsync(): Promise<unknown>
  setup(): void
  run(): void
  getResourceContainer(): ResourceContainer
}

