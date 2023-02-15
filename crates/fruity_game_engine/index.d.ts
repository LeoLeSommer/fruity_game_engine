export type ScriptValue = any;
export type ScriptOrNativeResource = any;
export type JsIntrospectObject = { [key: string]: any };

export type SettingsElem =
  | number
  | boolean
  | string
  | SettingsElem[]
  | Settings
  | null;

export type Settings = { [key: string]: SettingsElem };

export interface FrameService {
  getDelta(): number;
}

export interface ObjectFactoryService {
  instantiate(objectType: string, args: ScriptValue[]): ScriptValue | null;
}

export interface Module {
  name: string;
  dependencies: string[];
  setup: (world: World, settings: Settings) => void;
  loadResources: ((world: World, settings: Settings) => void) | null;
}

export class World {
  constructor(settings: Settings);
  registerModule(module: Module);
  setupModules();
  loadResources();
  run();
  getResourceContainer(): ScriptResourceContainer;
}

export interface ModuleService {
  registerModule(module: Module);
}

export interface ScriptResourceContainer {
  get<T>(identifier: string): T;
  contains(identifier: string): boolean;
  add(identifier: string, resource: JsIntrospectObject): boolean;
  remove(identifier: string);
}

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

export type ResourceReference<T> = T;
