import * as nativeBinding from './pkg/index.js'
import { setBundle } from 'fruity_game_engine'

export default function initFruityBundle() {
  setBundle(nativeBinding)
}