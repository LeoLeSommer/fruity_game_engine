export * from './pkg/index.js'
import { name, dependencies, setup } from './pkg/index.js'

export default {
    name: name(),
    dependencies: dependencies(),
    setup: setup,
}