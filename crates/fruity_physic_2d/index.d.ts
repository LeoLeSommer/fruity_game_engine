import {
  Module,
} from "fruity_game_engine"

import {
  Vector2d,
} from "fruity_graphic"

export class CircleCollider {
  center: Vector2d
  radius: number
  constructor(center: Vector2d, radius: number)
}

export class RectCollider {
  bottomLeft: Vector2d
  topRight: Vector2d
  constructor(bottomLeft: Vector2d, topRight: Vector2d)
}

export function createFruityPhysic2DModule(): Module
