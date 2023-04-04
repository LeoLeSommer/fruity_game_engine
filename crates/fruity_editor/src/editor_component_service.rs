use crate::components::fields::edit_introspect_fields;
use crate::ui::context::UIContext;
use crate::ui::elements::UIElement;
use fruity_ecs::component::component_reference::AnyComponentReference;
use fruity_ecs::component::AnyComponent;
use fruity_ecs::serialization_service::SerializationService;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::introspect::IntrospectFields;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::settings::Settings;
use fruity_game_engine::Arc;
use fruity_game_engine::{export_impl, export_struct, lazy_static, FruityError, FruityResult};
use std::collections::HashMap;
use std::fmt::Debug;

lazy_static! {
    pub static ref DEFAULT_INSPECTOR: Arc<dyn Fn(&mut UIContext, AnyComponentReference) -> FruityResult<UIElement> + Send + Sync> =
        Arc::new(|ctx, component| edit_introspect_fields(ctx, Box::new(component)));
}

#[derive(FruityAny)]
pub struct RegisterComponentParams {
    pub inspector:
        Arc<dyn Fn(&mut UIContext, AnyComponentReference) -> FruityResult<UIElement> + Send + Sync>,
    pub dependencies: Vec<String>,
}

impl Default for RegisterComponentParams {
    fn default() -> Self {
        Self {
            inspector: DEFAULT_INSPECTOR.clone(),
            dependencies: Vec::new(),
        }
    }
}

#[derive(FruityAny)]
#[export_struct]
pub struct EditorComponentService {
    components: HashMap<String, RegisterComponentParams>,
    serialization_service: ResourceReference<SerializationService>,
}

#[export_impl]
impl EditorComponentService {
    pub fn new(resource_container: ResourceContainer) -> Self {
        Self {
            components: HashMap::new(),
            serialization_service: resource_container.require::<SerializationService>(),
        }
    }

    pub fn register_component(
        &mut self,
        component_identifier: &str,
        params: RegisterComponentParams,
    ) {
        self.components
            .insert(component_identifier.to_string(), params);
    }

    pub fn inspect(
        &self,
        ctx: &mut UIContext,
        component: AnyComponentReference,
    ) -> FruityResult<UIElement> {
        let component_identifier = component.get_class_name().unwrap();

        match self.components.get(&component_identifier) {
            Some(params) => (params.inspector)(ctx, component),
            None => edit_introspect_fields(ctx, Box::new(component)),
        }
    }

    pub fn instantiate(
        &self,
        component_identifier: &str,
    ) -> FruityResult<Option<Vec<AnyComponent>>> {
        let serialization_service = self.serialization_service.read();
        let component_params =
            if let Some(component_params) = self.components.get(component_identifier) {
                component_params
            } else {
                return Ok(None);
            };

        let instance = serialization_service.instantiate(
            &Settings::Object(HashMap::new()),
            component_identifier.to_string(),
            &HashMap::new(),
        )?;
        let instance = if let Some(ScriptValue::Object(instance)) = instance {
            instance
        } else {
            return Ok(None);
        };
        let instance = instance
            .as_any_box()
            .downcast::<AnyComponent>()
            .or_else(|_| {
                Err(FruityError::GenericFailure(format!(
                    "A component is expected"
                )))
            })?;

        let mut result = vec![*instance];
        let mut dependencies = component_params
            .dependencies
            .iter()
            .map(|dependency| self.instantiate(dependency))
            .try_collect::<Vec<_>>()?
            .into_iter()
            .filter_map(|dependency| dependency)
            .flatten()
            .collect::<Vec<_>>();
        result.append(&mut dependencies);

        Ok(Some(result))
    }

    pub fn search(&self, search: &str) -> impl Iterator<Item = String> + '_ {
        let search = search.to_string();
        self.components
            .keys()
            .filter(move |key| key.to_lowercase().contains(&search.to_lowercase()))
            .map(|key| key.clone())
    }
}

impl Debug for EditorComponentService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
