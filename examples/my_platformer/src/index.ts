import { World, readSettings } from "fruity_game_engine";
import fruityEcs from "fruity_ecs";
import myPlatformer from "./myPlatformer";

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

class ComponentTest {
  value: number = 3;
}

entityService.create("test entity", true, [new ComponentTest()]);

// Run the world
world.run();
