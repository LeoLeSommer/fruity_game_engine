import { World, readSettings } from "fruity_game_engine";
import fruityEcs from "fruity_ecs";
import myPlatformer from "./myPlatformer";

console.log("Hello world!");

console.log("1");
const settings = readSettings("./assets/settings.yaml");
console.log("2");
const world = new World(settings);
console.log("3");

console.log(fruityEcs);

world.registerModule(fruityEcs);
world.registerModule(myPlatformer);

world.setupModules();
world.loadResources();
world.run();
