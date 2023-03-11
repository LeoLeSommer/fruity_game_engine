export type ScriptValue = any

export type JsIntrospectObject = { [key: string]: any }

export type ResourceReference<T> = T

export type ScriptOrNativeResource = any

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

export type RunMiddleware = (world: World, settings: Settings) => void

export interface FrameService {

  getDelta(): number
}

export interface Module {
  name: string
  dependencies: string[]
  setup?: ((arg0: World, arg1: Settings) => void) | null | undefined
  setupAsync?: ((arg0: World, arg1: Settings) => Promise<unknown>) | null | undefined
  loadResources?: ((arg0: World, arg1: Settings) => void) | null | undefined
  loadResourcesAsync?: ((arg0: World, arg1: Settings) => Promise<unknown>) | null | undefined
  runMiddleware?: RunMiddleware | null | undefined
}

export interface ObjectFactoryService {

  instantiate(objectType: string, fields: {[key: string]: ScriptValue}): ScriptValue | null
}

export interface ScriptResourceContainer {

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
  run(): void
  getResourceContainer(): ScriptResourceContainer
}

