import * as test1 from "fruity_game_engine";
import * as test2 from "fruity_ecs";
import * as test3 from "fruity_hierarchy";

console.log(test1, test2, test3);

/*import {
  World,
  readSettings,
  createFruityEcsModule,
  createFruityHierarchyModule,
} from "fruity_game_engine_bundle";

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

const world = new (World as any)({});

world.registerModule(createFruityEcsModule());
world.registerModule({
  name: "my_platformer",
  dependencies: ["fruity_ecs"],
  setup: (world: any) => {
    console.log("setup", world);
    const resourceContainer = world.getResourceContainer();*/

/*resourceContainer.add("custom_service", new CustomService());

    const customService = resourceContainer.get("custom_service");
    console.log("customService", customService);

    customService.hello("Frame");*/

/*const systemService = resourceContainer.get("system_service");
    const entityService = resourceContainer.get("entity_service");*/

/*systemService.addStartupSystem(
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
    });*/
/*},
  loadResources: (resourceContainer: any, settings: any) => {
    console.log("loadResources");
  },
});

world.setupModules();
world.loadResources();*/

// Setup the scene
/*const resourceContainer = world.getResourceContainer();
const entityService = resourceContainer.get("entity_service");

entityService.create("test entity", true, [
  new CustomComponent(),
  new CustomComponent({ value: 1 }),
  new CustomComponent2({ value: 144 }),
]);*/

// Run the world
// world.run();

/*import * as test from "fruity_game_engine_bundle";
console.log(test);*/

/*import {
  World,
  readSettings,
  createFruityEcsModule,
  createFruityHierarchyModule,
} from "fruity_game_engine_bundle";
import myPlatformer, {
  CustomComponent,
  CustomComponent2,
} from "./myPlatformer";

// const settings = Test.readSettings("./assets/settings.yaml");
const world = new (World as any)({});
console.log(world);

// Register the modules
world.registerModule(createFruityEcsModule());
world.registerModule(createFruityHierarchyModule());
world.registerModule(myPlatformer);

// Setup the world
world.setupModules();
world.loadResources();

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
*/
