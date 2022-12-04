import {
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
