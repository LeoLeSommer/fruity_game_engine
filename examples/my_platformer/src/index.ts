import { World } from "fruity_game_engine";
import { createFruityEcsModule } from "fruity_ecs";
import { createFruityHierarchyModule } from "fruity_hierarchy";

class CustomService {
  index = 1;

  hello(str: string) {
    console.log("Hello", str, this.index++);
  }
}

export class CustomComponent {
  constructor(args?: Partial<CustomComponent>) {
    Object.assign(this, args);
  }

  value: number = 3;
}

export class CustomComponent2 {
  constructor(args?: Partial<CustomComponent2>) {
    Object.assign(this, args);
  }

  value: number = 30;
}

const world = new World({});

world.registerModule(createFruityEcsModule());
world.registerModule(createFruityHierarchyModule());
world.registerModule({
  name: "my_platformer",
  dependencies: ["fruity_ecs", "fruity_hierarchy"],
  setup: (world: any) => {
    console.log("setup", world);
    const resourceContainer = world.getResourceContainer();
    resourceContainer.add("custom_service", new CustomService());

    const customService = resourceContainer.get("custom_service");
    console.log("customService", customService);

    customService.hello("Frame");

    const systemService = resourceContainer.get("system_service");
    const entityService = resourceContainer.get("entity_service");

    systemService.addStartupSystem(
      "test startup 1",
      () => {
        console.log("The engine has been turned on");

        return () => {
          console.log("The engine will be turned off");
        };
      },
      {
        ignorePause: true,
      }
    );

    systemService.addSystem("test 1", () => {
      console.log("A frame is rendered");

      entityService
        .query()
        .with("CustomComponent")
        .with("CustomComponent2")
        .forEach(([customComponent, customComponent2]: any[]) => {
          console.log(
            "Component",
            customComponent.value,
            customComponent2.value
          );
        });
    });
  },
  loadResources: (resourceContainer: any, settings: any) => {
    console.log("loadResources");
  },
});

// Setup the scene
const resourceContainer = world.getResourceContainer();
const entityService = resourceContainer.get("entity_service");

entityService.create("test entity", true, [
  new CustomComponent(),
  new CustomComponent({ value: 1 }),
  new CustomComponent2({ value: 144 }),
]);

// Run the world
world.run();
