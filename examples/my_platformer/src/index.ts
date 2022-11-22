import { World, readSettings } from "fruity_game_engine";
import myPlatformer from "./myPlatformer";

console.log("Hello world!");

const settings = readSettings("./assets/settings.yaml");
const world = new World(settings);

world.registerModule(myPlatformer);
world.setupModules();
world.loadResources();
world.run();
