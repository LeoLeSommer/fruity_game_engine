type ResourceContainer = any;
type Settings = any;
type World = any;

class CustomService {
  index = 1;

  hello(str: string) {
    console.log("Hello", str, this.index++);
  }
}

export class CustomComponent {
  constructor(args?: Partial<CustomComponent>) {
    Object.assign(this, args);
  }

  value: number = 3;
}

export class CustomComponent2 {
  constructor(args?: Partial<CustomComponent2>) {
    Object.assign(this, args);
  }

  value: number = 30;
}

export default {
  name: "my_platformer",
  dependencies: ["fruity_ecs"],
  setup: (world: World) => {
    console.log("setup", world);
    const resourceContainer = world.getResourceContainer();

    resourceContainer.add("custom_service", new CustomService());

    const customService = resourceContainer.get("custom_service");
    console.log("customService", customService);

    customService.hello("Frame");

    const systemService = resourceContainer.get("system_service");
    const entityService = resourceContainer.get("entity_service");

    systemService.addStartupSystem(
      "test startup 1",
      () => {
        console.log("The engine has been turned on");

        return () => {
          console.log("The engine will be turned off");
        };
      },
      {
        ignorePause: true,
      }
    );

    systemService.addSystem("test 1", () => {
      console.log("A frame is rendered");

      entityService
        .query()
        .with("CustomComponent")
        .with("CustomComponent2")
        .forEach(([customComponent, customComponent2]: any[]) => {
          console.log(
            "Component",
            customComponent.value,
            customComponent2.value
          );
        });
    });
  },
  load_resources: (
    resourceContainer: ResourceContainer,
    settings: Settings
  ) => {
    console.log("loadResources");
  },
};
