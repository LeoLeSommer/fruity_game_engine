import {
  Module,
} from "fruity_game_engine"

import {
  Vector2D,
} from "fruity_graphic"

export class CircleCollider {
  center: Vector2D
  radius: number
  constructor(center: Vector2D, radius: number)
}

export class RectCollider {
  bottomLeft: Vector2D
  topRight: Vector2D
  constructor(bottomLeft: Vector2D, topRight: Vector2D)
}

export function createFruityPhysic2DModule(): Module
