import initFruityBundle from "fruity_native_bundle";
import { FrameService, Settings, World } from "fruity_game_engine";
import {
  createFruityEcsModule,
  EntityService,
  SystemService,
} from "fruity_ecs";
import {
  createFruityGraphicModule,
  MaterialResource,
  Vector2D,
} from "fruity_graphic";
import {
  createFruityGraphic2DModule,
  Rotate2D,
  Sprite,
  Transform2D,
  Translate2D,
} from "fruity_graphic_2d";
import { createFruityGraphicWgpuModule } from "fruity_graphic_wgpu";
import { createFruityHierarchyModule } from "fruity_hierarchy";
import { createFruityHierarchy2DModule } from "fruity_hierarchy_2d";
import { createFruityInputModule, InputService } from "fruity_input";
import { createFruityInputWinitModule } from "fruity_input_winit";
import { createFruityPhysic2DModule } from "fruity_physic_2d";
import { createFruityPhysicParry2DModule } from "fruity_physic_parry_2d";
import { createFruityWindowsModule } from "fruity_windows";
import { createFruityWindowsWinitModule } from "fruity_windows_winit";

import settings from "./assets/settings.json" assert { type: "json" };
import scene from "./assets/scene.json" assert { type: "json" };

initFruityBundle();

class Move {
  constructor(args: Partial<Move>) {
    Object.assign(this, args);
  }

  velocity = 1.0;
}

class Velocity {
  constructor(args: Partial<Velocity>) {
    Object.assign(this, args);
  }

  velocity = 1.0;
}

const world = new World(settings as any);

world.registerModule(createFruityEcsModule());
world.registerModule(createFruityGraphicModule());
world.registerModule(createFruityGraphic2DModule());
world.registerModule(createFruityGraphicWgpuModule());
world.registerModule(createFruityHierarchyModule());
world.registerModule(createFruityHierarchy2DModule());
world.registerModule(createFruityInputModule());
world.registerModule(createFruityInputWinitModule());
world.registerModule(createFruityPhysic2DModule());
world.registerModule(createFruityPhysicParry2DModule());
world.registerModule(createFruityWindowsModule());
world.registerModule(createFruityWindowsWinitModule());

world.registerModule({
  name: "my_platformer",
  dependencies: [
    "fruity_ecs",
    "fruity_graphic",
    "fruity_graphic_2d",
    "fruity_hierarchy",
    "fruity_hierarchy_2d",
    "fruity_input",
    "fruity_physic_2d",
    "fruity_physic_parry_2d",
    "fruity_windows",
  ],
  setup: (world: World) => {
    const resourceContainer = world.getResourceContainer();
    const inputService =
      resourceContainer.require<InputService>("input_service");
    const systemService =
      resourceContainer.require<SystemService>("system_service");
    const frameService =
      resourceContainer.require<FrameService>("frame_service");
    const entityService =
      resourceContainer.require<EntityService>("entity_service");

    systemService.addStartupSystem(
      "initialize entities save",
      () => {
        entityService.restore(scene);
      },
      {
        ignorePause: true,
      }
    );

    systemService.addStartupSystem(
      "test startup 0",
      () => {
        console.log("Je commence tout");

        return () => {
          console.log("Je finis tout");
        };
      },
      {
        ignorePause: true,
      }
    );

    systemService.addStartupSystem("test startup 1", () => {
      console.log("Je commence");

      return () => {
        console.log("Je finis");
      };
    });

    /*systemService.addStartupSystem("test startup 2", () => {
      let handle = entityService
        .query()
        .withName()
        .with("Translate2D")
        .with("Velocity")
        .onCreated((name) => {
          console.log(`Entity created ${name}`);

          return () => {
            console.log(`Entity deleted ${name}`);
          };
        });

      return () => {
        handle.dispose();
      };
    });*/

    /*systemService.addStartupSystem("test startup 3", () => {
      let createdEntityId: number | null = null;
      const materialResource = resourceContainer.get<MaterialResource>(
        "./src/assets/material.material"
      );

      let handle1 = inputService.onPressed.addObserver((input) => {
        if (input === "Action 1") {
          createdEntityId = entityService.create("New Entity", true, [
            new Transform2D(),
            new Sprite(materialResource, null, 30),
            new Translate2D(new Vector2D(1, 1)),
            new Velocity({ velocity: 1.0 }),
          ]);
        }
      });

      let handle2 = inputService.onReleased.addObserver((input) => {
        if (input === "Action 1" && createdEntityId) {
          entityService.remove(createdEntityId);
        }
      });

      return () => {
        handle1.dispose();
        handle2.dispose();
      };
    });*/

    systemService.addSystem("test 1", () => {
      entityService
        .query()
        .with<Translate2D>("Translate2D")
        .with<Velocity>("Velocity")
        .forEach(([translate, velocity]) => {
          const beforeTranslate = translate.vec;
          translate.vec = beforeTranslate.add(
            beforeTranslate.mul(velocity.velocity * frameService.getDelta())
          );
        });
    });

    systemService.addSystem("test 2", () => {
      entityService
        .query()
        .withEntity()
        .with<Translate2D>("Translate2D")
        .with<Move>("Move")
        .forEach(([entity, translate, move]) => {
          let vel = new Vector2D(0, 0);
          if (inputService.isPressed("Run Left")) {
            vel.x -= move.velocity;
          }

          if (inputService.isPressed("Run Right")) {
            vel.x += move.velocity;
          }

          if (inputService.isPressed("Jump")) {
            vel.y += move.velocity;
          }

          if (inputService.isPressed("Down")) {
            vel.y -= move.velocity;
          }

          translate.vec = translate.vec.add(vel.mul(frameService.getDelta()));
        });
    });

    systemService.addSystem("test 3", () => {
      entityService
        .query()
        .with<Rotate2D>("Rotate2D")
        .with<Move>("Move")
        .forEach(([rotate, move]) => {
          if (inputService.isPressed("Rotate")) {
            rotate.angle += move.velocity * frameService.getDelta();
          }
        });
    });
  },
  loadResources: (world: World, settings: Settings) => {
    console.log("loadResources");
    const resourceContainer = world.getResourceContainer();
    resourceContainer.loadResourcesSettings(settings);
  },
});

// Run the world
world.run();
