import {
  Module,
  ScriptObject,
  Signal,
} from "fruity_game_engine"

export interface DialogService {

  save(defaultFilename: string, fileTypes: string[]): string | null
  open(fileTypes: string[]): string | null
}

export interface EditorComponentService {

}

export interface EditorMenuService {

}

export interface EditorPanelsService {

}

export interface FileExplorerService {

}

export interface FileExplorerState {

}

export interface InspectorService {

}

export interface InspectorState {
  onSelected: Signal<ScriptObject>
  onUnselected: Signal<void>
  select(selection: ScriptObject): void
  unselect(): void
  isGizmosEnabled(): boolean
  temporaryDisplayGizmos()
}

export interface IntrospectEditorService {

}

export interface MutationService {

  undo(): void
  redo(): void
  canUndo(): boolean
  canRedo(): boolean
}

export interface SceneState {

  run(): void
  pause(): void
  stop(): void
  isRunning(): boolean
  canStop(): boolean
  open(): void
}

export function createEditorModule(): Module
