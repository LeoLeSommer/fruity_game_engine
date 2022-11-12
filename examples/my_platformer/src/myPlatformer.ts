class CustomService {
  hello(str: string) {
    console.log("Hello", str);
  }
}

export default {
  name: "my_platformer",
  dependencies: ["fruity_ecs"],
  setup: (resourceContainer: ResourceContainer, settings: Settings) => {
    resourceContainer.add("custom_service", new CustomService());

    const systemService = resourceContainer.get("system_service");

    systemService.addStartupSystem("test startup 1", () => {
      console.log("Je commence");

      return () => {
        console.log("Je finis");
      };
    });

    systemService.addSystem("test 1", () => {
      const customService = resourceContainer.get("custom_service");
      customService.hello("Frame");
    });
  },
  loadResources: (
    resourceContainer: ResourceContainer,
    settings: Settings
  ) => {},
  run: (resourceContainer: ResourceContainer, settings: Settings) => {
    const entityService = resourceContainer.get("entity_service");
    entityService.loadScene("./assets/scene.frsc");
  },
};
