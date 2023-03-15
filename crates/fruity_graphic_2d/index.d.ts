import {
  ResourceReference,
  Module,
} from "fruity_game_engine"

import {
  TextureResource,
  Color,
  MaterialResource,
  MaterialParam,
  Vector2D,
  Matrix3,
} from "fruity_graphic"

export class Camera {
  near: number
  far: number
  target?: ResourceReference<TextureResource> | null | undefined | void
  backgroundColor: Color
  constructor()
}

export interface Graphic2dService {

  drawQuad(identifier: number, material: ResourceReference<MaterialResource>, params: {[key: string]: MaterialParam}, zIndex: number)
  drawLine(pos1: Vector2D, pos2: Vector2D, width: number, color: Color, zIndex: number, transform: Matrix3)
  drawPolyline(points: Vector2D[], width: number, color: Color, zIndex: number, transform: Matrix3)
  drawDottedLine(pos1: Vector2D, pos2: Vector2D, width: number, color: Color, zIndex: number, transform: Matrix3)
  drawRect(bottomLeft: Vector2D, topRight: Vector2D, width: number, fillColor: Color, borderColor: Color, zIndex: number, transform: Matrix3)
  drawArc(center: Vector2D, radius: number, angleRange: [number, number], width: number, fillColor: Color, borderColor: Color, zIndex: number, transform: Matrix3)
  drawCircle(center: Vector2D, radius: number, width: number, fillColor: Color, borderColor: Color, zIndex: number, transform: Matrix3)
}

export class Rotate2D {
  angle: number
  constructor(angle: number)
}

export class Scale2D {
  vec: Vector2D
  constructor(vec: Vector2D)
}

export class Sprite {
  material?: ResourceReference<MaterialResource> | null | undefined | void
  texture?: ResourceReference<TextureResource> | null | undefined | void
  zIndex: number
  constructor(material: ResourceReference<MaterialResource> | null | undefined | void, texture: ResourceReference<TextureResource> | null | undefined | void, zIndex: number)
}

export class Transform2D {
  transform: Matrix3
  constructor()
}

export class Translate2D {
  vec: Vector2D
  constructor(vec: Vector2D)
}

export function createFruityGraphic2DModule(): Module
