import {
  ResourceReference,
  Module,
} from "fruity_game_engine"

export type MaterialParam =
  | { type: 'uint', value: number }
  | { type: 'int', value: number }
  | { type: 'float', value: number }
  | { type: 'vector2d', value: Vector2D }
  | { type: 'color', value: Color }
  | { type: 'rect', value: {
    bottomLeft: Vector2D,
    topRight: Vector2D,
  } }
  | { type: 'matrix4', value: Matrix4 }

export type ShaderBindingVisibility = "vertex" | "fragment"
export type ShaderBindingType = "texture" | "sampler" | "uniform"
export type ShaderInstanceAttributeType = "int" | "uint" | "float" | "vector2D" | "vector4D"
export class Color {
  r: number
  g: number
  b: number
  a: number
  constructor(r: number, g: number, b: number, a: number)
}

export class GraphicService {

  startDraw(): void
  endDraw()
  renderScene(viewProj: Matrix4, backgroundColor: Color, target?: ResourceReference<TextureResource> | null | undefined | void)
  getCameraTransform(): Matrix4
  resize(width: number, height: number)
  worldPositionToViewportPosition(pos: Vector2D): [number, number]
  viewportPositionToWorldPosition(x: number, y: number): Vector2D
  getCursorPosition(): Vector2D
  isCursorHoverScene(): boolean
  getViewportOffset(): [number, number]
  setViewportOffset(x: number, y: number)
  getViewportSize(): [number, number]
  setViewportSize(x: number, y: number)
}

export class MaterialResource {

}

export class Matrix3 {
  0: number[][]
  translation(): Vector2D
  rotation(): number
  scale(): Vector2D
  invert(): Matrix3
}

export class Matrix4 {
  0: number[][]
  invert(): Matrix4
}

export class MeshResource {

}

export class MeshResourceSettings {
  vertices: Vertex[]
  indices: number[]
}

export interface ShaderBinding {
  visibility: ShaderBindingVisibility
  ty: ShaderBindingType
}

export interface ShaderBindingGroup {
  bindings: ShaderBinding[]
}

export interface ShaderInstanceAttribute {
  location: number
  ty: ShaderInstanceAttributeType
}

export class ShaderResource {

}

export interface ShaderResourceSettings {
  bindingGroups: ShaderBindingGroup[]
  instanceAttributes: ShaderInstanceAttribute[]
}

export class TextureResource {

  getSize(): [number, number]
}

export class Vector2D {
  x: number
  y: number
  constructor(x: number, y: number)
  horizontal(): Vector2D
  vertical(): Vector2D
  abs(): Vector2D
  normal(): Vector2D
  dot(v2: Vector2D): number
  lengthSquared(): number
  lerp(end: Vector2D, progress: number): Vector2D
  length(): number
  normalize(): Vector2D
  angle(): number
  inTriangle(p1: Vector2D, p2: Vector2D, p3: Vector2D): boolean
  inCircle(center: Vector2D, radius: number): boolean
  add(rhs: Vector2D): Vector2D
  sub(rhs: Vector2D): Vector2D
  mul(rhs: number): Vector2D
  div(rhs: number): Vector2D
}

export class Vector3D {
  x: number
  y: number
  z: number
  constructor(x: number, y: number, z: number)
  horizontal(): Vector3D
  vertical(): Vector3D
  depth(): Vector3D
  dot(v2: Vector3D): number
  lengthSquared(): number
  lerp(end: Vector3D, progress: number): Vector3D
  length(): number
  normalize(): Vector3D
  add(rhs: Vector3D): Vector3D
  sub(rhs: Vector3D): Vector3D
  mul(rhs: number): Vector3D
  div(rhs: number): Vector3D
}

export class Vertex {
  position: Vector3D
  texCoords: Vector2D
  normal: Vector3D
}

export function createFruityGraphicModule(): Module
