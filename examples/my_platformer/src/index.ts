import test from "fruity_game_engine";
console.log("Hello world!", test);

(test as any)
  .then((m: any) => {
    m.initPanicHook();

    console.log("Hello world!", m);
    const settings = m.readSettings("/assets/settings.yaml");
    console.log(settings);
  })
  .catch(console.error);

/*import Test from "fruity_game_engine";
import fruityEcs from "fruity_ecs";
import fruityHierarchy from "fruity_hierarchy";
import myPlatformer, {
  CustomComponent,
  CustomComponent2,
} from "./myPlatformer";

console.log("Hello world!", Test);

const settings = Test.readSettings("./assets/settings.yaml");
const world = new Test.World(settings);

// Register the modules
world.registerModule(fruityEcs);
world.registerModule(fruityHierarchy);
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
