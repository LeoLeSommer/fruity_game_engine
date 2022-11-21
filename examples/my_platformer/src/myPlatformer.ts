type ResourceContainer = any;
type Settings = any;

class CustomService {
  hello(str: string) {
    console.log("Hello", str);
  }
}

export default {
  name: "my_platformer",
  dependencies: [
    /*"fruity_ecs"*/
  ],
  setup: (resourceContainer: ResourceContainer, settings: Settings) => {
    console.log("setup");

    /*resourceContainer.add_untyped("custom_service", new CustomService());

    const customService = resourceContainer.get_untyped(
      /*<CustomService>/ "custom_service"
    );
    customService.hello("Frame");*/

    /*const systemService = resourceContainer.get("system_service");

    systemService.addStartupSystem("test startup 1", () => {
      console.log("Je commence");

      return () => {
        console.log("Je finis");
      };
    });

    systemService.addSystem("test 1", () => {
      const customService = resourceContainer.get("custom_service");
      customService.hello("Frame");
    });*/
  },
  load_resources: (
    resourceContainer: ResourceContainer,
    settings: Settings
  ) => {
    console.log("loadResources");
  },
  run: (resourceContainer: ResourceContainer, settings: Settings) => {
    console.log("run");
    /*const entityService = resourceContainer.get("entity_service");
    entityService.loadScene("./assets/scene.frsc");*/
  },
};
