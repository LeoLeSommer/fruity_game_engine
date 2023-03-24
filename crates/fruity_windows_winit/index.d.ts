import {
  Signal,
  Module,
} from "fruity_game_engine"

export interface WinitWindowService {
  cursorPosition: [number, number]
  onStartUpdate: Signal<void>
  onEndUpdate: Signal<void>
  onResize: Signal<[number, number]>
  onCursorMoved: Signal<[number, number]>
  onEventsCleared: Signal<void>
}

export function createFruityWindowsWinitModule(): Module
