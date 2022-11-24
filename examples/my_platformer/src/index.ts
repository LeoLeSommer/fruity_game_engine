import { World, readSettings } from "fruity_game_engine";
import fruityEcs from "fruity_ecs";
import myPlatformer from "./myPlatformer";

console.log("Hello world!");

const settings = readSettings("./assets/settings.yaml");
const world = new World(settings);

console.log(fruityEcs);

world.registerModule(fruityEcs);
world.registerModule(myPlatformer);

world.setupModules();
world.loadResources();
world.run();
