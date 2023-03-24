# FEATURES V1

## Build script

[X] Generate js import files for modules
[X] Generate ts typedef for modules
[ ] Automate ts import generation
[ ] replaces export_struct, export_impl ... by a common fruity_export

## Script Language Interface

[X] WASM Module
[X] WASM doesn't support multithread, so adapt the code to this constraint
[X] NAPI Module
[ ] A bug remains due to the multithread functions issues, impossible to do onCreated query with a finalizer

## ECS

[X] Make the archetype able to store datas in plain array for performances
[X] Put a get_constructor function into introspect
[X] Implements entity deletion
[X] Resources and services containers should be merged
[X] Abstract resources
[X] Make native systems easy to use
[X] Move ECS structure to a struct of array instead an array of struct (more efficient, easier to implement)
[X] Implement ECS hierarchy
[X] Add a way to have multiple components of the same type on a single entity
[X] There should be some memory leak in archetype
[X] Add component query selectors
[X] Create a way to query based on generics
[X] Change the begin/end system to a startup system that returns an optional end callback and add the startup that ignore pause
[X] Add a way to extend components
[ ] Support for async systems/queries for_each with tokio
[ ] Find a way to move the introspect declaration into the associated traits
[ ] Allow to access entities created in the current frame

## Editor

[X] Base editor
[X] Implement a basic hook system
[X] Create a pseudo DOM to abstract the interface creation
[X] Wrap the DOM system with egui
[X] Components visualization
[X] Entity hierarchy visualisation
[X] Resources browser
[X] Sprite gizmos
[X] Run/Pause
[X] Save/Load
[X] Add component from editor
[X] Remove component from editor
[X] Fix resize gizmos
[X] Add a cool grid for 2D view
[ ] Expose the GUI API to javascript to create custom component editor
[ ] Resources visualisation, take the material as an example and try to make it easy to edit in sprite component
[ ] Select an object when clicking on it (set_scissor_rect)
[ ] Add a cool free camera for 2D view

## Graphics

[X] Material should store wgpu bind groups
[X] Supports Meshes
[X] Material fields should now be a string identifier
[X] Sprite vertex/indices should be shared accross all sprites
[X] Squad transform should be done in shader instead of CPU
[X] Proceed instantied rendering
[X] Make instances parametrizable in material/shader
[ ] Multi-pass renderer
[ ] Implements spritesheet
[ ] Implement rendering composers

## Animation

[ ] Add an optional way to interpolate between serialized values
[ ] Add an animation system with keyframes
[ ] Create an editor for keyframes
[ ] Create a state system
[ ] Create interpolation between two states
[ ] Create an editor for states

## Physics 2D

[X] Primitive physics collider components (shared accross every physics engine)
[ ] Mesh physics collider components (shared accross every physics engine)
[ ] Implements basic Rapier crate features
[ ] Implements a collision only physic engine

## Game tools

[X] Inputs
[X] Time service
[ ] Tiles editor (make something like RPG maker, as easy to use as possible)
[ ] Particles

## Nice to have

[ ] FreeSpriteSpline (something like spriteshape for lines but more easy to use, width should be modifiable, thought to be used with a graphic tablet)
[ ] FreeSpriteShape (something like spriteshape but more easy to use, thought to be used with a graphic tablet)
[ ] 2D skeletons (take inspiration with unity's one wich is realy nice)
[ ] Implements a complete physic engine
[ ] 2D lights

## Others

[X] Implements a profiling tool
[ ] Implement a basic sound features

## Code clean

[ ] Put an auto-analyser and clean the code
[ ] Tests everywhere
[ ] Rust doc everywhere
[ ] Remove as many unwrap as possible
[ ] A lot of unsafe code were created to avoid lifetime issue in ecs, remove as many as possible
[ ] Use a tool that detect unused dependencies

# FEATURES V2

## For the future

[ ] Unreal released a new version of there engine with something called the Nanites, try to make some research about it (https://youtu.be/TMorJX3Nj6U)
