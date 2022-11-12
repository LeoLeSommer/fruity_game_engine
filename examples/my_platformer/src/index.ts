import myPlatformer from "./myPlatformer";

console.log("Hello world!");

const settings = readSettings("examples/test/settings.yaml");
const world = new World(settings);

world.registerModule(myPlatformer);
world.registerModule(fruityEcs);

world.setupModules();
world.loadResources();

world.run();
