import { World, read_settings } from "fruity_game_engine";
import myPlatformer from "./myPlatformer";

console.log("Hello world!");

const settings = read_settings("./assets/settings.yaml");
const world = new World(settings);
const resourceContainer = world.get_resource_container();
/*const frameService = resourceContainer.get_untyped("frame_service");

frameService.set_delta(10);
console.log("World", frameService.get_delta());*/

world.register_module(myPlatformer);

world.setup_modules();
console.log("run_1");
world.load_resources();
console.log("run_2");
world.run();
console.log("run_3");
