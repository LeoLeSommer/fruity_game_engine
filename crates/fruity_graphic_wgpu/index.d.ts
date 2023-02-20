import {
  Signal,
  ResourceReference,
  Module,
} from "fruity_game_engine"

import {
  Matrix4,
  Color,
  TextureResource,
  Vector2d,
  ShaderResource,
  MeshResourceSettings,
  ShaderResourceSettings,
} from "fruity_graphic"

export interface WgpuGraphicService {
  onBeforeDrawEnd: Signal<void>
  onAfterDrawEnd: Signal<void>
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

export interface WgpuMaterialResource {
  getShader(): ResourceReference<ShaderResource> | null
}

export interface WgpuMeshResource {
  params: MeshResourceSettings
}

export interface WgpuShaderResource {
  params: ShaderResourceSettings
}

export interface WgpuTextureResource {
  getSize(): [number, number]
}

export function createFruityGraphicWgpuModule(): Module
