import { World, readSettings } from "fruity_game_engine";
import fruityEcs from "fruity_ecs";
import myPlatformer, { CustomComponent } from "./myPlatformer";

console.log("Hello world!");

const settings = readSettings("./assets/settings.yaml");
const world = new World(settings);

console.log(fruityEcs);

// Register the modules
world.registerModule(fruityEcs);
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
]);

// Run the world
world.run();
