import * as nativeBinding from './pkg/index.js'
import { setBundle, getBundle } from 'fruity_game_engine'

export default function initFruityBundle() {
  setBundle(nativeBinding)
}

export function createFruityNativeBundleModule(...args) {
  return getBundle().createFruityNativeBundleModule(...args)
}