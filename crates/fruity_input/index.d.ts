import {
  Signal,
  Module,
} from "fruity_game_engine"

export interface DragService {

}

export interface InputService {
  inputMap: {[key: string]: string}
  pressedInputs: string[]
  pressedSources: string[]
  pressedModifiers: Modifiers
  pressedThisFrameInputs: string[]
  pressedThisFrameSources: string[]
  releasedThisFrameInputs: string[]
  releasedThisFrameSources: string[]
  onPressed: Signal<string>
  onReleased: Signal<string>
  registerInput(input: string, source: string)
  isPressed(input: string): boolean
  isSourcePressed(source: string): boolean
  isPressedThisFrame(input: string): boolean
  isKeyboardPressedThisFrame(source: string): boolean
  isSourcePressedThisFrame(source: string): boolean
  isReleasedThisFrame(input: string): boolean
  isSourceReleasedThisFrame(source: string): boolean
  notifyPressed(source: string): void
  notifyReleased(source: string): void
  handleFrameEnd()
}

export interface Modifiers {
  0: number
  hasShift(): boolean
  hasCtrl(): boolean
  hasAlt(): boolean
  hasLogo(): boolean
}

export function createFruityInputModule(): Module
