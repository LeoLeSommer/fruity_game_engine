import { World, read_settings } from "fruity_game_engine";
import myPlatformer from "./myPlatformer";

console.log("Hello world!");

const settings = read_settings("./assets/settings.yaml");
const world = new World(settings);

console.log("World", world);

console.log("register_module");
world.register_module(myPlatformer);
console.log("setup_modules");
world.setup_modules();
console.log("end");

// world.registerModule(fruityEcs);

/*console.log("run_setup");
world.setup_modules((...args: any) => {
  console.log("setup_end", args);
})*/
/*console.log("run_load_resources", test);
world.load_resources();

console.log("run_run");
world.run();

console.log("run_end");
*/
