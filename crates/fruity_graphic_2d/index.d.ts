import {
  ResourceReference,
  Module,
} from "fruity_game_engine"

import {
  TextureResource,
  Color,
  MaterialResource,
  MaterialParam,
  Vector2d,
  Matrix3,
} from "fruity_graphic"

export class Camera {
  near: number
  far: number
  target: ResourceReference<TextureResource> | null | undefined
  backgroundColor: Color
  constructor()
}

export interface Graphic2dService {
  drawQuad(identifier: number, material: ResourceReference<MaterialResource>, params: {[key: string]: MaterialParam}, zIndex: number)
  drawLine(pos1: Vector2d, pos2: Vector2d, width: number, color: Color, zIndex: number, transform: Matrix3)
  drawPolyline(points: Vector2d[], width: number, color: Color, zIndex: number, transform: Matrix3)
  drawDottedLine(pos1: Vector2d, pos2: Vector2d, width: number, color: Color, zIndex: number, transform: Matrix3)
  drawRect(bottomLeft: Vector2d, topRight: Vector2d, width: number, fillColor: Color, borderColor: Color, zIndex: number, transform: Matrix3)
  drawArc(center: Vector2d, radius: number, angleRange: [number, number], width: number, fillColor: Color, borderColor: Color, zIndex: number, transform: Matrix3)
  drawCircle(center: Vector2d, radius: number, width: number, fillColor: Color, borderColor: Color, zIndex: number, transform: Matrix3)
}

export class Rotate2d {
  angle: number
  constructor(angle: number)
}

export class Scale2d {
  vec: Vector2d
  constructor(vec: Vector2d)
}

export class Sprite {
  material: ResourceReference<MaterialResource> | null | undefined
  texture: ResourceReference<TextureResource> | null | undefined
  zIndex: number
  constructor(material?: ResourceReference<MaterialResource> | null | undefined, texture?: ResourceReference<TextureResource> | null | undefined, zIndex: number)
}

export class Transform2d {
  transform: Matrix3
  constructor()
}

export class Translate2d {
  vec: Vector2d
  constructor(vec: Vector2d)
}

export function createFruityGraphic2DModule(): Module
