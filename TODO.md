# FEATURES V1

## Build script

- [x] Generate js import files for modules
- [x] Generate ts typedef for modules
- [x] Automate ts import generation
- [ ] replaces export_struct, export_impl ... by a common fruity_export

## Script Language Interface

- [x] WASM Module
- [x] WASM doesn't support multithread, so adapt the code to this constraint
- [x] NAPI Module
- [ ] A bug remains due to the multithread functions issues, impossible to do onCreated query with a finalizer
- [ ] Introduce reference script values, remove clone in script value, add a lifetime on ScriptValue
- [ ] Reduce wasm cost
- [ ] Add FinalizationRegistry in wasm to manage rust object free https://github.com/tc39/proposal-weakrefs
- [ ] Reduce napi cost, use napi_sys instead of napi crate
- [ ] Use int id instead of string to access members in introspection

## ECS

- [x] Make the archetype able to store datas in plain array for performances
- [x] Put a get_constructor function into introspect
- [x] Implements entity deletion
- [x] Resources and services containers should be merged
- [x] Abstract resources
- [x] Make native systems easy to use
- [x] Move ECS structure to a struct of array instead an array of struct (more efficient, easier to implement)
- [x] Implement ECS hierarchy
- [x] Add a way to have multiple components of the same type on a single entity
- [x] There should be some memory leak in archetype
- [x] Add component query selectors
- [x] Create a way to query based on generics
- [x] Change the begin/end system to a startup system that returns an optional end callback and add the startup that ignore pause
- [x] Add a way to extend components
- [ ] Find a way to move the introspect declaration into the associated traits
- [ ] Allow to access entities created in the current frame
- [ ] Put permission into the component type instead of the entity
- [ ] Use uint id instead of string for component types
- [ ] Separate data (with) and filter (without) in queries
- [ ] Remove the pointer reallocation signals, replace it with shared containers
- [ ] Re-parallelize archetype mutations
- [ ] Reintroduce entity mutations in the frame
- [ ] Only one pool per frame
- [ ] Cache queries
- [ ] Support for async systems/queries for_each with tokio

## Editor

- [x] Base editor
- [x] Implement a basic hook system
- [x] Create a pseudo DOM to abstract the interface creation
- [x] Wrap the DOM system with egui
- [x] Components visualization
- [x] Entity hierarchy visualisation
- [x] Resources browser
- [x] Sprite gizmos
- [x] Run/Pause
- [x] Save/Load
- [x] Add component from editor
- [x] Remove component from editor
- [x] Fix resize gizmos
- [x] Add a cool grid for 2D view
- [ ] Expose the GUI API to javascript to create custom component editor
- [ ] Resources visualisation, take the material as an example and try to make it easy to edit in sprite component
- [ ] Select an object when clicking on it (set_scissor_rect)
- [ ] Add a cool free camera for 2D view

## Graphics

- [x] Material should store wgpu bind groups
- [x] Supports Meshes
- [x] Material fields should now be a string identifier
- [x] Sprite vertex/indices should be shared accross all sprites
- [x] Squad transform should be done in shader instead of CPU
- [x] Proceed instantied rendering
- [x] Make instances parametrizable in material/shader
- [ ] Put all render operation in a RenderWorld, parallelize the rendering https://www.gdcvault.com/play/1021926/Destiny-s-Multithreaded-Rendering
- [ ] Multi-pass renderer
- [ ] Implements spritesheet
- [ ] Implement rendering composers

## Animation

- [ ] Add an optional way to interpolate between serialized values
- [ ] Add an animation system with keyframes
- [ ] Create an editor for keyframes
- [ ] Create a state system
- [ ] Create interpolation between two states
- [ ] Create an editor for states

## Physics 2D

- [x] Primitive physics collider components (shared accross every physics engine)
- [ ] Mesh physics collider components (shared accross every physics engine)
- [ ] Implements basic Rapier crate features
- [ ] Implements a collision only physic engine

## Game tools

- [x] Inputs
- [x] Time service
- [ ] Tiles editor (make something like RPG maker, as easy to use as possible)
- [ ] Particles

## Nice to have

- [ ] FreeSpriteSpline (something like spriteshape for lines but more easy to use, width should be modifiable, thought to be used with a graphic tablet)
- [ ] FreeSpriteShape (something like spriteshape but more easy to use, thought to be used with a graphic tablet)
- [ ] 2D skeletons (take inspiration with unity's one wich is realy nice)
- [ ] Implements a complete physic engine
- [ ] 2D lights

## Others

- [x] Implements a profiling tool
- [ ] Implement a basic sound features

## Code clean

- [ ] Put an auto-analyser and clean the code
- [ ] Tests everywhere
- [ ] Rust doc everywhere
- [ ] Remove as many unwrap as possible
- [ ] A lot of unsafe code were created to avoid lifetime issue in ecs, remove as many as possible
- [ ] Use a tool that detect unused dependencies

# FEATURES V2

## For the future

- [ ] Unreal released a new version of there engine with something called the Nanites, try to make some research about it (https://youtu.be/TMorJX3Nj6U)
