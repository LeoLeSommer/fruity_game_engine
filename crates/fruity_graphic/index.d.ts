import {
  ResourceReference,
  Module,
} from "fruity_game_engine"

export type MaterialParam =
  | { type: 'uint', value: number }
  | { type: 'int', value: number }
  | { type: 'float', value: number }
  | { type: 'vector2d', value: Vector2d }
  | { type: 'color', value: Color }
  | { type: 'rect', value: {
    bottomLeft: Vector2d,
    topRight: Vector2d,
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

export interface GraphicService {
  startDraw()
  endDraw()
  renderScene(viewProj: Matrix4, backgroundColor: Color, target?: ResourceReference<TextureResource> | null | undefined)
  getCameraTransform(): Matrix4
  resize(width: number, height: number)
  worldPositionToViewportPosition(pos: Vector2d): [number, number]
  viewportPositionToWorldPosition(x: number, y: number): Vector2d
  getCursorPosition(): Vector2d
  isCursorHoverScene(): boolean
  getViewportOffset(): [number, number]
  setViewportOffset(x: number, y: number)
  getViewportSize(): [number, number]
  setViewportSize(x: number, y: number)
}

export interface MaterialResource {
}

export interface Matrix3 {
  0: number[][]
  translation(): Vector2d
  rotation(): number
  scale(): Vector2d
  invert(): Matrix3
}

export interface Matrix4 {
  0: number[][]
  invert(): Matrix4
}

export interface MeshResource {
}

export interface MeshResourceSettings {
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

export interface ShaderResource {
}

export interface ShaderResourceSettings {
  bindingGroups: ShaderBindingGroup[]
  instanceAttributes: ShaderInstanceAttribute[]
}

export interface TextureResource {
  getSize(): [number, number]
}

export class Vector2d {
  x: number
  y: number
  constructor(x: number, y: number)
  horizontal(): Vector2d
  vertical(): Vector2d
  abs(): Vector2d
  normal(): Vector2d
  dot(v2: Vector2d): number
  lengthSquared(): number
  lerp(end: Vector2d, progress: number): Vector2d
  length(): number
  normalize(): Vector2d
  angle(): number
  inTriangle(p1: Vector2d, p2: Vector2d, p3: Vector2d): boolean
  inCircle(center: Vector2d, radius: number): boolean
  add(rhs: Vector2d): Vector2d
  sub(rhs: Vector2d): Vector2d
  mul(rhs: number): Vector2d
  div(rhs: number): Vector2d
}

export class Vector3d {
  x: number
  y: number
  z: number
  constructor(x: number, y: number, z: number)
  horizontal(): Vector3d
  vertical(): Vector3d
  depth(): Vector3d
  dot(v2: Vector3d): number
  lengthSquared(): number
  lerp(end: Vector3d, progress: number): Vector3d
  length(): number
  normalize(): Vector3d
  add(rhs: Vector3d): Vector3d
  sub(rhs: Vector3d): Vector3d
  mul(rhs: number): Vector3d
  div(rhs: number): Vector3d
}

export interface Vertex {
  position: Vector3d
  texCoords: Vector2d
  normal: Vector3d
}

export function createFruityGraphicModule(): Module
